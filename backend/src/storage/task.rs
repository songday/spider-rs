use spider_rs_common::result::Result;

use crate::task::RunningTask;

async fn append_url(task: RunningTask, url: &str) -> Result<()> {
    //todo check domain policy
    Ok(())
}

async fn pop_url(task_id: u64) -> Result<String> {
    Ok(String::new())
}

async fn running_tasks() {}

async fn all_tasks() {}
