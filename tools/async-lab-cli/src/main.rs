use spinners::{Spinner, Spinners, Stream};
use std::{
	collections::BTreeMap,
	env,
	ffi::OsStr,
	fs,
	io::{self, IsTerminal, Read},
	path::{Path, PathBuf},
	process::{Command, ExitCode, Output, Stdio},
	thread,
	time::Duration,
};

const PROGRESS_DIRECTORY: &str = ".async-lab";
const PROGRESS_FILE: &str = "progress.tsv";
const WATCH_INTERVAL: Duration = Duration::from_millis(200);
const PROCESS_POLL_INTERVAL: Duration = Duration::from_millis(25);

type AppResult<T> = Result<T, String>;
type Progress = BTreeMap<String, String>;

#[derive(Debug, Clone, Copy)]
struct Ui {
	colors: bool,
	spinner: bool,
}

impl Ui {
	fn detect() -> Self {
		let stdout_is_terminal = io::stdout().is_terminal();
		let stderr_is_terminal = io::stderr().is_terminal();
		let terminal_is_suitable = env::var("TERM").map_or(true, |term| term != "dumb");

		Self {
			colors: (stdout_is_terminal || stderr_is_terminal) &&
				terminal_is_suitable &&
				env::var_os("NO_COLOR").is_none(),
			spinner: stderr_is_terminal && terminal_is_suitable,
		}
	}

	fn paint(self, code: &str, text: &str) -> String {
		if self.colors { format!("\x1b[{code}m{text}\x1b[0m") } else { text.to_owned() }
	}

	fn strong(self, text: &str) -> String {
		self.paint("1", text)
	}

	fn dim(self, text: &str) -> String {
		self.paint("2", text)
	}

	fn info(self, text: &str) -> String {
		self.paint("1;36", text)
	}

	fn success(self, text: &str) -> String {
		self.paint("1;32", text)
	}

	fn warning(self, text: &str) -> String {
		self.paint("1;33", text)
	}

	fn error(self, text: &str) -> String {
		self.paint("1;31", text)
	}

	fn accent(self, text: &str) -> String {
		self.paint("1;35", text)
	}
}

#[derive(Debug, Clone)]
struct Exercise {
	key: String,
	source: PathBuf,
	check: PathBuf,
	hint: Option<PathBuf>,
}

