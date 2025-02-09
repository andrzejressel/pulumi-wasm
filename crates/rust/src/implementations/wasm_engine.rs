use crate::implementations::engine::{GestaltEngine, GestaltOutput};
use anyhow::Error;
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::output_interface;
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::pulumi_engine::Engine as WitEngine;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::RwLock;
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;
use crate::output::HASHMAP;

type Function = Box<dyn Fn(&String) -> Result<String, Error> + Send>;

pub(crate) struct WasmOutput<T> {
    wasm_output: output_interface::Output,
    engine: Rc<RwLock<InnerWasmEngine>>,
    phantom: PhantomData<T>,
}

impl <T> Clone for WasmOutput<T> {
    fn clone(&self) -> Self {
        WasmOutput {
            engine: self.engine.clone(),
            wasm_output: todo!(),
            // underlying_id: self.underlying_id,
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
        let mut map = HASHMAP.lock().unwrap();
        map.insert(uuid.clone(), Box::new(f));

        uuid
        // let new_output = self.wit_engine.map(uuid.as_str());

        // new_output
    }
}

impl <T> GestaltOutput<T> for WasmOutput<T> {
    type Me<A> = WasmOutput<T>;

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
}
