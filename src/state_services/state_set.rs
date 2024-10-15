
use serde::Serialize;
use serde_json::{json, Value};

use super::reqest_set::Request;
use serde::ser::{Serializer, SerializeStruct};

pub struct State {
    pub user_ruid: String, // user id
    pub user_perm: Vec<String>, // user permission list
    pub session_id: Option<String>,
    pub api_key: Option<String>,
    pub status: u32, // status like http status code
    pub stage: u32, // 0: instance, 1: session, 2: parsing, 3: auth, 4: processing, 5: build
    pub reqest: Request,
}

impl State {
    pub fn new() -> Self {
        Self {
            user_ruid: "".to_string(),
            user_perm: Vec::new(),
            session_id: None,
            api_key: None,
            status: 100,
            stage: 0,
            reqest: Request::new(),
        }
    }

}

impl Serialize for State {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("State", 9)?;
        state.serialize_field("user_ruid", &self.user_ruid)?;
        state.serialize_field("user_perm", &self.user_perm)?;
        state.serialize_field("session_id", &self.session_id)?;
        state.serialize_field("api_key", &self.api_key)?;
        state.serialize_field("status", &self.status)?;
        state.serialize_field("stage", &self.stage)?;
        state.serialize_field("reqest", &self.reqest)?;
        state.serialize_field("version", &"1.0.0")?;
        state.serialize_field("type", &11)?;
        state.end()
    }
}
