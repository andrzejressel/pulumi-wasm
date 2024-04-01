use crate::bindings::component::pulumi_wasm::external_world::is_in_preview;
use crate::bindings::exports::component::pulumi_wasm::function_reverse_callback::{
    FunctionInvocationRequest, FunctionInvocationResult,
};
use crate::bindings::exports::component::pulumi_wasm::output_interface::GuestOutput;
use crate::bindings::exports::component::pulumi_wasm::output_interface::Output as WasmOutput;
use crate::bindings::exports::component::pulumi_wasm::register_interface::RegisterResourceRequest;
use crate::bindings::exports::component::pulumi_wasm::stack_interface::OutputBorrow;
use crate::bindings::exports::component::pulumi_wasm::{
    function_reverse_callback, output_interface, register_interface, stack_interface,
};
use bindings::component::pulumi_wasm::external_world;
use core::fmt::Debug;
use log::{error, info};
use prost::Message;
use prost_types::value::Kind;
use prost_types::Struct;
use rmpv::{Utf8String, Value};
use std::collections::BTreeMap;
use std::fmt::Formatter;
use std::ops::Deref;

use crate::output::{access_map, output_map, FunctionId, FunctionSource, OutputContent};
bindings::export!(Component with_types_in bindings);

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
mod bindings;
mod grpc {
    #![allow(clippy::all)]
    #![allow(clippy::pedantic)]
    tonic::include_proto!("pulumirpc");
}
mod finalizer;
mod output;

struct Component;

impl stack_interface::Guest for Component {
    fn add_export(name: String, value: OutputBorrow<'_>) {
        wasm_common::setup_logger();
        let output = value.get::<Output>().clone();
        info!("Adding export [{name}] with output [{output:?}]");
        output_map().insert(name, output);
    }

    fn finish() -> bool {
        wasm_common::setup_logger();
        finalizer::finish()
    }
}

impl output_interface::Guest for Component {
    type Output = Output;

    fn describe_outputs() -> String {
        wasm_common::setup_logger();
        let outputs = access_map()
            .iter()
            .map(|o| {
                let ref_cell = o.borrow();
                let content = ref_cell.deref();
                content.tpe()
                // o.tpe
            })
            .collect::<Vec<_>>();

        format!("{:?}", outputs)
    }

    fn non_done_exists() -> bool {
        wasm_common::setup_logger();
        for o in access_map() {
            let ref_cell = o.borrow();
            let content = ref_cell.deref();
            match content {
                OutputContent::Done(_) => {}
                OutputContent::Mapped(_, _, _) => return true,
                OutputContent::Func(_, _) => return true,
                OutputContent::Nothing => return true,
            }
        }
        false
    }
}

#[derive(Clone)]
pub struct Output {
    output: output::OutputContentRefCell,
    tags: Vec<String>,
}

impl Debug for Output {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Result::Ok(()) // TODO: Implement Debug for Output
    }
}

impl GuestOutput for Output {
    fn new(value: Vec<u8>) -> Self {
        wasm_common::setup_logger();
        let value = rmpv::decode::read_value(&mut value.as_slice()).unwrap();
        let cell = output::create_new(value);
        Output {
            output: cell,
            tags: vec![],
        }
    }

    fn map(&self, function_name: String) -> WasmOutput {
        wasm_common::setup_logger();
        let function_id = FunctionId::from_string(&function_name);
        let function_source = FunctionSource::from_str("source");
        let output = output::map_external(function_id, function_source, self.output.clone());
        WasmOutput::new(Output {
            output,
            tags: vec![],
        })
    }

    fn get(&self) -> Option<Vec<u8>> {
        wasm_common::setup_logger();
        let ref_cell = self.output.borrow();
        let content = ref_cell.deref();

        match content {
            OutputContent::Done(v) => {
                let mut vec = vec![];
                rmpv::encode::write_value(&mut vec, v).unwrap();
                Some(vec)
            }
            OutputContent::Mapped(_, _, _) | OutputContent::Func(_, _) | OutputContent::Nothing => {
                None
            }
        }
    }

