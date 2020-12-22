use tokio::runtime::{Builder, Runtime};

pub fn init_runtime() -> Runtime {
    let r = Builder::new()
        .threaded_scheduler()
        .enable_all()
        .core_threads(num_cpus::get() + 1)
        .thread_name("songday-spider-rs")
        .thread_stack_size(64 * 1024 * 1024)
        .build();
    match r {
        Ok(rt) => rt,
        Err(e) => {
            panic!("Create runtime failed: {:?}", e);
        }
    }
}
