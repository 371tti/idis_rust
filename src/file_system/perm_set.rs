use std::{collections::{btree_set::Intersection, HashSet}, fmt, str};

use serde::{de::{self, MapAccess, Visitor}, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

mod hex_u128_set {
    use super::*;
    
    pub fn serialize<S>(set: &HashSet<u128>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex_set: Vec<String> = set.iter().map(|&num| format!("{:x}", num)).collect();
        hex_set.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashSet<u128>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_set: Vec<String> = Vec::deserialize(deserializer)?;
        hex_set
            .into_iter()
            .map(|hex| u128::from_str_radix(&hex, 16).map_err(de::Error::custom))
            .collect()
    }
}

// AccessControl 構造体
#[derive(Clone, Serialize, Deserialize)]
pub struct AccessControl {
    #[serde(with = "hex_u128_set")]
    pub allow: HashSet<u128>,
    #[serde(with = "hex_u128_set")]
    pub deny: HashSet<u128>,
}

// Perm 構造体
#[derive(Clone, Serialize, Deserialize)]
pub struct Perm {
    pub create: AccessControl,
    pub read: AccessControl,
    pub delete: AccessControl,
    pub reaction: AccessControl,
    pub share: AccessControl,
}


pub struct AllowAccess {
    pub create: bool,
    pub read: bool,
    pub delete: bool,
    pub reaction: bool,
    pub share: bool,
}

impl AllowAccess {
    pub fn new(create: bool, read: bool, delete: bool, reaction: bool, share: bool) -> Self {
        Self {
            create,
            read,
            delete,
            reaction,
            share,
        }
    }
}


impl Perm {
    pub fn new(create: AccessControl, read: AccessControl, delete: AccessControl, reaction: AccessControl, share: AccessControl) -> Self {
        Self {
            create,
            read,
            delete,
            reaction,
            share,
        }
    }

    pub fn allowed_operations(&self, perm: HashSet<u128>) -> AllowAccess {
        AllowAccess::new(
            self.is_allowed(&perm, &self.create.allow, &self.create.deny),
            self.is_allowed(&perm, &self.read.allow, &self.read.deny),
            self.is_allowed(&perm, &self.delete.allow, &self.delete.deny),
            self.is_allowed(&perm, &self.reaction.allow, &self.reaction.deny),
            self.is_allowed(&perm, &self.share.allow, &self.share.deny),
        )
    }

    fn is_allowed(&self, perm: &HashSet<u128>, allow_set: &HashSet<u128>, deny_set: &HashSet<u128>) -> bool {
        allow_set.intersection(perm).next().is_some() && !deny_set.intersection(perm).next().is_none()
    }
}
