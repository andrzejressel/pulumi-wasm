use core::fmt::Debug;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use globals::get_pulumi_engine;
use pulumi_wasm_core::{Engine, OutputId};

use crate::bindings::exports::component::pulumi_wasm::output_interface::{GuestOutput, Output};
use crate::bindings::exports::component::pulumi_wasm::register_interface::{
    ObjectField, RegisterResourceRequest, RegisterResourceResult, RegisterResourceResultField,
    ResultField,
};
use crate::bindings::exports::component::pulumi_wasm::stack_interface::{
    FunctionInvocationRequest, FunctionInvocationResult, OutputBorrow,
};
use crate::bindings::exports::component::pulumi_wasm::{
    output_interface, register_interface, stack_interface,
};

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
mod globals;
mod pulumi_connector_impl;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct CustomOutputId(OutputId);

impl From<OutputId> for CustomOutputId {
    fn from(output_id: OutputId) -> Self {
        Self(output_id)
    }
}

struct Component;

impl stack_interface::Guest for Component {
    fn add_export(name: String, value: OutputBorrow<'_>) {
        wasm_common::setup_logger();
        let refcell: &RefCell<Engine> = &get_pulumi_engine();
        refcell
            .borrow_mut()
            .add_output(name.into(), value.get::<CustomOutputId>().0);
    }

    fn finish(functions: Vec<FunctionInvocationResult>) -> Vec<FunctionInvocationRequest> {
        wasm_common::setup_logger();

        let refcell: &RefCell<Engine> = &get_pulumi_engine();

        let v = functions
            .iter()
            .map(|function_invocation_result| {
                let v = serde_json::from_str(function_invocation_result.value.as_str()).unwrap();
                (function_invocation_result.id.get::<CustomOutputId>().0, v)
            })
            .collect();

        let results = refcell.borrow_mut().run(v).unwrap_or_default();

        results
            .into_iter()
            .map(|result| {
                let vec = result.value.to_string();
                let id: CustomOutputId = result.output_id.into();
                FunctionInvocationRequest {
                    id: Output::new(id),
                    function_id: result.function_name.into(),
                    value: vec,
                }
            })
            .collect()

        // vec![]

        // true
        // finalizer::finish()
    }
}

impl output_interface::Guest for Component {
    type Output = CustomOutputId;
}

impl register_interface::Guest for Component {
    fn register(request: RegisterResourceRequest<'_>) -> RegisterResourceResult {
        wasm_common::setup_logger();
        let refcell: &RefCell<Engine> = &get_pulumi_engine();

        let outputs = request
            .results
            .iter()
            .map(|ResultField { name }| name.clone().into())
            .collect::<HashSet<_>>();

        let object = request
            .object
            .iter()
            .map(|ObjectField { name, value }| {
                (name.clone().into(), value.get::<CustomOutputId>().0)
            })
            .collect::<HashMap<_, _>>();

        let (_, field_outputs) = refcell.borrow_mut().create_register_resource_node(
            request.type_.to_string(),
            request.name.to_string(),
            object,
            outputs,
        );

        RegisterResourceResult {
            fields: field_outputs
                .iter()
                .map(|(field_name, output_id)| RegisterResourceResultField {
                    name: field_name.as_string().clone(),
                    output: Output::new(CustomOutputId(*output_id)),
                })
                .collect(),
        }
    }
}

impl GuestOutput for CustomOutputId {
    fn new(value: String) -> CustomOutputId {
        wasm_common::setup_logger();
        let value = serde_json::from_str(&value).unwrap();
        let refcell: &RefCell<Engine> = &get_pulumi_engine();
        let output_id = refcell.borrow_mut().create_done_node(value);
        output_id.into()
    }

    fn map(&self, function_name: String) -> Output {
        wasm_common::setup_logger();
        let refcell: &RefCell<Engine> = &get_pulumi_engine();
        let output_id = refcell
            .borrow_mut()
            .create_native_function_node(function_name.into(), self.0);
        Output::new::<CustomOutputId>(output_id.into())
    }

    fn duplicate(&self) -> Output {
        wasm_common::setup_logger();
        Output::new::<CustomOutputId>(self.0.into())
    }
}
