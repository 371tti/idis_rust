pub struct WsConnectionSet {
    pub ws_connections: HashMap<Vec<u8>, HashMap<u128, WsConnection>>,
}