#[derive(Debug)]
enum CheckOutcome {
	Passed,
	Failed { stage: &'static str, output: String },
	ChangedDuringCheck,
}

fn main() -> ExitCode {
	let ui = Ui::detect();
	match run(&ui) {
		Ok(success) =>
			if success {
				ExitCode::SUCCESS
			} else {
				ExitCode::FAILURE
			},
		Err(error) => {
			eprintln!("{} {error}", ui.error("error:"));
			ExitCode::FAILURE
		},
	}
}

fn run(ui: &Ui) -> AppResult<bool> {
	let arguments: Vec<String> = env::args().skip(1).collect();
	let command = arguments.first().map(String::as_str).unwrap_or("watch");

	if matches!(command, "help" | "--help" | "-h") {
		print_help(ui);
		return Ok(true);
	}

	let root = find_repository_root()?;
	let exercises = discover_exercises(&root)?;
	if exercises.is_empty() {
		return Err("no Rust exercises were found".to_owned());
	}

	match command {
		"watch" => {
			watch(ui, &root, &exercises)?;
			Ok(true)
		},
		"list" => {
			list(ui, &root, &exercises)?;
			Ok(true)
		},
		"check" => check_command(ui, &root, &exercises, arguments.get(1).map(String::as_str)),
		"hint" => {
			hint_command(ui, &root, &exercises, arguments.get(1).map(String::as_str))?;
			Ok(true)
		},
		unknown => Err(format!("unknown command `{unknown}`; run `cargo run -- --help` for usage")),
	}
}

fn print_help(ui: &Ui) {
	println!(
		"{}\n\nUSAGE:
    cargo run                         Watch the next incomplete exercise
    cargo run -- watch                Same as the default command
    cargo run -- list                 Show every exercise and its status
    cargo run -- check [NAME]         Check one exercise, or the next incomplete one
    cargo run -- hint [NAME]          Show the hint for one exercise

The watcher checks an exercise immediately, then checks it again whenever its source changes.
Passing progress is stored locally in .async-lab/progress.tsv.",
		ui.info("Async Lab exercise runner")
	);
}

fn find_repository_root() -> AppResult<PathBuf> {
	let current =
		env::current_dir().map_err(|error| format!("could not read current directory: {error}"))?;

	for candidate in current.ancestors() {
		if candidate.join("Cargo.toml").is_file() &&
			candidate.join("00-async-mental-model").is_dir()
		{
			return Ok(candidate.to_path_buf());
		}
	}

	Err("run this command from the Async Lab repository or one of its subdirectories".to_owned())
}

fn discover_exercises(root: &Path) -> AppResult<Vec<Exercise>> {
	let mut chapters = read_directories(root)?
		.into_iter()
		.filter(|path| path.file_name().is_some_and(is_chapter_directory))
		.collect::<Vec<_>>();
	chapters.sort();

	let mut exercises = Vec::new();
	for chapter in chapters {
		let exercise_directory = chapter.join("exercises");
		if !exercise_directory.is_dir() {
			continue;
		}

		let mut sources = read_files_with_extension(&exercise_directory, "rs")?;
		sources.sort();

		for source in sources {
			let file_name = source
				.file_name()
				.ok_or_else(|| format!("exercise has no file name: {}", source.display()))?;
			let stem = source.file_stem().and_then(OsStr::to_str).ok_or_else(|| {
				format!("exercise file name is not valid UTF-8: {}", source.display())
			})?;
			let chapter_name = chapter
				.file_name()
				.and_then(OsStr::to_str)
				.ok_or_else(|| format!("chapter name is not valid UTF-8: {}", chapter.display()))?;

			let check = chapter.join("checks").join(file_name);
			let solution = chapter.join("solutions").join(file_name);
			let hint_path = chapter.join("hints").join(format!("{stem}.md"));

			if !check.is_file() {
				return Err(format!(
					"missing checker for `{chapter_name}/{stem}`: {}",
					check.display()
				));
			}
			if !solution.is_file() {
				return Err(format!(
					"missing reference solution for `{chapter_name}/{stem}`: {}",
					solution.display()
				));
			}

			exercises.push(Exercise {
				key: format!("{chapter_name}/{stem}"),
				source,
				check,
				hint: hint_path.is_file().then_some(hint_path),
			});
		}
	}

	Ok(exercises)
}

fn read_directories(path: &Path) -> AppResult<Vec<PathBuf>> {
	read_entries(path, |entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
}

fn read_files_with_extension(path: &Path, extension: &str) -> AppResult<Vec<PathBuf>> {
	read_entries(path, |entry| {
		entry.file_type().is_ok_and(|kind| kind.is_file()) &&
			entry.path().extension() == Some(OsStr::new(extension))
	})
}

fn read_entries(path: &Path, include: impl Fn(&fs::DirEntry) -> bool) -> AppResult<Vec<PathBuf>> {
	let entries = fs::read_dir(path)
		.map_err(|error| format!("could not read {}: {error}", path.display()))?;
	Ok(entries
		.filter_map(Result::ok)
		.filter(include)
		.map(|entry| entry.path())
		.collect())
}

fn is_chapter_directory(name: &OsStr) -> bool {
	let Some(name) = name.to_str() else {
		return false;
	};
	let bytes = name.as_bytes();
	bytes.len() >= 4 && bytes[0].is_ascii_digit() && bytes[1].is_ascii_digit() && bytes[2] == b'-'
}

fn progress_path(root: &Path) -> PathBuf {
	root.join(PROGRESS_DIRECTORY).join(PROGRESS_FILE)
}

fn load_progress(root: &Path) -> AppResult<Progress> {
	match fs::read_to_string(progress_path(root)) {
		Ok(contents) => Ok(parse_progress(&contents)),
		Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Progress::new()),
		Err(error) => Err(format!("could not read local progress: {error}")),
	}
}

fn parse_progress(contents: &str) -> Progress {
	contents
		.lines()
		.filter_map(|line| line.split_once('\t'))
		.map(|(key, fingerprint)| (key.to_owned(), fingerprint.to_owned()))
		.collect()
}

fn save_progress(root: &Path, progress: &Progress) -> AppResult<()> {
	let directory = root.join(PROGRESS_DIRECTORY);
	fs::create_dir_all(&directory)
		.map_err(|error| format!("could not create {}: {error}", directory.display()))?;
	let contents = progress
		.iter()
		.map(|(key, fingerprint)| format!("{key}\t{fingerprint}\n"))
		.collect::<String>();
	fs::write(progress_path(root), contents)
		.map_err(|error| format!("could not save local progress: {error}"))
}

fn source_fingerprint(path: &Path) -> AppResult<String> {
	let bytes =
		fs::read(path).map_err(|error| format!("could not read {}: {error}", path.display()))?;
	Ok(fingerprint_bytes(&bytes))
}

fn completion_fingerprint(exercise: &Exercise) -> AppResult<String> {
	let source = fs::read(&exercise.source)
		.map_err(|error| format!("could not read {}: {error}", exercise.source.display()))?;
	let check = fs::read(&exercise.check)
		.map_err(|error| format!("could not read {}: {error}", exercise.check.display()))?;

	let mut combined = source;
	combined.push(0xff);
	combined.extend(check);
	Ok(fingerprint_bytes(&combined))
}

fn fingerprint_bytes(bytes: &[u8]) -> String {
	let mut hash = 0xcbf2_9ce4_8422_2325_u64;
	for byte in bytes {
		hash ^= u64::from(*byte);
		hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
	}
	format!("{hash:016x}")
}

fn is_complete(exercise: &Exercise, progress: &Progress) -> AppResult<bool> {
	let current = completion_fingerprint(exercise)?;
	Ok(progress.get(&exercise.key) == Some(&current))
}

fn next_incomplete<'a>(
	exercises: &'a [Exercise],
	progress: &Progress,
) -> AppResult<Option<&'a Exercise>> {
	for exercise in exercises {
		if !is_complete(exercise, progress)? {
			return Ok(Some(exercise));
		}
	}
	Ok(None)
}

