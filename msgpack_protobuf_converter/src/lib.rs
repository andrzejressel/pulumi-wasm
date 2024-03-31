use prost_types::Struct as ProtobufStruct;
use prost_types::Value as ProtobufValue;
use rmpv::{Value as MsgpackValue, Value};
use anyhow::{anyhow, Context, Result};
use log::{error, info};

enum Type {
    Int,
    Float,
    String,
    Array(Box<Type>),
    Map(Box<Vec<(String, Type)>>),
}

fn msgpack_to_protobuf(v: &MsgpackValue) -> Result<ProtobufValue> {
    info!("Converting value [{v}] to protoc value");
    let result = match v {
        MsgpackValue::Nil => prost_types::Value {
            kind: Option::from(prost_types::value::Kind::NullValue(0)),
        },
        MsgpackValue::Integer(i) => prost_types::Value {
            kind: Option::from(prost_types::value::Kind::NumberValue(i.as_f64().context(format!("Cannot convert integer [{i}] to f64"))?)),
        },
        MsgpackValue::String(s) => prost_types::Value {
            kind: Option::from(prost_types::value::Kind::StringValue(
                s.clone().into_str().unwrap(),
            )),
        },
        MsgpackValue::Boolean(b) => prost_types::Value {
            kind: Option::from(prost_types::value::Kind::BoolValue(*b)),
        },
        MsgpackValue::Array(arr) => {
            let values = arr.iter().enumerate().map(|(i, v)| msgpack_to_protobuf(v).context(format!("Cannot convert element with index [{i}]"))).collect::<Result<_>>()?;
            prost_types::Value {
                kind: Option::from(prost_types::value::Kind::ListValue(
                    prost_types::ListValue {
                        values,
                    },
                )),
            }
        }
        MsgpackValue::Map(map) => {
            let keys = map.iter().map(|(k, _)| {
                match k {
                    MsgpackValue::String(s) => Ok(s.clone().into_str().unwrap()),
                    _ => {
                        error!("Cannot convert map key [{k}] to string");
                        return Err(anyhow!("Cannot convert map key [{k}] to string"));
                    }
                }
            }).collect::<Result<Vec<_>>>()?;
            prost_types::Value {
                kind: Option::from(prost_types::value::Kind::StructValue(
                    prost_types::Struct {
                        fields: map
                            .iter()
                            .zip(keys)
                            .map(|((_, v), k)| {
                                (
                                    k,
                                    msgpack_to_protobuf(v).unwrap(),
                                )
                            })
                            .collect(),
                    },
                )),
            }
        }
        MsgpackValue::Binary(_) => {
            error!("Cannot convert binary messagepack to protoc");
            return Err(anyhow!("Cannot convert binary messagepack to protoc"));
        }
        MsgpackValue::Ext(_, _) => {
            error!("Cannot convert ext messagepack to protoc");
            return Err(anyhow!("Cannot convert ext messagepack to protoc"));
        }
        Value::F32(f) => {
            prost_types::Value {
                kind: Option::from(prost_types::value::Kind::NumberValue(*f as f64)),
            }
        }
        Value::F64(f) => {
            prost_types::Value {
                kind: Option::from(prost_types::value::Kind::NumberValue(*f)),
            }
        }
    };
    info!("Result: [{result:?}]");
    Ok(result)
}

fn protobuf_to_msgpack(message: &ProtobufStruct) -> Result<MsgpackValue> {
    todo!()
}


#[cfg(test)]
mod tests {
    mod msgpack_to_protobuf {
        use std::collections::{BTreeMap, HashMap};
        use rmpv::Value as MsgpackValue;
        use prost_types::Value as ProtobufValue;
        use super::super::msgpack_to_protobuf;

        #[test]
        fn should_convert_nil() {
            let msgpack_value = MsgpackValue::Nil;
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(protobuf_value.kind, Some(prost_types::value::Kind::NullValue(0)));
        }

        #[test]
        fn should_convert_integer() {
            let msgpack_value = MsgpackValue::from(42);
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(protobuf_value.kind, Some(prost_types::value::Kind::NumberValue(42.0)));
        }

        #[test]
        fn should_convert_string() {
            let msgpack_value = MsgpackValue::from("hello");
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(protobuf_value.kind, Some(prost_types::value::Kind::StringValue("hello".to_string())));
        }

        #[test]
        fn should_convert_boolean() {
            let msgpack_value = MsgpackValue::from(true);
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(protobuf_value.kind, Some(prost_types::value::Kind::BoolValue(true)));
        }

        #[test]
        fn should_fail_on_binary() {
            let msgpack_value = MsgpackValue::Binary(vec![0, 1, 2, 3]);
            let error = msgpack_to_protobuf(&msgpack_value).unwrap_err();
            assert!(error.to_string().contains("Cannot convert binary messagepack to protoc"));
        }

        #[test]
        fn should_fail_on_ext() {
            let msgpack_value = MsgpackValue::Ext(42, vec![0, 1, 2, 3]);
            let error = msgpack_to_protobuf(&msgpack_value).unwrap_err();
            assert!(error.to_string().contains("Cannot convert ext messagepack to protoc"));
        }

        #[test]
        fn should_convert_array() {
            let msgpack_value = MsgpackValue::Array(vec![MsgpackValue::from(42), MsgpackValue::from(43)]);
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(protobuf_value.kind, Some(prost_types::value::Kind::ListValue(prost_types::ListValue {
                values: vec![
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::NumberValue(42.0))
                    },
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::NumberValue(43.0))
                    },
                ]
            })));
        }

        #[test]
        fn should_convert_map() {
            let msgpack_value = MsgpackValue::Map(vec![
                (MsgpackValue::from("key1"), MsgpackValue::from(42)),
                (MsgpackValue::from("key2"), MsgpackValue::from(43)),
            ]);
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(protobuf_value.kind, Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                fields: BTreeMap::from([
                    ("key1".to_string(), prost_types::Value {
                        kind: Some(prost_types::value::Kind::NumberValue(42.0))
                    }),
                    ("key2".to_string(), prost_types::Value {
                        kind: Some(prost_types::value::Kind::NumberValue(43.0))
                    })
                ])
            })));
        }

        #[test]
        fn should_convert_f32() {
            let msgpack_value = MsgpackValue::F32(42.0);
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(protobuf_value.kind, Some(prost_types::value::Kind::NumberValue(42.0)));
        }

        #[test]
        fn should_convert_f64() {
            let msgpack_value = MsgpackValue::F64(42.0);
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(protobuf_value.kind, Some(prost_types::value::Kind::NumberValue(42.0)));
        }

        #[test]
        fn should_fail_when_nonstring_map_key() {
            let msgpack_value = MsgpackValue::Map(vec![
                (MsgpackValue::from(42), MsgpackValue::from(42)),
                (MsgpackValue::from("key2"), MsgpackValue::from(43)),
            ]);
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap_err();
            assert_eq!(protobuf_value.to_string(), "Cannot convert map key [42] to string");
        }

        #[test]
        fn should_return_nested_array_error() {
            let msgpack_value = MsgpackValue::Array(vec![MsgpackValue::from(42), MsgpackValue::Binary(vec![])]);
            let err = msgpack_to_protobuf(&msgpack_value).unwrap_err();
            let chain: Vec<_> = anyhow::Chain::new(err.as_ref())
                .map(|e| e.to_string())
                .collect();

            assert_eq!(
                vec![
                    "Cannot convert element with index [1]",
                    "Cannot convert binary messagepack to protoc",
                ],
                chain
            );
        }
    }
}