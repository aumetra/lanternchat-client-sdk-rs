use super::*;

bitflags::bitflags! {
    #[derive(Default)]
    pub struct RoleFlags: i16 {
        const HOIST         = 1 << 0;
        const MENTIONABLE   = 1 << 1;
    }
}

common::impl_serde_for_bitflags!(RoleFlags);
common::impl_schema_for_bitflags!(RoleFlags);
common::impl_sql_for_bitflags!(RoleFlags);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
pub struct Role {
    pub id: Snowflake,

    // TODO: Revist removing this
    pub party_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<SmolStr>,
    pub name: SmolStr,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<SmolStr>,
    pub permissions: Permissions,
    pub color: Option<u32>, // can be intentionally null
    pub position: i16,
    pub flags: RoleFlags,
}

impl Role {
    pub fn is_mentionable(&self) -> bool {
        self.flags.contains(RoleFlags::MENTIONABLE)
    }

    pub fn is_admin(&self) -> bool {
        self.permissions.contains(Permissions::ADMINISTRATOR)
    }
}
