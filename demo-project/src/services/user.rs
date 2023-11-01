use super::{request_get, request_put};
use crate::error::Error;
use crate::types::*;

/// Get current user info
pub async fn current() -> Result<UserInfoWrapper, Error> {
    request_get::<UserInfoWrapper>("/user".to_string()).await
}

/// Save info of current user
pub async fn save(user_update_info: UserUpdateInfoWrapper) -> Result<UserInfoWrapper, Error> {
    request_put::<UserUpdateInfoWrapper, UserInfoWrapper>("/user".to_string(), user_update_info)
        .await
}
