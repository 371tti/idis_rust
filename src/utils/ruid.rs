use rand::RngCore;
use rand_chacha::ChaCha20Rng;
use chrono::Utc;
use std::sync::Mutex;

use crate::sys::init::AppConfig;

pub struct Ruid {
    id: u128, // IDを保持する
}

impl Ruid {
    pub fn to_string(&self) -> String {
        format!("{:032x}", self.id) // u128を16進数文字列に変換
    }

    pub fn to_u128(&self) -> u128 {
        self.id
    }
}

pub struct RuidGenerator {
    prefix: u16,
    device_id: u16,
    rng: Mutex<ChaCha20Rng>, // ChaCha20RngをMutexで包む
}

impl RuidGenerator {
    pub fn new(app_config: &AppConfig) -> Self {
        Self {
            prefix: 0x0000,
            device_id: app_config.server_id,
            rng: Mutex::new(app_config.ruid_rng.clone()), // ChaCha20Rngをエントロピーで初期化
        }
    }

    pub fn generate(&self, prefix: u16, device_id: Option<u16>) -> Ruid {
        let id_builder = IdBuilder::new(prefix, device_id.unwrap_or(self.device_id));
        let id = id_builder.build(&self.rng); // Mutex<ChaCha20Rng>を直接渡す

        Ruid { id }
    }
}

pub struct IdBuilder {
    prefix: u16,
    device_id: u16,
}

impl IdBuilder {
    const PREFIX_SHIFT: u8 = 112;
    const DEVICE_ID_SHIFT: u8 = 92;
    const TIMESTAMP_SHIFT: u8 = 44;

    pub fn new(prefix: u16, device_id: u16) -> Self {
        Self { prefix, device_id }
    }

    pub fn build(self, rng: &Mutex<ChaCha20Rng>) -> u128 {
        let timestamp = (Utc::now().timestamp_millis() & ((1 << 48) - 1)) as u128;

        // RNGを使う直前でロックを取得
        let mut rng = rng.lock().unwrap();
        let rand = (rng.next_u64() & ((1 << 44) - 1)) as u128;

        (self.prefix as u128) << Self::PREFIX_SHIFT
            | (self.device_id as u128) << Self::DEVICE_ID_SHIFT
            | (timestamp << Self::TIMESTAMP_SHIFT)
            | rand
    }
}
