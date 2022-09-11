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

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
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

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // 스레드를 생성하고 벡터에 저장
        }

        ThreadPool { threads }
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
