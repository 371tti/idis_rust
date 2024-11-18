
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::reqest_set::Request;
use serde::ser::{Serializer, SerializeStruct};
use crate::utils::base64;

use serde_with::{serde_as, DisplayFromStr};
use crate::utils::custom_serializers_adapters::{Hex, Base64};

#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
pub struct State {
    pub user_ruid: String, // user id
    #[serde_as(as = "Vec<Hex>")]
    pub user_perm: Vec<u128>, // user permission list
    #[serde_as(as = "Option<Base64>")]
    pub session_id: Option<Vec<u8>>,
    #[serde_as(as = "Option<Base64>")]
    pub api_key: Option<Vec<u8>>,
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

