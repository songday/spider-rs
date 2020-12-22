use reqwest::Client;
use reqwest::ClientBuilder;

use spider_rs_common::result::{Error, Result};
use spider_rs_common::task::TaskSettings;

pub fn create_client(task_settings: TaskSettings) -> Result<Client> {
    let builder = ClientBuilder::new()
        .connect_timeout(task_settings.connection_timeout)
        .timeout(task_settings.connection_timeout + task_settings.read_timeout);
    match builder.build() {
        Ok(c) => Ok(c),
        Err(e) => {
            println!("{:?}", e);
            Err(Error::CreateClientFailed)
        }
    }
}

async fn get(client: Client, url: &str) -> Result<()> {
    Ok(())
}
