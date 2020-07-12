#[macro_use]
extern crate serde_derive;

use serde_cbor;
use std::time::SystemTime;

const DATA_SIZE: usize = 16 * 1024 * 1024;

// zerocopyはこういうC likeでないenumでは使えなかった
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum TestEnum {
    BigData {
        // これがあるとすごく速い！
        #[serde(with = "serde_bytes")]
        data: Vec<u8>,
    },
    SmallData {
        data: u8,
    },
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum TestEnum2 {
    BigData { data: Vec<u8> },
    SmallData { data: u8 },
}

fn main() {
    // テストデータ
    println!("with serde_bytes:");
    let mut data = Vec::<u8>::with_capacity(DATA_SIZE);
    unsafe { data.set_len(DATA_SIZE) };

    let val = TestEnum::BigData { data };

    // bincode
    let encoded = measure_time("bincode   (encode)", || {
        bincode::serialize(&val).unwrap()
    });
    let decoded = measure_time("bincode   (decode)", || {
        bincode::deserialize::<TestEnum>(&encoded[..]).unwrap()
    });
    assert_eq!(val, decoded);

    // cbor
    let encoded = measure_time("cbor      (encode)", || {
        serde_cbor::to_vec(&val).unwrap()}
    );
    let decoded = measure_time("cbor      (decode)", || {
        serde_cbor::from_slice(&encoded).unwrap()
    });
    assert_eq!(val, decoded);

    // rmps
    let encoded = measure_time("rmp serde (encode)", || {
        rmp_serde::to_vec(&val).unwrap()
    });
    let decoded = measure_time("rmp serde (decode)", || {
        rmp_serde::from_read_ref(&encoded).unwrap()
    });
    assert_eq!(val, decoded);

    // テストデータ
    println!("");
    println!("without serde_bytes:");
    let mut data = Vec::<u8>::with_capacity(DATA_SIZE);
    unsafe { data.set_len(DATA_SIZE) };

    let val = TestEnum2::BigData { data };

    // bincode
    let encoded = measure_time("bincode   (encode)", || {
        bincode::serialize(&val).unwrap()}
    );
    let decoded = measure_time("bincode   (decode)", || {
        bincode::deserialize::<TestEnum2>(&encoded[..]).unwrap()
    });
    assert_eq!(val, decoded);

    // cbor
    let encoded = measure_time("cbor      (encode)", || {
        serde_cbor::to_vec(&val).unwrap()}
    );
    let decoded = measure_time("cbor      (decode)", || {
        serde_cbor::from_slice(&encoded).unwrap()
    });
    assert_eq!(val, decoded);

    // rmps
    let encoded = measure_time("rmp serde (encode)", || {
        rmp_serde::to_vec(&val).unwrap()}
    );
    let decoded = measure_time("rmp serde (decode)", || {
        rmp_serde::from_read_ref(&encoded).unwrap()
    });
    assert_eq!(val, decoded);
}

/// fの実行にかかる時間を計測する
fn measure_time<F, T>(msg: &str, f: F) -> T
where
    F: Fn() -> T,
{
    let start = SystemTime::now();

    let r = f();

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("{} {:?}", msg, duration);

    r
}
