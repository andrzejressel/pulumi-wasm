pub mod runner;

use anyhow::{anyhow, Error, Result};
use pulumi_gestalt_rust_adapter::{
    GestaltCompositeOutput, GestaltContext, GestaltOutput, InvokeResourceRequest,
    RegisterResourceRequest,
};
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::pulumi_engine::Engine as WitEngine;
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::stack_interface::{
    add_export, finish, FunctionInvocationRequest, FunctionInvocationResult,
};
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::{
    output_interface, register_interface,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::RwLock;
use uuid::Uuid;

type Function = Box<dyn Fn(&String) -> Result<String, Error> + Send>;

pub struct WasmOutput<T> {
    wasm_output: output_interface::Output,
    context: Rc<RwLock<InnerWasmContext>>,
    phantom: PhantomData<T>,
}

impl<T> Clone for WasmOutput<T> {
    fn clone(&self) -> Self {
        WasmOutput {
            context: self.context.clone(),
            wasm_output: self.wasm_output.clone(),
            phantom: PhantomData,
        }
    }
}

pub(crate) struct InnerWasmContext {
    wit_context: WitEngine,
    functions: HashMap<String, Function>,
}

pub struct WasmContext {
    context: Rc<RwLock<InnerWasmContext>>,
}

impl GestaltContext for WasmContext {
    type Output<T> = WasmOutput<T>;
    type CompositeOutput = WasmCompositeOutput;
    type OutputId = output_interface::Output;

    fn new_output<T: serde::Serialize>(&self, value: &T) -> WasmOutput<T> {
        Self::new_output_priv(self, value, false)
    }

    fn new_secret<T: serde::Serialize>(&self, value: &T) -> WasmOutput<T> {
        Self::new_output_priv(self, value, true)
    }

    fn register_resource(
        &self,
        request: RegisterResourceRequest<Self::OutputId>,
    ) -> Self::CompositeOutput {
        let mut object_fields = Vec::new();
        for object in request.object {
            object_fields.push(register_interface::ObjectField {
                name: object.name.clone(),
                value: object.value,
            });
        }
        let request = register_interface::RegisterResourceRequest {
            type_: request.type_,
            name: request.name,
            version: request.version,
            object: object_fields,
        };

        let context = self.context.clone();
        let context = context.read().unwrap();

        let result = register_interface::register(&context.wit_context, &request);

        WasmCompositeOutput {
            context: self.context.clone(),
            wasm_output: result,
        }
    }

    fn invoke_resource(
        &self,
        request: InvokeResourceRequest<Self::OutputId>,
    ) -> Self::CompositeOutput {
        let mut object_fields = Vec::new();
        for object in request.object {
            object_fields.push(register_interface::ObjectField {
                name: object.name.clone(),
                value: object.value,
            });
        }
        let request = register_interface::ResourceInvokeRequest {
            token: request.token,
            version: request.version,
            object: object_fields,
        };

        let context = self.context.clone();
        let context = context.read().unwrap();

        let result = register_interface::invoke(&context.wit_context, &request);

        WasmCompositeOutput {
            context: self.context.clone(),
            wasm_output: result,
        }
    }
}

impl WasmContext {
    fn new(in_preview: bool) -> WasmContext {
        let wit_context = WitEngine::new(in_preview);
        let context = InnerWasmContext {
            wit_context,
            functions: HashMap::new(),
        };

        WasmContext {
            context: Rc::new(RwLock::new(context)),
        }
    }

    fn new_output_priv<T: serde::Serialize>(&self, value: &T, secret: bool) -> WasmOutput<T> {
        let binding = serde_json::to_string(&value).unwrap();
        let context = self.context.clone();
        let inner_context = context.read().unwrap();
        let resource =
            output_interface::Output::new(&inner_context.wit_context, binding.as_str(), secret);
        WasmOutput {
            context: self.context.clone(),
            wasm_output: resource,
            phantom: PhantomData,
        }
    }

    fn invoke_function(&self, function_id: &str, value: &str) -> Result<String, Error> {
        let context = self.context.clone();
        let context = context.read().unwrap();
        let function = context
            .functions
            .get(function_id)
            .ok_or_else(|| anyhow!("Function with id {function_id} not found"))?;
        let result = function(&value.to_owned())?;
        Ok(result)
    }

    fn invoke_finish(
        &self,
        results: Vec<FunctionInvocationResult>,
    ) -> Result<Vec<FunctionInvocationRequest>> {
        let context = self.context.clone();
        let context = context.read().unwrap();
        let functions = finish(&context.wit_context, &results);
        Ok(functions)
    }
}

impl InnerWasmContext {
    fn put_function<T, B, F>(&mut self, f: F) -> String
    where
        F: Fn(T) -> B + Send + 'static,
        T: DeserializeOwned,
        B: Serialize,
    {
        let f = move |arg: &String| {
            let argument = serde_json::from_str(arg)?;
            let result = f(argument);
            let result = serde_json::to_string(&result)?;
            Ok(result)
        };

        let uuid = Uuid::now_v7().to_string();
        self.functions.insert(uuid.clone(), Box::new(f));

        uuid
    }
}

impl<T> GestaltOutput<T> for WasmOutput<T> {
    type Me<A> = WasmOutput<A>;
    type OutputId = output_interface::Output;

    fn map<B, F>(&self, f: F) -> Self::Me<B>
    where
        F: Fn(T) -> B + Send + 'static,
        T: DeserializeOwned,
        B: Serialize,
    {
        let context = self.context.clone();
        let mut context = context.write().unwrap();

        let function_name = context.put_function(f);
        let new_output = self.wasm_output.map(function_name.as_str());

        WasmOutput {
            context: self.context.clone(),
            wasm_output: new_output,
            phantom: PhantomData,
        }
    }

    fn add_to_export(&self, key: &str) {
        add_export(key, &self.wasm_output);
    }

    fn combine<RESULT>(&self, others: &[&Self::OutputId]) -> Self::Me<RESULT> {
        let mut all_outputs = Vec::new();
        all_outputs.push(&self.wasm_output);
        all_outputs.extend_from_slice(others);
        let result = output_interface::combine(&all_outputs);
        WasmOutput {
            context: self.context.clone(),
            wasm_output: result,
            phantom: PhantomData,
        }
    }

    unsafe fn transmute<F>(self) -> Self::Me<F> {
        WasmOutput {
            context: self.context.clone(),
            wasm_output: self.wasm_output,
            phantom: PhantomData,
        }
    }

    fn get_id(&self) -> &Self::OutputId {
        &self.wasm_output
    }
}

impl<T: Serialize> WasmOutput<T> {
    #[deprecated(note = "Use `Context::new_output` instead")]
    pub fn new(context: &WasmContext, value: &T) -> WasmOutput<T> {
        context.new_output(value)
    }

    #[deprecated(note = "Use `Context::new_secret` instead")]
    pub fn new_secret(context: &WasmContext, value: &T) -> WasmOutput<T> {
        context.new_secret(value)
    }
}

pub struct WasmCompositeOutput {
    context: Rc<RwLock<InnerWasmContext>>,
    wasm_output: output_interface::RegisterOutput,
}

impl GestaltCompositeOutput for WasmCompositeOutput {
    type Output<T> = WasmOutput<T>;
    fn get_field<T>(&self, key: &str) -> Self::Output<T> {
        let output_id = self.wasm_output.extract_field(key);

        WasmOutput {
            context: self.context.clone(),
            wasm_output: output_id,
            phantom: PhantomData,
        }
    }
}
