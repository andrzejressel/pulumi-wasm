use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

use anyhow::{anyhow, Context, Result};
use log::{error, info};
use prost_types::value::Kind as ProtobufKind;
use prost_types::Value as ProtobufValue;
use rmpv::{Value as MsgpackValue, Value};

#[derive(Debug, PartialEq)]
enum Type {
    Nullable(Box<Type>),
    Bool,
    Int,
    Double,
    String,
    Array(Box<Type>),
    Map(HashMap<String, Type>),
}

fn msgpack_to_protobuf(v: &MsgpackValue) -> Result<ProtobufValue> {
    info!("Converting value [{v}] to protoc value");
    let result = match v {
        MsgpackValue::Nil => prost_types::Value {
            kind: Option::from(ProtobufKind::NullValue(0)),
        },
        MsgpackValue::Integer(i) => prost_types::Value {
            kind: Option::from(ProtobufKind::NumberValue(
                i.as_f64()
                    .context(format!("Cannot convert integer [{i}] to f64"))?,
            )),
        },
        MsgpackValue::String(s) => prost_types::Value {
            kind: Option::from(ProtobufKind::StringValue(s.clone().into_str().unwrap())),
        },
        MsgpackValue::Boolean(b) => prost_types::Value {
            kind: Option::from(ProtobufKind::BoolValue(*b)),
        },
        MsgpackValue::Array(arr) => {
            let values = arr
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    msgpack_to_protobuf(v)
                        .context(format!("Cannot convert element with index [{i}]"))
                })
                .collect::<Result<_>>()?;
            prost_types::Value {
                kind: Option::from(ProtobufKind::ListValue(prost_types::ListValue { values })),
            }
        }
        MsgpackValue::Map(map) => {
            let keys = map
                .iter()
                .map(|(k, _)| match k {
                    MsgpackValue::String(s) => Ok(s.clone().into_str().unwrap()),
                    _ => {
                        error!("Cannot convert map key [{k}] to string");
                        Err(anyhow!("Cannot convert map key [{k}] to string"))
                    }
                })
                .collect::<Result<Vec<_>>>()?;
            prost_types::Value {
                kind: Option::from(ProtobufKind::StructValue(prost_types::Struct {
                    fields: map
                        .iter()
                        .zip(keys)
                        .map(|((_, v), k)| {
                            Ok((
                                k.clone(),
                                msgpack_to_protobuf(v)
                                    .context(format!("Cannot convert value for key [{k}]"))?,
                            ))
                        })
                        .collect::<Result<BTreeMap<_, _>>>()?,
                })),
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
        Value::F32(f) => prost_types::Value {
            kind: Option::from(ProtobufKind::NumberValue(*f as f64)),
        },
        Value::F64(f) => prost_types::Value {
            kind: Option::from(ProtobufKind::NumberValue(*f)),
        },
    };
    info!("Result: [{result:?}]");
    Ok(result)
}

