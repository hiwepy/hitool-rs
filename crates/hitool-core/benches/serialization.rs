#![allow(missing_docs)]

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use hitool_core::{
    MusliDescriptive, MusliPacked, MusliStorage, MusliWire,
    serialize_util::{Decode, Encode},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
struct Message {
    #[musli(Binary, name = 0)]
    id: u64,
    #[musli(Binary, name = 1)]
    name: String,
    #[musli(Binary, name = 2)]
    tags: Vec<String>,
    #[musli(Binary, name = 3)]
    payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[musli(packed)]
struct PackedMessage {
    id: u64,
    name: String,
    tags: Vec<String>,
    payload: Vec<u8>,
}

fn message() -> Message {
    Message {
        id: 42,
        name: "HiTool serialization benchmark".into(),
        tags: (0..16).map(|index| format!("tag-{index}")).collect(),
        payload: (0_u16..4096)
            .map(|value| u8::try_from(value % 251).expect("value is bounded"))
            .collect(),
    }
}

fn packed_message(value: &Message) -> PackedMessage {
    PackedMessage {
        id: value.id,
        name: value.name.clone(),
        tags: value.tags.clone(),
        payload: value.payload.clone(),
    }
}

fn serialization_benchmarks(criterion: &mut Criterion) {
    let value = message();
    let packed = packed_message(&value);
    let bincode_config = bincode::config::standard();

    let json = serde_json::to_vec(&value).expect("fixture serializes");
    let bincode =
        bincode::serde::encode_to_vec(&value, bincode_config).expect("fixture serializes");
    let postcard = postcard::to_stdvec(&value).expect("fixture serializes");
    let wire = MusliWire::encode(&value).expect("fixture serializes");
    let storage = MusliStorage::encode(&value).expect("fixture serializes");
    let packed_bytes = MusliPacked::encode(&packed).expect("fixture serializes");
    let descriptive = MusliDescriptive::encode(&value).expect("fixture serializes");

    let mut encode = criterion.benchmark_group("serialization/encode");
    encode.bench_function("serde_json", |bencher| {
        bencher.iter(|| serde_json::to_vec(black_box(&value)).expect("fixture serializes"));
    });
    encode.bench_function("bincode", |bencher| {
        bencher.iter(|| {
            bincode::serde::encode_to_vec(black_box(&value), bincode_config)
                .expect("fixture serializes")
        });
    });
    encode.bench_function("postcard", |bencher| {
        bencher.iter(|| postcard::to_stdvec(black_box(&value)).expect("fixture serializes"));
    });
    encode.bench_function("musli_wire", |bencher| {
        bencher.iter(|| MusliWire::encode(black_box(&value)).expect("fixture serializes"));
    });
    encode.bench_function("musli_storage", |bencher| {
        bencher.iter(|| MusliStorage::encode(black_box(&value)).expect("fixture serializes"));
    });
    encode.bench_function("musli_packed", |bencher| {
        bencher.iter(|| MusliPacked::encode(black_box(&packed)).expect("fixture serializes"));
    });
    encode.bench_function("musli_descriptive", |bencher| {
        bencher.iter(|| MusliDescriptive::encode(black_box(&value)).expect("fixture serializes"));
    });
    encode.finish();

    let mut decode = criterion.benchmark_group("serialization/decode");
    decode.bench_function("serde_json", |bencher| {
        bencher.iter(|| {
            serde_json::from_slice::<Message>(black_box(&json)).expect("fixture deserializes")
        });
    });
    decode.bench_function("bincode", |bencher| {
        bencher.iter(|| {
            bincode::serde::decode_from_slice::<Message, _>(black_box(&bincode), bincode_config)
                .expect("fixture deserializes")
        });
    });
    decode.bench_function("postcard", |bencher| {
        bencher.iter(|| {
            postcard::from_bytes::<Message>(black_box(&postcard)).expect("fixture deserializes")
        });
    });
    decode.bench_function("musli_wire", |bencher| {
        bencher
            .iter(|| MusliWire::decode::<Message>(black_box(&wire)).expect("fixture deserializes"));
    });
    decode.bench_function("musli_storage", |bencher| {
        bencher.iter(|| {
            MusliStorage::decode::<Message>(black_box(&storage)).expect("fixture deserializes")
        });
    });
    decode.bench_function("musli_packed", |bencher| {
        bencher.iter(|| {
            MusliPacked::decode::<PackedMessage>(black_box(&packed_bytes))
                .expect("fixture deserializes")
        });
    });
    decode.bench_function("musli_descriptive", |bencher| {
        bencher.iter(|| {
            MusliDescriptive::decode::<Message>(black_box(&descriptive))
                .expect("fixture deserializes")
        });
    });
    decode.finish();

    let mut reuse = criterion.benchmark_group("serialization/reuse_buffer");
    reuse.bench_function("musli_wire", |bencher| {
        let mut output = Vec::with_capacity(wire.len());
        bencher.iter(|| {
            MusliWire::encode_into(&mut output, black_box(&value)).expect("fixture serializes");
            black_box(output.len());
        });
    });
    reuse.finish();
}

criterion_group!(benches, serialization_benchmarks);
criterion_main!(benches);