fn watch(ui: &Ui, root: &Path, exercises: &[Exercise]) -> AppResult<()> {
	let mut progress = load_progress(root)?;

	loop {
		let Some(exercise) = next_incomplete(exercises, &progress)? else {
			println!("\n{}", ui.success("🎉 ALL RUST EXERCISES ARE COMPLETE. NICE WORK!"));
			return Ok(());
		};

		println!("\n{} {}", ui.info("📘 NEXT EXERCISE"), ui.strong(&exercise.key));
		println!("{} {}", ui.dim("Edit:"), ui.dim(&relative_path(root, &exercise.source)));

		loop {
			match attempt(ui, root, exercise, &mut progress)? {
				CheckOutcome::Passed => break,
				CheckOutcome::ChangedDuringCheck => {
					println!(
						"{}",
						ui.warning(
							"🔄 SOURCE CHANGED DURING THE CHECK — TRYING THE SAVED VERSION AGAIN"
						)
					);
				},
				CheckOutcome::Failed { stage, output } => {
					print_failure(ui, root, exercise, stage, &output);
					wait_for_source_change(ui, &exercise.source)?;
					println!(
						"\n{} {}",
						ui.info("🔄 CHANGE DETECTED — CHECKING"),
						ui.strong(&exercise.key)
					);
				},
			}
		}
	}
}

fn wait_for_source_change(ui: &Ui, source: &Path) -> AppResult<()> {
	let initial = source_fingerprint(source)?;
	println!("{} {}", ui.info("👀 WATCHING FOR CHANGES"), ui.dim("Press Ctrl-C to stop."));

	loop {
		thread::sleep(WATCH_INTERVAL);
		match source_fingerprint(source) {
			Ok(current) if current != initial => return Ok(()),
			Ok(_) | Err(_) => {},
		}
	}
}

