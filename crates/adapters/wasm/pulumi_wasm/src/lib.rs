use pulumi_gestalt_core_core::{Engine, OutputId, PulumiServiceImpl};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::bindings::exports::component::pulumi_wasm::output_interface::{
    GuestOutput, GuestRegisterOutput, Output, RegisterOutput,
};
use crate::bindings::exports::component::pulumi_wasm::pulumi_engine::EngineBorrow;
use crate::bindings::exports::component::pulumi_wasm::register_interface::{
    ObjectField, RegisterResourceRequest, ResourceInvokeRequest,
};
use crate::bindings::exports::component::pulumi_wasm::stack_interface::{
    FunctionInvocationRequest, FunctionInvocationResult, OutputBorrow,
};
use crate::bindings::exports::component::pulumi_wasm::{
    output_interface, pulumi_engine, register_interface, stack_interface,
};

bindings::export!(Component with_types_in bindings);

#[allow(clippy::all)]
#[allow(unused_braces)]
#[allow(static_mut_refs)]
#[rustfmt::skip]
mod bindings;

mod pulumi_connector_impl;

struct CustomOutputId(OutputId, Rc<RefCell<Engine>>);
struct CustomRegisterOutputId(OutputId, Rc<RefCell<Engine>>);

struct LocalPulumiEngine(Rc<RefCell<Engine>>, bool);

impl pulumi_engine::GuestEngine for LocalPulumiEngine {
    fn new(in_preview: bool) -> Self {
        let rc = Rc::new(RefCell::new(Engine::new(PulumiServiceImpl::new(
            pulumi_connector_impl::PulumiConnectorImpl {},
            in_preview,
        ))));
        LocalPulumiEngine(rc, in_preview)
    }
}

struct Component;

impl GuestRegisterOutput for CustomRegisterOutputId {
    fn extract_field(&self, field_name: String) -> Output {
        let refcell: &Rc<RefCell<Engine>> = &self.1.clone();
        let field_name = field_name.into();
        let output_id = refcell
            .borrow_mut()
            .create_extract_field(field_name, self.0);
        Output::new::<CustomOutputId>(CustomOutputId(output_id, refcell.clone()))
    }
}

impl pulumi_engine::Guest for Component {
    type Engine = LocalPulumiEngine;
}

impl stack_interface::Guest for Component {
    fn add_export(name: String, value: OutputBorrow<'_>) {
        pulumi_wasm_common::setup_logger();
        let rc = value.get::<CustomOutputId>().1.clone();
        let refcell: &RefCell<Engine> = &rc;
        refcell
            .borrow_mut()
            .add_output(name.into(), value.get::<CustomOutputId>().0);
    }

    fn finish(
        engine: EngineBorrow<'_>,
        functions: Vec<FunctionInvocationResult>,
    ) -> Vec<FunctionInvocationRequest> {
        pulumi_wasm_common::setup_logger();

        let refcell: &RefCell<Engine> = &engine.get::<LocalPulumiEngine>().0;

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
                let id: CustomOutputId = CustomOutputId(
                    result.output_id,
                    engine.get::<LocalPulumiEngine>().0.clone(),
                );
                FunctionInvocationRequest {
                    id: Output::new(id),
                    function_id: result.function_name.into(),
                    value: vec,
                }
            })
            .collect()
    }
}

impl output_interface::Guest for Component {
    type Output = CustomOutputId;
    type RegisterOutput = CustomRegisterOutputId;

    fn combine(outputs: Vec<OutputBorrow>) -> Output {
        if outputs.is_empty() {
            panic!("combine must have at least one output");
        }
        let refcell = &outputs.first().unwrap().get::<CustomOutputId>().1.clone();
        pulumi_wasm_common::setup_logger();

        let output_id = refcell.borrow_mut().create_combine_outputs(
            outputs
                .into_iter()
                .map(|output| output.get::<CustomOutputId>().0)
                .collect(),
        );
        Output::new::<CustomOutputId>(CustomOutputId(output_id, refcell.clone()))
    }
}

impl register_interface::Guest for Component {
    fn register(engine: EngineBorrow<'_>, request: RegisterResourceRequest<'_>) -> RegisterOutput {
        pulumi_wasm_common::setup_logger();
        let refcell: &RefCell<Engine> = &engine.get::<LocalPulumiEngine>().0;

        let object = request
            .object
            .iter()
            .map(|ObjectField { name, value }| {
                (name.clone().into(), value.get::<CustomOutputId>().0)
            })
            .collect::<HashMap<_, _>>();

        let output_id = refcell.borrow_mut().create_register_resource_node(
            request.type_.to_string(),
            request.name.to_string(),
            object,
            request.version.to_string(),
        );

        RegisterOutput::new(CustomRegisterOutputId(
            output_id,
            engine.get::<LocalPulumiEngine>().0.clone(),
        ))
    }

    fn invoke(engine: EngineBorrow<'_>, request: ResourceInvokeRequest<'_>) -> RegisterOutput {
        pulumi_wasm_common::setup_logger();
        let refcell: &RefCell<Engine> = &engine.get::<LocalPulumiEngine>().0;

        let object = request
            .object
            .iter()
            .map(|ObjectField { name, value }| {
                (name.clone().into(), value.get::<CustomOutputId>().0)
            })
            .collect::<HashMap<_, _>>();

        let output_id = refcell.borrow_mut().create_resource_invoke_node(
            request.token,
            object,
            request.version.to_string(),
        );

        RegisterOutput::new(CustomRegisterOutputId(
            output_id,
            engine.get::<LocalPulumiEngine>().0.clone(),
        ))
    }
}

impl GuestOutput for CustomOutputId {
    fn new(engine: EngineBorrow<'_>, value: String, secret: bool) -> CustomOutputId {
        pulumi_wasm_common::setup_logger();
        let value = serde_json::from_str(&value).unwrap();
        let refcell: &RefCell<Engine> = &engine.get::<LocalPulumiEngine>().0;

        let output_id = refcell.borrow_mut().create_done_node(value, secret);
        CustomOutputId(output_id, engine.get::<LocalPulumiEngine>().0.clone())
    }

    fn map(&self, function_name: String) -> Output {
        pulumi_wasm_common::setup_logger();
        let refcell: &Rc<RefCell<Engine>> = &self.1.clone();

        let output_id = refcell
            .borrow_mut()
            .create_native_function_node(function_name.into(), self.0);
        Output::new::<CustomOutputId>(CustomOutputId(output_id, refcell.clone()))
    }
}
