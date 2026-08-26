Ask two separate questions. Do the operations overlap in lifetime? If yes, they are concurrent. Do
they execute instructions at the same instant on different threads or cores? Only then are they
parallel. A blocked worker is a separate fairness problem: tasks can overlap in lifetime while one is
starved of execution time.
