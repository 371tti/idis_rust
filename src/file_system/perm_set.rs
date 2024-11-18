use std::{collections::{btree_set::Intersection, HashSet}, fmt, str};

use serde::{de::{self, MapAccess, Visitor}, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{serde_as, DisplayFromStr};
use crate::utils::custom_serializers_adapters::Hex;

// AccessControl 構造体
#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
pub struct AccessControl {
   #[serde_as(as = "HashSet<Hex>")]
    pub allow: HashSet<u128>,
    #[serde_as(as = "HashSet<Hex>")]
    pub deny: HashSet<u128>,
}

// Perm 構造体
#[derive(Clone, Serialize, Deserialize)]
pub struct Perm {
    pub create: AccessControl,
    pub edit: AccessControl,
    pub read: AccessControl,
    pub delete: AccessControl,
    pub reaction: AccessControl,
    pub share: AccessControl,
}


pub struct AllowAccess {
    pub create: bool,
    pub read: bool,
    pub edit: bool,
    pub delete: bool,
    pub reaction: bool,
    pub share: bool,
}

impl AllowAccess {
    pub fn new(create: bool, read: bool, edit: bool, delete: bool, reaction: bool, share: bool) -> Self {
        Self {
            create,
            read,
            edit,
            delete,
            reaction,
            share,
        }
    }
}


impl Perm {
    pub fn new(create: AccessControl, read: AccessControl, edit: AccessControl, delete: AccessControl, reaction: AccessControl, share: AccessControl) -> Self {
        Self {
            create,
            read,
            edit,
            delete,
            reaction,
            share,
        }
    }

    pub fn allowed_operations(&self, perm: HashSet<u128>) -> AllowAccess {
        AllowAccess::new(
            self.is_allowed(&perm, &self.create.allow, &self.create.deny),
            self.is_allowed(&perm, &self.read.allow, &self.read.deny),
            self.is_allowed(&perm, &self.edit.allow, &self.edit.deny),
            self.is_allowed(&perm, &self.delete.allow, &self.delete.deny),
            self.is_allowed(&perm, &self.reaction.allow, &self.reaction.deny),
            self.is_allowed(&perm, &self.share.allow, &self.share.deny),
        )
    }

    fn is_allowed(&self, perm: &HashSet<u128>, allow_set: &HashSet<u128>, deny_set: &HashSet<u128>) -> bool {
        allow_set.intersection(perm).next().is_some() && !deny_set.intersection(perm).next().is_none()
    }
}
