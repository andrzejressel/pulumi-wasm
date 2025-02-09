use anyhow::Error;
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::pulumi_engine::Engine as WitEngine;
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::stack_interface::add_export;
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::{output_interface, register_interface};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::RwLock;
use uuid::Uuid;
use pulumi_gestalt_rust_adapter::{GestaltCompositeOutput, GestaltEngine, GestaltOutput, RegisterResourceRequest};

type Function = Box<dyn Fn(&String) -> Result<String, Error> + Send>;

pub struct WasmOutput<T> {
    wasm_output: output_interface::Output,
    engine: Rc<RwLock<InnerWasmEngine>>,
    phantom: PhantomData<T>,
}

impl <T> Clone for WasmOutput<T> {
    fn clone(&self) -> Self {
        WasmOutput {
            engine: self.engine.clone(),
            wasm_output: self.wasm_output.clone(),
            phantom: PhantomData,
        }
    }
}

pub(crate) struct InnerWasmEngine {
    wit_engine: WitEngine,
    functions: HashMap<String, Function>,
}

pub(crate) struct WasmEngine {
    engine: Rc<RwLock<InnerWasmEngine>>
}

impl GestaltEngine for WasmEngine {
    type Output<T> = WasmOutput<T>;
    type CompositeOutput = WasmCompositeOutput;
    type OutputId = output_interface::Output;

    fn new<T>(&self, value: String, secret: bool) -> WasmOutput<T> {
        let engine = self.engine.clone();
        let inner_engine = engine.read().unwrap();
        let resource = output_interface::Output::new(&inner_engine.wit_engine, value.as_str(), secret);
        WasmOutput {
            engine: self.engine.clone(),
            wasm_output: resource,
            phantom: PhantomData,
        }
    }

    fn register_resource(&self, request: RegisterResourceRequest<Self::OutputId>) -> Self::CompositeOutput {
        let mut object_fields = Vec::new();
        for object in request.props {
            object_fields.push(register_interface::ObjectField {
                name: object.name.clone(),
                value: &object.value,
            });
        }
        let request = register_interface::RegisterResourceRequest {
            type_: request.type_,
            name: request.name,
            version: request.version,
            object: object_fields
        };

        let engine = self.engine.clone();
        let engine = engine.read().unwrap();

        let result = register_interface::register(&engine.wit_engine, &request);

        WasmCompositeOutput {
            engine: self.engine.clone(),
            wasm_output: result,
        }
    }
}

impl InnerWasmEngine {
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

impl <T> GestaltOutput<T> for WasmOutput<T> {
    type Me<A> = WasmOutput<T>;
    type OutputId = output_interface::Output;

    fn map<B, F>(&self, f: F) -> Self::Me<B>
    where
        F: Fn(T) -> B + Send + 'static,
        T: DeserializeOwned,
        B: Serialize
    {
        let engine = self.engine.clone();
        let mut engine = engine.write().unwrap();

        let function_name = engine.put_function(f);
        let new_output = self.wasm_output.map(function_name.as_str());

        WasmOutput {
            engine: self.engine.clone(),
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
        all_outputs.extend_from_slice(&others);
        let result = output_interface::combine(&all_outputs);
        WasmOutput {
            engine: self.engine.clone(),
            wasm_output: result,
            phantom: PhantomData,
        }
    }

    unsafe fn transmute<F>(self) -> Self::Me<F> {
        Self {
            engine: self.engine.clone(),
            wasm_output: self.wasm_output,
            phantom: PhantomData,
        }
    }

    fn get_id(&self) -> &output_interface::Output {
        &self.wasm_output
    }
}

pub struct WasmCompositeOutput {
    engine: Rc<RwLock<InnerWasmEngine>>,
    wasm_output: output_interface::RegisterOutput,
}

impl GestaltCompositeOutput for WasmCompositeOutput {
    type Output<T> = WasmOutput<T>;
    fn get_field<T>(&self, key: &str) -> Self::Output<T> {
        let output_id = self.wasm_output.extract_field(key);

        WasmOutput {
            engine: self.engine.clone(),
            wasm_output: output_id,
            phantom: PhantomData,
        }
    }
}