fn attempt(
	ui: &Ui,
	root: &Path,
	exercise: &Exercise,
	progress: &mut Progress,
) -> AppResult<CheckOutcome> {
	println!("{} {}", ui.info("🧪 CHECKING"), ui.strong(&exercise.key));
	let before = completion_fingerprint(exercise)?;
	let outcome = run_checker(ui, root, exercise)?;
	let after = completion_fingerprint(exercise)?;

	if before != after {
		return Ok(CheckOutcome::ChangedDuringCheck);
	}

	if matches!(outcome, CheckOutcome::Passed) {
		progress.insert(exercise.key.clone(), after);
		save_progress(root, progress)?;
		println!("{} {}", ui.success("✅ PASSED"), ui.strong(&exercise.key));
	}

	Ok(outcome)
}

fn run_checker(ui: &Ui, root: &Path, exercise: &Exercise) -> AppResult<CheckOutcome> {
	let binary_directory = root.join("target").join("async-lab-checks");
	fs::create_dir_all(&binary_directory).map_err(|error| {
		format!("could not create checker output directory {}: {error}", binary_directory.display())
	})?;

	let binary_name = exercise.key.replace(['/', '-'], "_");
	let binary = binary_directory.join(&binary_name);
	let crate_name = format!("async_lab_check_{binary_name}");

	let mut compile_command = Command::new("rustc");
	compile_command
		.current_dir(root)
		.arg("--edition=2024")
		.arg("--test")
		.arg(&exercise.check)
		.arg("--crate-name")
		.arg(crate_name)
		.arg("-o")
		.arg(&binary);
	if ui.colors {
		compile_command.arg("--color=always");
	}

	let compilation =
		run_command_with_spinner(compile_command, ui, &format!("⚙️  Compiling {}", exercise.key))?;

	if !compilation.status.success() {
		return Ok(CheckOutcome::Failed {
			stage: "compilation",
			output: command_output(&compilation),
		});
	}

	let mut test_command = Command::new(&binary);
	test_command.arg("--nocapture");
	if ui.colors {
		test_command.arg("--color=always");
	}
	let tests = run_command_with_spinner(
		test_command,
		ui,
		&format!("🧪 Running tests for {}", exercise.key),
	)?;

	if tests.status.success() {
		Ok(CheckOutcome::Passed)
	} else {
		Ok(CheckOutcome::Failed { stage: "tests", output: command_output(&tests) })
	}
}

fn run_command_with_spinner(mut command: Command, ui: &Ui, message: &str) -> AppResult<Output> {
	let mut child = command
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
		.map_err(|error| format!("could not start `{message}`: {error}"))?;

	let mut stdout = child
		.stdout
		.take()
		.ok_or_else(|| format!("could not capture stdout for `{message}`"))?;
	let mut stderr = child
		.stderr
		.take()
		.ok_or_else(|| format!("could not capture stderr for `{message}`"))?;

	let stdout_reader = thread::spawn(move || {
		let mut bytes = Vec::new();
		stdout.read_to_end(&mut bytes).map(|_| bytes)
	});
	let stderr_reader = thread::spawn(move || {
		let mut bytes = Vec::new();
		stderr.read_to_end(&mut bytes).map(|_| bytes)
	});

	let mut spinner = ui
		.spinner
		.then(|| Spinner::with_stream(Spinners::Aesthetic, ui.info(message), Stream::Stderr));
	let status = loop {
		match child.try_wait() {
			Ok(Some(status)) => break status,
			Ok(None) => {
				thread::sleep(PROCESS_POLL_INTERVAL);
			},
			Err(error) => {
				stop_spinner(&mut spinner);
				return Err(format!("could not wait for `{message}`: {error}"));
			},
		}
	};
	stop_spinner(&mut spinner);

	let stdout = join_command_output(stdout_reader, "stdout", message)?;
	let stderr = join_command_output(stderr_reader, "stderr", message)?;

	Ok(Output { status, stdout, stderr })
}

