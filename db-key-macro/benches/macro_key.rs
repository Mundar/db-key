use db_key_macro::{db_key, DBKey};

#[db_key(path = "macro_key")]
pub(crate) struct AttribKey {
    id: u64,
    index: u32,
}

#[derive(DBKey, Debug)]
#[key(path = "macro_key")]
pub(crate) struct Derive {
    pub id: u64,
    pub index: u32,
}
