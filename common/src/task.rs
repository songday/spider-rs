use core::time::Duration;

use crate::param::ScrapeDomainPolicy;

pub struct TaskSettings {
    // 任务id
    pub id: String,
    // 任务名称
    pub name: String,
    // 起始地址
    pub start_url: String,
    // 连接数
    pub max_connection: u8,
    // 下载间隙
    pub pause_duration: Option<Duration>,
    // 域名下载策略
    pub domain_policy: ScrapeDomainPolicy,
    // 是否检查 robots.txt
    pub check_robots_policies: bool,
    // 连接超时时间
    pub connection_timeout: Duration,
    // 下载超时时间
    pub read_timeout: Duration,
}
