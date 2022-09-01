window.SIDEBAR_ITEMS = {"enum":[["Error","This is the error type used by Postcard"],["FeedResult","The result of feeding the accumulator."]],"fn":[["from_bytes","Deserialize a message of type `T` from a byte slice. The unused portion (if any) of the byte slice is not returned."],["from_bytes_cobs","Deserialize a message of type `T` from a cobs-encoded byte slice. The unused portion (if any) of the byte slice is not returned."],["serialize_with_flavor","`serialize_with_flavor()` has three generic parameters, `T, F, O`."],["take_from_bytes","Deserialize a message of type `T` from a byte slice. The unused portion (if any) of the byte slice is returned for further usage"],["take_from_bytes_cobs","Deserialize a message of type `T` from a cobs-encoded byte slice. The unused portion (if any) of the byte slice is returned for further usage"],["to_allocvec","Serialize a `T` to an `alloc::vec::Vec<u8>`. Requires the `alloc` feature."],["to_allocvec_cobs","Serialize and COBS encode a `T` to an `alloc::vec::Vec<u8>`. Requires the `alloc` feature."],["to_slice","Serialize a `T` to the given slice, with the resulting slice containing data in a serialized format."],["to_slice_cobs","Serialize a `T` to the given slice, with the resulting slice containing data in a serialized then COBS encoded format. The terminating sentinel `0x00` byte is included in the output buffer."],["to_vec","Serialize a `T` to a `heapless::Vec<u8>`, with the `Vec` containing data in a serialized format. Requires the (default) `heapless` feature."],["to_vec_cobs","Serialize a `T` to a `heapless::Vec<u8>`, with the `Vec` containing data in a serialized then COBS encoded format. The terminating sentinel `0x00` byte is included in the output `Vec`. Requires the (default) `heapless` feature."]],"mod":[["flavors","Flavors - Plugins for `postcard`"]],"struct":[["CobsAccumulator","An accumulator used to collect chunked COBS data and deserialize it."],["Deserializer","A structure for deserializing a postcard message. For now, Deserializer does not implement the same Flavor interface as the serializer does, as messages are typically easier to deserialize in place. This may change in the future for consistency, or to support items that cannot be deserialized in-place, such as compressed message types"],["Serializer","A `serde` compatible serializer, generic over “Flavors” of serializing plugins."]],"type":[["Result","This is the Result type used by Postcard."]]};