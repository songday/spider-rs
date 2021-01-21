use tokio::runtime::{Builder, Runtime};

pub fn init_runtime() -> Runtime {
    let r = Builder::new_multi_thread()
        .worker_threads(num_cpus::get() + 1)
        .thread_name("songday-spider-rs")
        .thread_stack_size(64 * 1024)
        .enable_all()
        .build();
    match r {
        Ok(rt) => rt,
        Err(e) => {
            panic!("Create runtime failed: {:?}", e);
        }
    }
}
