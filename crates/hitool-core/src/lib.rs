//! Core utilities shared by the `HiTool` workspace.
//!
//! The crate intentionally avoids async runtimes, HTTP clients, and database
//! drivers. It provides small, deterministic building blocks with explicit
//! errors and allocation behavior.

#![forbid(unsafe_code)]

mod advanced_codec;
mod builder;
mod clone_support;
mod codec;
mod coll_stream_util;
mod coll_util;
mod collection;
mod collection_adapters;
mod collection_iter;
mod collection_partition;
mod collection_types;
mod compiler;
mod compress;
mod date;
mod error;
mod getter;
mod hutool_codec;
mod id;
mod iter_util;
mod list_util;
mod mutable;
mod radix_codec;
mod stream;
mod string;

pub use advanced_codec::{
    HashIds, MorseCodec, base32_decode, base32_encode, base32_hex_decode, base32_hex_encode,
    base58_decode, base58_decode_checked, base58_decode_checked_auto, base58_encode,
    base58_encode_checked, base62_decode, base62_encode, base62_inverted_decode,
    base62_inverted_encode, bcd_decode, bcd_encode, caesar_decode, caesar_encode,
    idna_decode_domain, idna_encode_domain, punycode_decode, punycode_encode,
    punycode_encode_prefixed, rot_decode, rot_encode,
};
pub use builder::{
    Builder, BuilderError, CompareToBuilder, EqualsBuilder, GenericBuilder, HashCodeBuilder, IdKey,
};
pub use clone_support::{CloneRuntimeException, CloneSupport, Cloneable, DefaultCloneable};
pub use codec::{
    base64_decode, base64_encode, base64_url_decode, base64_url_encode, hex_decode, hex_encode,
    percent_decode, percent_encode_component,
};
pub use coll_stream_util::CollStreamUtil;
pub use coll_util::{BlockingQueue, CollUtil, CollectionKind, CreatedCollection};
pub use collection::{distinct, group_by, partition};
pub use collection_adapters::{
    CollectionUtil, ComputeIter, LineIter, NodeListIter, SpliteratorUtil, TransCollection,
    TransSpliterator,
};
pub use collection_iter::{
    ArrayIter, CopiedIter, EnumerationIter, FilterIter, IterChain, IterableIter,
    IteratorEnumeration, ResettableIter, TransIter,
};
pub use collection_partition::{
    AvgPartition, Partition, PartitionIter, RandomAccessAvgPartition, RandomAccessPartition,
};
pub use collection_types::{
    BoundedPriorityQueue, ConcurrentHashSet, UniqueKeySet, ring_next_for_len, ring_next_index,
    ring_next_u64,
};
pub use compiler::{
    ClassFileManager, ClassFileObject, CompileOutput, CompilerException, DEFAULT_MAX_SOURCE_BYTES,
    RustSourceCompiler, SourceFileObject, SourceFileObjectUtil, diagnostic_messages,
};
pub use compress::{
    DEFAULT_MAX_SIZE_DIFF, Deflate, Gzip, ZipCopyVisitor, ZipEntry, ZipLimits, ZipReader,
    ZipWriter, memory_zip_writer,
};
pub use date::DateUtil;
pub use error::{CoreError, Result};
pub use getter::{
    ArrayTypeGetter, BasicTypeGetter, GroupedTypeGetter, ListTypeGetter, OptArrayTypeGetter,
    OptBasicTypeGetter, OptNullBasicTypeFromObjectGetter, OptNullBasicTypeFromStringGetter,
    OptNullBasicTypeGetter, StringMapGetter,
};
pub use hutool_codec::{
    Base16Codec, Decoder, Encoder, PercentCodec, base64_decode_range_tolerant, base64_decode_text,
    base64_decode_to_file, base64_decode_to_writer, base64_decode_tolerant, base64_encode_config,
    base64_encode_file, base64_encode_reader, base64_encode_text, base64_encode_without_padding,
    encoding_for_label, is_base64, is_base64_code,
};
pub use id::IdUtil;
pub use iter_util::IterUtil;
pub use list_util::ListUtil;
pub use mutable::{
    Mutable, MutableBool, MutableByte, MutableDouble, MutableFloat, MutableInt, MutableLong,
    MutableObj, MutablePair, MutableShort,
};
pub use radix_codec::{
    Base32Decoder, Base32Encoder, Base58Decoder, Base58Encoder, Base62Decoder, Base62Encoder,
    base32_decode_text, base32_decode_to_file, base32_decode_to_writer, base32_encode_file,
    base32_encode_reader, base32_encode_text, base62_decode_text, base62_decode_text_gbk,
    base62_decode_to_file, base62_decode_to_writer, base62_encode_file, base62_encode_reader,
    base62_encode_text, bcd_encode_ascii_prefix,
};
pub use stream::{CollectorCharacteristic, CollectorUtil, SimpleCollector, StreamUtil};
pub use string::{
    StrExt, format_template, is_blank, lower_first, remove_all, remove_chars, split, upper_first,
};

/// Common imports for applications using `hitool-core`.
pub mod prelude {
    pub use crate::{
        DateUtil, IdUtil, Mutable, MutableBool, MutableByte, MutableDouble, MutableFloat,
        MutableInt, MutableLong, MutableObj, MutablePair, MutableShort, StrExt,
    };
}