fn protobuf_to_msgpack(message: &ProtobufValue, tpe: &Type) -> Result<MsgpackValue> {
    let kind = &message.kind;

    match (kind, tpe) {
        (None, _) => Err(anyhow!("Protobuf value kind is null")),
        (Some(ProtobufKind::NullValue(_)), Type::Nullable(_)) => Ok(MsgpackValue::Nil),
        (Some(ProtobufKind::NullValue(_)), _) => {
            Err(anyhow!("Invalid type [{tpe:?}] for null value"))
        }
        (Some(_), Type::Nullable(t)) => protobuf_to_msgpack(message, t),
        (Some(ProtobufKind::NumberValue(f)), Type::Double) => Ok(MsgpackValue::F64(*f)),
        (Some(ProtobufKind::NumberValue(f)), Type::Int) => Ok(MsgpackValue::from(*f as i64)),
        (Some(ProtobufKind::NumberValue(_)), tpe) => Err(anyhow!(
            "invalid type [{tpe:?}] for number value [{message:?}]"
        )),
        (Some(ProtobufKind::StringValue(s)), Type::String) => Ok(MsgpackValue::from(s.clone())),
        (Some(ProtobufKind::StringValue(_)), tpe) => Err(anyhow!(
            "invalid type [{tpe:?}] for string value [{message:?}]"
        )),
        (Some(ProtobufKind::BoolValue(b)), Type::Bool) => Ok(MsgpackValue::from(*b)),
        (Some(ProtobufKind::BoolValue(_)), tpe) => Err(anyhow!(
            "invalid type [{tpe:?}] for bool value [{message:?}]"
        )),
        (Some(ProtobufKind::ListValue(list)), Type::Array(t)) => {
            let values = list
                .values
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    protobuf_to_msgpack(v, t)
                        .context(format!("Cannot convert value from index {i}"))
                })
                .collect::<Result<Vec<_>>>()?;
            Ok(MsgpackValue::Array(values))
        }
        (Some(ProtobufKind::ListValue(_)), tpe) => Err(anyhow!(
            "invalid type [{tpe:?}] for list value [{message:?}]"
        )),
        (Some(ProtobufKind::StructValue(protobuf_struct)), Type::Map(type_map)) => {
            let map = combine_maps(type_map, &protobuf_struct.fields);

            let fields = map
                .iter()
                .map(|(k, (v, t))| {
                    Ok((
                        MsgpackValue::from((*k).clone()),
                        protobuf_to_msgpack(t, v)
                            .context(format!("Cannot convert value for key [{k}]"))?,
                    ))
                })
                .collect::<Result<Vec<(_, _)>>>()?;
            Ok(MsgpackValue::Map(fields))
        }
        (Some(ProtobufKind::StructValue(_)), tpe) => Err(anyhow!(
            "invalid type [{tpe:?}] for struct value [{message:?}]"
        )),
    }
}