fn stop_spinner(spinner: &mut Option<Spinner>) {
	if let Some(spinner) = spinner {
		spinner.stop();
	}
}

fn join_command_output(
	reader: thread::JoinHandle<io::Result<Vec<u8>>>,
	stream: &str,
	message: &str,
) -> AppResult<Vec<u8>> {
	reader
		.join()
		.map_err(|_| format!("{stream} reader panicked while running `{message}`"))?
		.map_err(|error| format!("could not read {stream} while running `{message}`: {error}"))
}

fn command_output(output: &Output) -> String {
	let mut combined = String::from_utf8_lossy(&output.stdout).into_owned();
	combined.push_str(&String::from_utf8_lossy(&output.stderr));
	combined
}

fn render_hint(ui: &Ui, hint: &str) -> String {
	let rendered = hint
		.split("**")
		.enumerate()
		.map(
			|(index, segment)| {
				if index % 2 == 1 { ui.accent(segment) } else { render_inline_code(ui, segment) }
			},
		)
		.collect::<String>();

	rendered
		.trim()
		.lines()
		.map(|line| format!("  {line}"))
		.collect::<Vec<_>>()
		.join("\n")
}

fn render_inline_code(ui: &Ui, text: &str) -> String {
	text.split('`')
		.enumerate()
		.map(|(index, segment)| if index % 2 == 1 { ui.info(segment) } else { segment.to_owned() })
		.collect()
}

fn print_failure(ui: &Ui, root: &Path, exercise: &Exercise, stage: &str, output: &str) {
	eprintln!("\n{} {}", ui.error("❌ CHECK FAILED"), ui.strong(&exercise.key),);
	eprintln!("{} {}", ui.dim("Stage:"), ui.warning(&stage.to_uppercase()));
	if !output.trim().is_empty() {
		eprintln!(
			"\n{}",
			ui.dim("── CHECKER OUTPUT ─────────────────────────────────────────────")
		);
		eprintln!("{}", output.trim());
		eprintln!("{}", ui.dim("──────────────────────────────────────────────────────────────"));
	}

	if let Some(hint) = &exercise.hint {
		match fs::read_to_string(hint) {
			Ok(contents) => {
				eprintln!("\n{}\n{}", ui.warning("💡 HINT"), render_hint(ui, &contents));
			},
			Err(error) => eprintln!(
				"\n{} A hint exists but could not be read from {}: {error}",
				ui.error("error:"),
				relative_path(root, hint)
			),
		}
	}
}

fn list(ui: &Ui, root: &Path, exercises: &[Exercise]) -> AppResult<()> {
	let progress = load_progress(root)?;
	for exercise in exercises {
		let status = if is_complete(exercise, &progress)? {
			ui.success("✅ DONE")
		} else {
			ui.warning("○ TODO")
		};
		println!(
			"{status} {} {}",
			ui.strong(&exercise.key),
			ui.dim(&format!("({})", relative_path(root, &exercise.source)))
		);
	}
	Ok(())
}

fn check_command(
	ui: &Ui,
	root: &Path,
	exercises: &[Exercise],
	query: Option<&str>,
) -> AppResult<bool> {
	let mut progress = load_progress(root)?;
	let exercise = match query {
		Some(query) => find_exercise(exercises, query)?,
		None => next_incomplete(exercises, &progress)?
			.ok_or_else(|| "all Rust exercises are already complete".to_owned())?,
	};

	match attempt(ui, root, exercise, &mut progress)? {
		CheckOutcome::Passed => Ok(true),
		CheckOutcome::ChangedDuringCheck => {
			eprintln!(
				"{}",
				ui.warning("The source changed during the check. Run the command again.")
			);
			Ok(false)
		},
		CheckOutcome::Failed { stage, output } => {
			print_failure(ui, root, exercise, stage, &output);
			Ok(false)
		},
	}
}

