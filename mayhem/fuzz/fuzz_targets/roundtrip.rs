#![no_main]
use libfuzzer_sys::fuzz_target;

use bencode::{ByteBuf, bencode_serialize_to_writer, dyn_from_bytes};

// Parse arbitrary bytes into a dynamic BencodeValue; if it parses, re-serialize it, re-parse the
// serialized form, and assert the two values are equal (a parse/serialize fixpoint check).
fuzz_target!(|data: &[u8]| {
    if let Ok(value) = dyn_from_bytes::<ByteBuf>(data) {
        let mut bytes = Vec::new();
        bencode_serialize_to_writer(&value, &mut bytes)
            .expect("deserialized value to serialize");
        let value_2 =
            dyn_from_bytes::<ByteBuf>(&bytes).expect("serialized value to deserialize");
        assert_eq!(value, value_2);
    }
});
