mod task;

fn init() {
    let runtime_data = sled::open("./data/runtime-data").expect("open");
}
