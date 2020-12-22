use std::collections::HashMap;

use reqwest::Client;
use lazy_static::lazy_static;

use spider_rs_common::result::Result;
use spider_rs_common::task::TaskSettings;
use spider_rs_common::util;

lazy_static! {
    static ref RunningTasks: HashMap<String, RunningTask> = HashMap::with_capacity(100);
}

pub(crate) struct RunningTask {
    client: Client,
    settings: TaskSettings,
}

fn new_task(mut task_settings: TaskSettings) -> TaskSettings {
    if task_settings.id.is_empty() {
        task_settings.id = util::simple_uuid();
    }
    task_settings
}
