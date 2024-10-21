use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod serde_key;
use serde_key::{SerdeKey, SerdeRawKey, SerializableKey};

mod macro_key;
use macro_key::{AttribKey, DeriveKey};

macro_rules! basic_key_benches {
    ($group:ident, $key_struct:ident, $name:literal, $use_func:ident, $change_func:ident) => {
        $group.bench_function(concat!("Create ", $name), |b| b.iter(||
            (black_box($key_struct::new(ID, INDEX)))));
        let key = $key_struct::new(ID, INDEX);
        $group.bench_function(concat!("Use ", $name), |b| b.iter(||
            (black_box($use_func(&key)))));
        let key_actual = $use_func(&key);
        let key_ref = key_actual.as_ref();
        $group.bench_function(concat!("Change ", $name), |b| b.iter(||
            (black_box($change_func(key_ref, NEW_INDEX)))));
    };
}

fn use_serde_key(key: &SerdeKey) -> SerdeRawKey {
    SerializableKey::serialize(key).unwrap()
}

fn change_serde_key(key: &[u8], new: u32) -> Result<SerdeRawKey, bincode::Error> {
    let mut key: SerdeKey = SerializableKey::deserialize(key)?;
    key.index = new;
    SerializableKey::serialize(&key)
}

#[inline]
fn use_attrib_key(key: &AttribKey) -> &AttribKey { key }

fn change_attrib_key(key: &[u8], new: u32) -> AttribKey {
    let mut key = AttribKey::from(key);
    key.set_index(new);
    key
}

#[inline]
fn use_derive_key(key: &DeriveKey) -> &DeriveKey { key }

fn change_derive_key(key: &[u8], new: u32) -> DeriveKey {
    let mut key = DeriveKey::from(key);
    key.set_index(new);
    key
}

fn criterion_benchmark(c: &mut Criterion) {
    {
        const ID: u64 = 0x123456789ABCDEF0;
        const INDEX: u32 = 0xECA86420;
        const NEW_INDEX: u32 = 0xFDB97531;

        let mut group = c.benchmark_group("Database Keys");
        group.sample_size(1000);
        basic_key_benches!{group, SerdeKey, "Serde Key", use_serde_key, change_serde_key}
        basic_key_benches!{group, AttribKey, "Attribute Key", use_attrib_key, change_attrib_key}
        basic_key_benches!{group, DeriveKey, "Derive Key", use_derive_key, change_derive_key}
        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