    fn get_field(&self, field: String, required: bool) -> WasmOutput {
        wasm_common::setup_logger();

        info!("Getting field [{field}] from Output [TODO]");

        let o = output::map_internal(vec![self.output.clone()], move |v| {
            let v = v[0].clone();
            info!("Value is [{v}]");

            let v = match v {
                Value::Map(m) => {
                    let key = Value::String(Utf8String::from(field.clone()));
                    let maybe_value = m.iter().find(|(k, _)| k == &key).map(|(_, v)| v.clone()); //.unwrap_or(Value::Nil)
                    match maybe_value {
                        None if is_in_preview() => None,
                        None if required => {
                            error!("Field [{field}] not found in map [{m:?}]");
                            unreachable!("Field [{field}] not found in map [{m:?}]")
                        }
                        None => Some(Value::Nil),
                        Some(v) => Some(v),
                    }
                }
                v => {
                    error!("Value is not a map [{v}]");
                    unreachable!("Value is not a map [{v}]")
                }
            };

            info!("Result is [{v:?}]");

            v
        });

        WasmOutput::new(Output {
            output: o,
            tags: vec![],
        })
    }

    fn get_type(&self) -> String {
        wasm_common::setup_logger();
        let ref_cell = self.output.borrow();
        let content = ref_cell.deref();
        match content {
            OutputContent::Done(_) => "Done",
            OutputContent::Mapped(_, _, _) => "Mapped",
            OutputContent::Func(_, _) => "Func",
            OutputContent::Nothing => "Nothing",
        }
        .to_string()
    }

    fn duplicate(&self) -> WasmOutput {
        wasm_common::setup_logger();
        WasmOutput::new(Output {
            output: self.output.clone(),
            tags: self.tags.clone(),
        })
    }
}

impl function_reverse_callback::Guest for Component {
    fn get_functions(source: String) -> Vec<FunctionInvocationRequest> {
        wasm_common::setup_logger();
        let function_source = &FunctionSource::from_string(&source);
        access_map()
            .iter()
            .flat_map(|f| {
                let f1 = &*f.borrow();

                match f1 {
                    OutputContent::Mapped(id, s, prev) if s == function_source => {
                        match &*prev.borrow() {
                            OutputContent::Done(v) => {
                                info!("Found function [{id:?}] with value [{v}]");
                                let mut vec = vec![];
                                rmpv::encode::write_value(&mut vec, v).unwrap();
                                Some(FunctionInvocationRequest {
                                    id: WasmOutput::new(Output {
                                        output: f.clone(),
                                        tags: vec![],
                                    }),
                                    function_id: id.get(),
                                    value: vec,
                                })
                            }
                            OutputContent::Mapped(_, _, _)
                            | OutputContent::Func(_, _)
                            | OutputContent::Nothing => None,
                        }
                    }
                    OutputContent::Mapped(_, _, _)
                    | OutputContent::Func(_, _)
                    | OutputContent::Done(_)
                    | OutputContent::Nothing => None,
                }
            })
            .collect()
    }

    fn set_functions(results: Vec<FunctionInvocationResult>) {
        wasm_common::setup_logger();
        for x in results {
            let value = rmpv::decode::read_value(&mut x.value.as_slice()).unwrap();
            let borrowed = &x.id.get::<Output>().output;
            borrowed.replace(OutputContent::Done(value));
        }
    }
}

fn protoc_to_messagepack(value: prost_types::Value) -> Value {
    info!("Converting protoc value [{value:?}] to messagepack value");

    let kind = match value.kind {
        None => {
            error!("Kind is none");
            unreachable!("Kind is none")
        }
        Some(ref k) => k,
    };

    let result = match kind {
        // Kind::NullValue(_) => todo!(),
        Kind::NumberValue(n) => Value::F64(*n),
        Kind::StringValue(s) => Value::String(Utf8String::from(s.clone())),
        Kind::BoolValue(b) => Value::Boolean(*b),
        // Kind::StructValue(_) => todo!(),
        // Kind::ListValue(_) => todo!(),
        _ => {
            error!("Cannot convert protoc [{value:?}] to messagepack");
            todo!()
        }
    };

    info!("Result: [{result:?}]");
    result
}

