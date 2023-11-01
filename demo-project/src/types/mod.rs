//! Common types

mod articles;
mod profiles;
mod tags;
mod user;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use articles::{
    ArticleCreateUpdateInfo, ArticleCreateUpdateInfoWrapper, ArticleInfo, ArticleInfoWrapper,
    ArticleListInfo,
};

pub use profiles::{ProfileInfo, ProfileInfoWrapper};
pub use tags::TagListInfo;
pub use user::{UserInfo, UserInfoWrapper, UserUpdateInfo, UserUpdateInfoWrapper};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

pub type DeleteWrapper = HashMap<(), ()>;
