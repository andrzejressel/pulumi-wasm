use crate::bindings::component::pulumi_wasm::output_interface;
use anyhow::Error;
use lazy_static::lazy_static;
use log::info;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Mutex;
use uuid::Uuid;

pub struct Output<T> {
    phantom: PhantomData<T>,
    future: output_interface::Output,
}

impl<T: serde::Serialize> From<T> for Output<T> {
    fn from(value: T) -> Output<T> {
        Output::new(&value)
    }
}

impl<T: serde::Serialize> From<T> for Output<Option<T>> {
    fn from(value: T) -> Self {
        Output::new(&Some(value))
    }
}

type Function = Box<dyn Fn(&String) -> Result<String, Error> + Send>;

lazy_static! {
    pub(crate) static ref HASHMAP: Mutex<HashMap<String, Function>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

impl<T> Output<T> {
    ///
    /// # Safety
    ///
    /// Returns handle to inner output representation. Only needed in provider glue code.
    pub unsafe fn get_inner(&self) -> &output_interface::Output {
        &self.future
    }

    ///
    /// # Safety
    ///
    /// Underlying output type must the same as `F` and this Output will take ownership of the handle.
    /// This means that the handle must not be deallocated by something else.
    pub unsafe fn new_from_handle<F: serde::Serialize>(handle: u32) -> Output<F> {
        let output = output_interface::Output::from_handle(handle);
        Output {
            phantom: PhantomData::<F>,
            future: output,
        }
    }

    pub fn map<B, F>(&self, f: F) -> Output<B>
    where
        F: Fn(T) -> B + Send + 'static,
        T: serde::de::DeserializeOwned + Debug,
        B: serde::Serialize + Debug,
    {
        let f = move |arg: &String| {
            let argument = serde_json::from_str(arg)?;
            info!("Argument: {:?}", argument);
            let result = f(argument);
            info!("Result: {:?}", result);
            let result = serde_json::to_string(&result)?;
            Ok(result)
        };

        let uuid = Uuid::now_v7().to_string();
        let mut map = HASHMAP.lock().unwrap();
        map.insert(uuid.clone(), Box::new(f));

        let new_output = self.future.map(uuid.as_str());

        Output {
            phantom: PhantomData,
            future: new_output,
        }
    }

    pub(crate) fn add_to_export(&self, name: &str) {
        crate::bindings::component::pulumi_wasm::stack_interface::add_export(name, &self.future);
    }
}

impl<T: serde::Serialize> Output<T> {
    pub fn new(value: &T) -> Self {
        let binding = serde_json::to_string(&value).unwrap();
        let resource = output_interface::Output::new(binding.as_str());
        Output {
            phantom: PhantomData,
            future: resource,
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/outputs.rs"));
