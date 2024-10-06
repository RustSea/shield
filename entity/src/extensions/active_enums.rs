use crate::sea_orm_active_enums::ApiUserAccess;
use std::cmp::Ordering;

pub mod access_level {
    pub const READ: u32 = 10;
    pub const WRITE: u32 = 20;
    pub const DELETE: u32 = 30;
    pub const ADMIN: u32 = 100;
}

impl ApiUserAccess {
    fn to_level(&self) -> u32 {
        match self {
            ApiUserAccess::Read => access_level::READ,
            ApiUserAccess::Write => access_level::WRITE,
            ApiUserAccess::Delete => access_level::DELETE,
            ApiUserAccess::Admin => access_level::ADMIN,
        }
    }

    pub fn has_access(&self, required: ApiUserAccess) -> bool {
        self.to_level() >= required.to_level()
    }
}

impl PartialOrd for ApiUserAccess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.to_level().cmp(&other.to_level()))
    }
}

impl Ord for ApiUserAccess {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_level().cmp(&other.to_level())
    }
}