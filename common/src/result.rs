use serde_derive::{Deserialize, Serialize};
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;
// pub type AsyncResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, ThisError, Debug, Deserialize, Serialize)]
pub enum Error {
    #[error("Parsing listening address failed")]
    CreateClientFailed,

    #[error("Parsing listening address failed")]
    HtmlDocParseFailed,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub code: Error,
    pub detail: String,
}

// 如果要在Yew前端展示，这里可以不用手动序列化，让Yew反序列化再展示出来就可以了
// impl Serialize for Error {
//     fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
//         where
//             S: Serializer,
//     {
//         format!("{}", self).serialize(serializer)
//     }
// }

// impl std::fmt::Display for Error {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<String> {
//         unimplemented!()
//     }
// }

// impl std::fmt::Display for ErrResponse {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         unimplemented!()
//     }
// }