fn combine_maps<'a, A: Eq + Hash + Ord, B, C>(
    map1: &'a HashMap<A, B>,
    map2: &'a BTreeMap<A, C>,
) -> BTreeMap<&'a A, (&'a B, &'a C)> {
    let mut result = BTreeMap::new();

    for (key, map2_value) in map2 {
        if let Some(map1_value) = map1.get(key) {
            result.insert(key, (map1_value, map2_value));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    mod msgpack_to_protobuf {
        use std::collections::BTreeMap;

        use prost_types::value::Kind as ProtobufKind;
        use rmpv::Value as MsgpackValue;

        use super::super::msgpack_to_protobuf;

        #[test]
        fn should_convert_nil() {
            let msgpack_value = MsgpackValue::Nil;
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(protobuf_value.kind, Some(ProtobufKind::NullValue(0)));
        }

        #[test]
        fn should_convert_integer() {
            let msgpack_value = MsgpackValue::from(42);
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(protobuf_value.kind, Some(ProtobufKind::NumberValue(42.0)));
        }

        #[test]
        fn should_convert_string() {
            let msgpack_value = MsgpackValue::from("hello");
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(
                protobuf_value.kind,
                Some(ProtobufKind::StringValue("hello".to_string()))
            );
        }

        #[test]
        fn should_convert_boolean() {
            let msgpack_value = MsgpackValue::from(true);
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(protobuf_value.kind, Some(ProtobufKind::BoolValue(true)));
        }

        #[test]
        fn should_fail_on_binary() {
            let msgpack_value = MsgpackValue::Binary(vec![0, 1, 2, 3]);
            let error = msgpack_to_protobuf(&msgpack_value).unwrap_err();
            assert!(error
                .to_string()
                .contains("Cannot convert binary messagepack to protoc"));
        }

        #[test]
        fn should_fail_on_ext() {
            let msgpack_value = MsgpackValue::Ext(42, vec![0, 1, 2, 3]);
            let error = msgpack_to_protobuf(&msgpack_value).unwrap_err();
            assert!(error
                .to_string()
                .contains("Cannot convert ext messagepack to protoc"));
        }

        #[test]
        fn should_convert_array() {
            let msgpack_value =
                MsgpackValue::Array(vec![MsgpackValue::from(42), MsgpackValue::from(43)]);
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(
                protobuf_value.kind,
                Some(ProtobufKind::ListValue(prost_types::ListValue {
                    values: vec![
                        prost_types::Value {
                            kind: Some(ProtobufKind::NumberValue(42.0))
                        },
                        prost_types::Value {
                            kind: Some(ProtobufKind::NumberValue(43.0))
                        },
                    ]
                }))
            );
        }

        #[test]
        fn should_convert_map() {
            let msgpack_value = MsgpackValue::Map(vec![
                (MsgpackValue::from("key1"), MsgpackValue::from(42)),
                (MsgpackValue::from("key2"), MsgpackValue::from(43)),
            ]);
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(
                protobuf_value.kind,
                Some(ProtobufKind::StructValue(prost_types::Struct {
                    fields: BTreeMap::from([
                        (
                            "key1".to_string(),
                            prost_types::Value {
                                kind: Some(ProtobufKind::NumberValue(42.0))
                            }
                        ),
                        (
                            "key2".to_string(),
                            prost_types::Value {
                                kind: Some(ProtobufKind::NumberValue(43.0))
                            }
                        )
                    ])
                }))
            );
        }

        #[test]
        fn should_convert_f32() {
            let msgpack_value = MsgpackValue::F32(42.0);
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(protobuf_value.kind, Some(ProtobufKind::NumberValue(42.0)));
        }

        #[test]
        fn should_convert_f64() {
            let msgpack_value = MsgpackValue::F64(42.0);
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap();
            assert_eq!(protobuf_value.kind, Some(ProtobufKind::NumberValue(42.0)));
        }

        #[test]
        fn should_fail_when_nonstring_map_key() {
            let msgpack_value = MsgpackValue::Map(vec![
                (MsgpackValue::from(42), MsgpackValue::from(42)),
                (MsgpackValue::from("key2"), MsgpackValue::from(43)),
            ]);
            let protobuf_value = msgpack_to_protobuf(&msgpack_value).unwrap_err();
            assert_eq!(
                protobuf_value.to_string(),
                "Cannot convert map key [42] to string"
            );
        }

        #[test]
        fn should_return_nested_array_error() {
            let msgpack_value =
                MsgpackValue::Array(vec![MsgpackValue::from(42), MsgpackValue::Binary(vec![])]);
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

        #[test]
        fn should_return_nested_map_error() {
            let msgpack_value = MsgpackValue::Map(vec![
                (MsgpackValue::from("key1"), MsgpackValue::from(42)),
                (MsgpackValue::from("key2"), MsgpackValue::Binary(vec![])),
            ]);
            let err = msgpack_to_protobuf(&msgpack_value).unwrap_err();
            let chain: Vec<_> = anyhow::Chain::new(err.as_ref())
                .map(|e| e.to_string())
                .collect();

            assert_eq!(
                vec![
                    "Cannot convert value for key [key2]",
                    "Cannot convert binary messagepack to protoc",
                ],
                chain
            );
        }
    }

    mod protobuf_to_msgpack {
        use prost_types::value::Kind as ProtobufKind;
        use std::collections::{BTreeMap, HashMap};

        use crate::{protobuf_to_msgpack, Type};

        #[test]
        fn should_convert_nil() {
            let protobuf_value = prost_types::Value {
                kind: Some(ProtobufKind::NullValue(0)),
            };
            let msgpack_value =
                protobuf_to_msgpack(&protobuf_value, &Type::Nullable(Box::from(Type::Int)))
                    .unwrap();
            assert_eq!(msgpack_value, rmpv::Value::Nil);
        }

        #[test]
        fn should_pass_nullable_integer() {
            let protobuf_value = prost_types::Value {
                kind: Some(ProtobufKind::NumberValue(42.0)),
            };
            let msgpack_value =
                protobuf_to_msgpack(&protobuf_value, &Type::Nullable(Box::from(Type::Int)))
                    .unwrap();
            assert_eq!(msgpack_value, rmpv::Value::from(42));
        }

        #[test]
        fn should_convert_integer() {
            let protobuf_value = prost_types::Value {
                kind: Some(ProtobufKind::NumberValue(42.0)),
            };
            let msgpack_value = protobuf_to_msgpack(&protobuf_value, &Type::Int).unwrap();
            assert_eq!(msgpack_value, rmpv::Value::from(42));
        }

        #[test]
        fn should_convert_float() {
            let protobuf_value = prost_types::Value {
                kind: Some(ProtobufKind::NumberValue(42.0)),
            };
            let msgpack_value = protobuf_to_msgpack(&protobuf_value, &Type::Double).unwrap();
            assert_eq!(msgpack_value, rmpv::Value::F64(42.0));
        }

        #[test]
        fn should_convert_string() {
            let protobuf_value = prost_types::Value {
                kind: Some(ProtobufKind::StringValue("hello".to_string())),
            };
            let msgpack_value = protobuf_to_msgpack(&protobuf_value, &Type::String).unwrap();
            assert_eq!(msgpack_value, rmpv::Value::from("hello"));
        }

        #[test]
        fn should_convert_boolean() {
            let protobuf_value = prost_types::Value {
                kind: Some(ProtobufKind::BoolValue(true)),
            };
            let msgpack_value = protobuf_to_msgpack(&protobuf_value, &Type::Bool).unwrap();
            assert_eq!(msgpack_value, rmpv::Value::from(true));
        }

        #[test]
        fn should_convert_array() {
            let protobuf_value = prost_types::Value {
                kind: Some(ProtobufKind::ListValue(prost_types::ListValue {
                    values: vec![
                        prost_types::Value {
                            kind: Some(ProtobufKind::NumberValue(42.0)),
                        },
                        prost_types::Value {
                            kind: Some(ProtobufKind::NumberValue(43.0)),
                        },
                    ],
                })),
            };
            let msgpack_value =
                protobuf_to_msgpack(&protobuf_value, &Type::Array(Box::from(Type::Int))).unwrap();
            assert_eq!(
                msgpack_value,
                rmpv::Value::Array(vec![rmpv::Value::from(42), rmpv::Value::from(43)])
            );
        }

        #[test]
        fn should_return_nested_array_error() {
            let protobuf_value = prost_types::Value {
                kind: Some(ProtobufKind::ListValue(prost_types::ListValue {
                    values: vec![
                        prost_types::Value {
                            kind: Some(ProtobufKind::NumberValue(42.0)),
                        },
                        prost_types::Value {
                            kind: Some(ProtobufKind::NullValue(0)),
                        },
                    ],
                })),
            };
            let err = protobuf_to_msgpack(&protobuf_value, &Type::Array(Box::from(Type::Int)))
                .unwrap_err();
            let chain: Vec<_> = anyhow::Chain::new(err.as_ref())
                .map(|e| e.to_string())
                .collect();

            assert_eq!(
                vec![
                    "Cannot convert value from index 1",
                    "Invalid type [Int] for null value",
                ],
                chain
            );
        }

        #[test]
        fn should_convert_nullable_array() {
            let protobuf_value = prost_types::Value {
                kind: Some(ProtobufKind::ListValue(prost_types::ListValue {
                    values: vec![
                        prost_types::Value {
                            kind: Some(ProtobufKind::NumberValue(42.0)),
                        },
                        prost_types::Value {
                            kind: Some(ProtobufKind::NullValue(0)),
                        },
                    ],
                })),
            };
            let msgpack_value = protobuf_to_msgpack(
                &protobuf_value,
                &Type::Array(Box::from(Type::Nullable(Box::from(Type::Int)))),
            )
            .unwrap();
            assert_eq!(
                msgpack_value,
                rmpv::Value::Array(vec![rmpv::Value::from(42), rmpv::Value::Nil])
            );
        }

        #[test]
        fn should_convert_map() {
            let protobuf_value = prost_types::Value {
                kind: Some(ProtobufKind::StructValue(prost_types::Struct {
                    fields: BTreeMap::from([
                        (
                            "key1".to_string(),
                            prost_types::Value {
                                kind: Some(ProtobufKind::NumberValue(42.0)),
                            },
                        ),
                        (
                            "key2".to_string(),
                            prost_types::Value {
                                kind: Some(ProtobufKind::NumberValue(43.0)),
                            },
                        ),
                    ]),
                })),
            };
            let msgpack_value = protobuf_to_msgpack(
                &protobuf_value,
                &Type::Map(HashMap::from([
                    ("key1".to_string(), Type::Int),
                    ("key2".to_string(), Type::Int),
                ])),
            )
            .unwrap();
            assert_eq!(
                msgpack_value,
                rmpv::Value::Map(vec![
                    (rmpv::Value::from("key1"), rmpv::Value::from(42)),
                    (rmpv::Value::from("key2"), rmpv::Value::from(43)),
                ])
            );
        }

        #[test]
        fn should_return_nested_map_error() {
            let protobuf_value = prost_types::Value {
                kind: Some(ProtobufKind::StructValue(prost_types::Struct {
                    fields: BTreeMap::from([
                        (
                            "key1".to_string(),
                            prost_types::Value {
                                kind: Some(ProtobufKind::NumberValue(42.0)),
                            },
                        ),
                        (
                            "key2".to_string(),
                            prost_types::Value {
                                kind: Some(ProtobufKind::NullValue(0)),
                            },
                        ),
                    ]),
                })),
            };
            let err = protobuf_to_msgpack(
                &protobuf_value,
                &Type::Map(HashMap::from([
                    ("key1".to_string(), Type::Int),
                    ("key2".to_string(), Type::Int),
                ])),
            )
            .unwrap_err();
            let chain: Vec<_> = anyhow::Chain::new(err.as_ref())
                .map(|e| e.to_string())
                .collect();

            assert_eq!(
                vec![
                    "Cannot convert value for key [key2]",
                    "Invalid type [Int] for null value",
                ],
                chain
            );
        }

        #[test]
        fn should_convert_nullable_map() {
            let protobuf_value = prost_types::Value {
                kind: Some(ProtobufKind::StructValue(prost_types::Struct {
                    fields: BTreeMap::from([
                        (
                            "key1".to_string(),
                            prost_types::Value {
                                kind: Some(ProtobufKind::NumberValue(42.0)),
                            },
                        ),
                        (
                            "key2".to_string(),
                            prost_types::Value {
                                kind: Some(ProtobufKind::NullValue(0)),
                            },
                        ),
                    ]),
                })),
            };
            let msgpack_value = protobuf_to_msgpack(
                &protobuf_value,
                &Type::Map(HashMap::from([
                    ("key1".to_string(), Type::Int),
                    ("key2".to_string(), Type::Nullable(Box::from(Type::Int))),
                ])),
            )
            .unwrap();
            assert_eq!(
                msgpack_value,
                rmpv::Value::Map(vec![
                    (rmpv::Value::from("key1"), rmpv::Value::from(42)),
                    (rmpv::Value::from("key2"), rmpv::Value::Nil),
                ])
            );
        }
    }

    #[test]
    fn combine_maps_test() {
        let map1 = HashMap::from([("key1", 42), ("key2", 43)]);
        let map2 = BTreeMap::from([("key2", 44), ("key3", 45)]);

        let result = super::combine_maps(&map1, &map2);

        assert_eq!(result, BTreeMap::from([(&"key2", (&43, &44))]));
    }
}
