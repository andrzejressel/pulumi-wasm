use pulumi_gestalt_wit::pulumi_gestalt_bindings::exports::component::pulumi_gestalt::types::RegisterResourceRequest;
use pulumi_gestalt_wit::pulumi_gestalt_bindings::exports::component::pulumi_gestalt::types::ObjectField;
use pulumi_gestalt_wit::pulumi_gestalt_bindings::exports::component::pulumi_gestalt::types::ResourceInvokeRequest;
use pulumi_gestalt_wit::pulumi_gestalt_bindings::exports::component::pulumi_gestalt::context::FunctionInvocationResult;
use pulumi_gestalt_wit::pulumi_gestalt_bindings::exports::component::pulumi_gestalt::types::FunctionInvocationRequest;
use pulumi_gestalt_wit::pulumi_gestalt_bindings::exports::component::pulumi_gestalt::output_interface::OutputBorrow;
use pulumi_gestalt_core::{Engine, OutputId, PulumiServiceImpl};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use pulumi_gestalt_wit::pulumi_gestalt_bindings::exports::component::pulumi_gestalt::output_interface::{
    GuestOutput, GuestCompositeOutput, Output, CompositeOutput,
};
use pulumi_gestalt_wit::pulumi_gestalt_bindings::exports::component::pulumi_gestalt::{
    output_interface, context,
};

mod bindings;
mod pulumi_connector_impl;

struct CustomOutputId(OutputId, Rc<RefCell<Engine>>);
struct CustomCompositeOutputId(OutputId, Rc<RefCell<Engine>>);

struct LocalPulumiContext(Rc<RefCell<Engine>>);

impl context::GuestContext for LocalPulumiContext {
    fn new(in_preview: bool) -> Self {
        let rc = Rc::new(RefCell::new(Engine::new(PulumiServiceImpl::new(
            pulumi_connector_impl::PulumiConnectorImpl {},
            in_preview,
        ))));
        LocalPulumiContext(rc)
    }
    fn create_output(&self, value: String, secret: bool) -> Output {
        pulumi_gestalt_rust_common::setup_logger();
        let value = serde_json::from_str(&value).unwrap();
        let refcell: &RefCell<Engine> = &self.0;

        let output_id = refcell.borrow_mut().create_done_node(value, secret);
        Output::new(CustomOutputId(output_id, self.0.clone()))
    }

    fn register_resource(&self, request: RegisterResourceRequest<'_>) -> CompositeOutput {
        pulumi_gestalt_rust_common::setup_logger();
        let refcell: &RefCell<Engine> = &self.0;

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

        CompositeOutput::new(CustomCompositeOutputId(output_id, self.0.clone()))
    }

    fn invoke_resource(&self, request: ResourceInvokeRequest<'_>) -> CompositeOutput {
        pulumi_gestalt_rust_common::setup_logger();
        let refcell: &RefCell<Engine> = &self.0;

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

        CompositeOutput::new(CustomCompositeOutputId(output_id, self.0.clone()))
    }

    fn finish(&self, functions: Vec<FunctionInvocationResult>) -> Vec<FunctionInvocationRequest> {
        pulumi_gestalt_rust_common::setup_logger();

        let refcell: &RefCell<Engine> = &self.0;

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
                let id: CustomOutputId = CustomOutputId(result.output_id, self.0.clone());
                FunctionInvocationRequest {
                    id: Output::new(id),
                    function_name: result.function_name.into(),
                    value: vec,
                }
            })
            .collect()
    }
}

struct Component;

impl GuestCompositeOutput for CustomCompositeOutputId {
    fn get_field(&self, field_name: String) -> Output {
        let refcell: &Rc<RefCell<Engine>> = &self.1.clone();
        let field_name = field_name.into();
        let output_id = refcell
            .borrow_mut()
            .create_extract_field(field_name, self.0);
        Output::new::<CustomOutputId>(CustomOutputId(output_id, refcell.clone()))
    }
}

impl context::Guest for Component {
    type Context = LocalPulumiContext;
}

impl output_interface::Guest for Component {
    type Output = CustomOutputId;
    type CompositeOutput = CustomCompositeOutputId;
}

impl GuestOutput for CustomOutputId {
    fn map(&self, function_name: String) -> Output {
        pulumi_gestalt_rust_common::setup_logger();
        let refcell: &Rc<RefCell<Engine>> = &self.1.clone();

        let output_id = refcell
            .borrow_mut()
            .create_native_function_node(function_name.into(), self.0);
        Output::new::<CustomOutputId>(CustomOutputId(output_id, refcell.clone()))
    }

    fn combine(&self, outputs: Vec<OutputBorrow>) -> Output {
        pulumi_gestalt_rust_common::setup_logger();
        let mut all_outputs = Vec::with_capacity(outputs.len() + 1);
        all_outputs.push(self.0);
        all_outputs.extend(
            outputs
                .iter()
                .map(|output| output.get::<CustomOutputId>().0),
        );

        let output_id = self.1.borrow_mut().create_combine_outputs(all_outputs);
        Output::new::<CustomOutputId>(CustomOutputId(output_id, self.1.clone()))
    }

    fn add_to_export(&self, name: String) {
        pulumi_gestalt_rust_common::setup_logger();
        self.1.borrow_mut().add_output(name.into(), self.0);
    }

    fn clone(&self) -> Output {
        Output::new::<CustomOutputId>(CustomOutputId(self.0, self.1.clone()))
    }
}
