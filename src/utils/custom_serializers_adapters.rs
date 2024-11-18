use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};
use num_traits::Num;
use std::fmt;
use std::str::FromStr;

pub struct Hex;

impl<T> SerializeAs<T> for Hex
where
    T: fmt::LowerHex,
{
    fn serialize_as<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{:x}", value);
        serializer.serialize_str(&s)
    }
}

impl<'de, T> DeserializeAs<'de, T> for Hex
where
    T: Num + FromStr<Err = T::FromStrRadixErr>,
    T::FromStrRadixErr: fmt::Display,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        T::from_str_radix(&s, 16).map_err(serde::de::Error::custom)
    }
}

pub struct Base64;

impl SerializeAs<Vec<u8>> for Base64 {
    fn serialize_as<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded = base64::encode(bytes);
        serializer.serialize_str(&encoded)
    }
}

impl<'de> DeserializeAs<'de, Vec<u8>> for Base64 {
    fn deserialize_as<D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        base64::decode(&s).map_err(serde::de::Error::custom)
    }
}

pub struct TimeStamp;

impl SerializeAs<i64> for TimeStamp {
    fn serialize_as<S>(timestamp: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // ミリ秒を秒とナノ秒に分割
        let seconds = *timestamp / 1_000;
        let nanos = (*timestamp % 1_000) * 1_000_000;

        let datetime = NaiveDateTime::from_timestamp_opt(seconds, nanos as u32)
            .ok_or_else(|| serde::ser::Error::custom("Invalid Unix timestamp"))?;
        let datetime: DateTime<Utc> = DateTime::from_utc(datetime, Utc);
        let s = datetime.to_rfc3339();
        serializer.serialize_str(&s)
    }
}

impl<'de> DeserializeAs<'de, i64> for TimeStamp {
    fn deserialize_as<D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let datetime = DateTime::parse_from_rfc3339(&s)
            .map_err(serde::de::Error::custom)?;
        // 秒とナノ秒をミリ秒に変換
        Ok(datetime.timestamp() * 1_000 + (datetime.timestamp_subsec_millis() as i64))
    }
}
