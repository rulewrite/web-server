pub struct ThreadPool;

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

        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
