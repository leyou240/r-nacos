use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ApiResult<T>
where
    T: Sized + Default,
{
    pub data: Option<T>,
    pub success: bool,
    pub code: Option<String>,
    pub msg: Option<String>,
}

impl<T> ApiResult<T>
where
    T: Sized + Default,
{
    pub fn success(data: Option<T>) -> Self {
        Self {
            data,
            success: true,
            code: None,
            msg: None,
        }
    }

    pub fn error(code: String, msg: Option<String>) -> Self {
        Self {
            data: None,
            success: false,
            code: Some(code),
            msg,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PageResult<T> {
    size: usize,
    list: Vec<T>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct UserSession {
    pub username: String,
    pub nickname: String,
    pub roles: Vec<String>,
    pub extend_infos: HashMap<String, String>,
}