fn protoc_object_to_messagepack_map(o: prost_types::Struct) -> rmpv::Value {
    let fields = o
        .fields
        .iter()
        .map(|(k, v)| {
            let k = Value::String(k.clone().into());
            let v = protoc_to_messagepack(v.clone());
            (k, v)
        })
        .collect::<Vec<_>>();

    Value::Map(fields)
}

fn messagepack_to_protoc(v: &Value) -> prost_types::Value {
    info!("Converting value [{v}] to protoc value");
    let result = match v {
        Value::Nil => prost_types::Value {
            kind: Option::from(prost_types::value::Kind::NullValue(0)),
        },
        Value::Integer(i) => prost_types::Value {
            kind: Option::from(prost_types::value::Kind::NumberValue(i.as_f64().unwrap())),
        },
        Value::String(s) => prost_types::Value {
            kind: Option::from(prost_types::value::Kind::StringValue(
                s.clone().into_str().unwrap(),
            )),
        },
        _ => {
            error!("Cannot convert messagepack [{v}] to protoc");
            todo!()
        }
    };
    info!("Result: [{result:?}]");
    result
}

impl register_interface::Guest for Component {
    fn register(request: RegisterResourceRequest) -> WasmOutput {
        wasm_common::setup_logger();

        let values = request
            .object
            .iter()
            .map(|o| o.value.get::<Output>().output.clone())
            .collect::<Vec<_>>();
        let names = request
            .object
            .iter()
            .map(|o| o.name.clone())
            .collect::<Vec<_>>();

        let new_output = output::map_internal(values, move |v| {
            info!("Converting values [{v:?}] with names [{names:?}]");

            let object = Self::create_protobuf_struct(&names, &v);

            info!("Resulting object: [{object:?}]");

            let request = grpc::RegisterResourceRequest {
                r#type: request.type_.clone(),
                name: request.name.clone(),
                parent: "".to_string(),
                custom: true,
                object: Some(object),
                protect: false,
                dependencies: vec![],
                provider: "".to_string(),
                property_dependencies: Default::default(),
                // property_dependencies: HashMap::from(
                //     [("value".to_string(), register_resource_request::PropertyDependencies { urns: vec!["test".to_string()] })]
                // ),
                delete_before_replace: false,
                version: "".to_string(),
                ignore_changes: vec![],
                accept_secrets: true,
                additional_secret_outputs: vec![],
                alias_ur_ns: vec![],
                import_id: "".to_string(),
                custom_timeouts: None,
                delete_before_replace_defined: false,
                supports_partial_values: false,
                remote: false,
                accept_resources: false,
                providers: Default::default(),
                replace_on_changes: vec![],
                plugin_download_url: "".to_string(),
                plugin_checksums: Default::default(),
                retain_on_delete: false,
                aliases: vec![],
                deleted_with: "".to_string(),
                alias_specs: false,
                source_position: None,
            };

            let vec_request = request.encode_to_vec();

            let result_vec = external_world::register_resource(vec_request.as_slice());

            let response =
                grpc::RegisterResourceResponse::decode(&mut result_vec.as_slice()).unwrap();

            info!("Response: [{response:?}]");

            let object = response.object.unwrap_or(Struct::default());

            info!("Converting protobuf struct {object:?}");

            let result = protoc_object_to_messagepack_map(object);

            info!("Message pack map: [{result:?}]");

            Some(result)
        });

        WasmOutput::new(Output {
            output: new_output,
            tags: vec![],
        })
    }
}

impl Component {
    pub fn create_protobuf_struct(names: &[String], v: &[Value]) -> Struct {
        let pairs = names
            .iter()
            .zip(v.iter())
            .map(|(name, value)| {
                let v = messagepack_to_protoc(value);
                (name.clone(), v)
            })
            .collect::<Vec<_>>();

        Struct {
            fields: BTreeMap::from_iter(pairs),
        }
    }
}