fn hint_command(
	ui: &Ui,
	root: &Path,
	exercises: &[Exercise],
	query: Option<&str>,
) -> AppResult<()> {
	let progress = load_progress(root)?;
	let exercise = match query {
		Some(query) => find_exercise(exercises, query)?,
		None => next_incomplete(exercises, &progress)?
			.ok_or_else(|| "all Rust exercises are already complete".to_owned())?,
	};

	match &exercise.hint {
		Some(path) => {
			let hint = fs::read_to_string(path)
				.map_err(|error| format!("could not read {}: {error}", path.display()))?;
			println!(
				"{} {}\n{}",
				ui.warning("💡 HINT —"),
				ui.strong(&exercise.key),
				render_hint(ui, &hint)
			);
		},
		None =>
			println!("{} {}.", ui.warning("💡 NO HINT AVAILABLE FOR"), ui.strong(&exercise.key)),
	}
	Ok(())
}

fn find_exercise<'a>(exercises: &'a [Exercise], query: &str) -> AppResult<&'a Exercise> {
	let matches = exercises
		.iter()
		.filter(|exercise| {
			exercise.key == query ||
				exercise
					.source
					.file_stem()
					.and_then(OsStr::to_str)
					.is_some_and(|stem| stem == query)
		})
		.collect::<Vec<_>>();

	match matches.as_slice() {
		[exercise] => Ok(exercise),
		[] => Err(format!("no exercise matches `{query}`")),
		_ => Err(format!("more than one exercise matches `{query}`; use its full key")),
	}
}

fn relative_path(root: &Path, path: &Path) -> String {
	path.strip_prefix(root).unwrap_or(path).display().to_string()
}

#[cfg(test)]
mod tests {
	use super::{Ui, fingerprint_bytes, is_chapter_directory, parse_progress, render_hint};
	use std::ffi::OsStr;

	#[test]
	fn recognizes_numbered_chapter_directories() {
		assert!(is_chapter_directory(OsStr::new("00-async-mental-model")));
		assert!(is_chapter_directory(OsStr::new("16-capstone")));
		assert!(!is_chapter_directory(OsStr::new("tools")));
		assert!(!is_chapter_directory(OsStr::new("0-incomplete")));
	}

	#[test]
	fn fingerprints_depend_on_content() {
		assert_eq!(fingerprint_bytes(b"same"), fingerprint_bytes(b"same"));
		assert_ne!(fingerprint_bytes(b"first"), fingerprint_bytes(b"second"));
	}

	#[test]
	fn parses_valid_progress_lines_and_ignores_invalid_ones() {
		let progress = parse_progress("00/chapter\tabc123\ninvalid\n01/chapter\tdef456\n");
		assert_eq!(progress.get("00/chapter").map(String::as_str), Some("abc123"));
		assert_eq!(progress.get("01/chapter").map(String::as_str), Some("def456"));
		assert_eq!(progress.len(), 2);
	}

	#[test]
	fn styling_can_be_disabled_for_non_terminal_output() {
		let ui = Ui { colors: false, spinner: false };
		assert_eq!(ui.success("passed"), "passed");
		assert_eq!(ui.error("failed"), "failed");
	}

	#[test]
	fn styling_wraps_terminal_text_in_ansi_sequences() {
		let ui = Ui { colors: true, spinner: false };
		assert_eq!(ui.success("passed"), "\x1b[1;32mpassed\x1b[0m");
		assert_eq!(ui.error("failed"), "\x1b[1;31mfailed\x1b[0m");
	}

	#[test]
	fn hint_renderer_removes_markdown_markers_without_colors() {
		let ui = Ui { colors: false, spinner: false };
		assert_eq!(
			render_hint(&ui, "Use **Future** and call `poll`."),
			"  Use Future and call poll."
		);
	}

	#[test]
	fn hint_renderer_styles_emphasis_and_inline_code() {
		let ui = Ui { colors: true, spinner: false };
		let rendered = render_hint(&ui, "Use **Future** and call `poll`.");
		assert!(rendered.contains("\x1b[1;35mFuture\x1b[0m"));
		assert!(rendered.contains("\x1b[1;36mpoll\x1b[0m"));
		assert!(!rendered.contains("**"));
		assert!(!rendered.contains('`'));
	}
}
