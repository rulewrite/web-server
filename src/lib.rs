use std::{error::Error, fmt, thread};

#[derive(Debug)]
pub struct PoolCreationError;

impl Error for PoolCreationError {
    fn description(&self) -> &str {
        "스레드 풀의 size는 0일 수 없다."
    }
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PoolCreationError: 스레드 풀의 개수는 0일 수 없다.")
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        Worker {
            id,
            thread: thread::spawn(|| {}),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// 새 스레드풀을 생성
    ///
    /// size는 풀의 스레드 수
    ///
    /// # Panics
    ///
    /// size가 0이면 `new` 함수는 패닉을 일으킨다.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            return Ok(ThreadPool::new(size));
        }

        Err(PoolCreationError)
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
