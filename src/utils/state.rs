use serde::Serialize;
use serde_json::Value;



#[derive(Serialize)]
pub struct State {
    pub user_ruid: u128, // user id
    pub user_perm: Vec<u128>, // user permission list
    pub session_id: Option<Vec<u8>>, // session id
    pub status: u32, // status like http status code
    pub stage: u32, // 0: instance, 1: session, 2: parsing, 3: auth, 4: processing, 5: build
    pub result: Option<Value>,
}

impl State {
    pub fn new() -> Self {
        Self {
            user_ruid: 0,
            user_perm: Vec::new(),
            session_id: None,
            status: 100,
            stage: 0,
            result: None,
        }
    }

}
