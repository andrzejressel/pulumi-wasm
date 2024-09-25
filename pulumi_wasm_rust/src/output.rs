use anyhow::Error;
use log::info;
use once_cell::sync::Lazy;
use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface;
use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::stack_interface::add_export;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::sync::Mutex;
use uuid::Uuid;

pub struct Output<T> {
    phantom: PhantomData<T>,
    underlying_id: u32,
}

impl<T> Copy for Output<T> {}

impl<T> Clone for Output<T> {
    fn clone(&self) -> Self {
        *self
    }
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

impl From<&str> for Output<String> {
    fn from(value: &str) -> Self {
        Output::new(&value.to_string())
    }
}

impl From<&str> for Output<Option<String>> {
    fn from(value: &str) -> Self {
        Output::new(&Some(value.to_string()))
    }
}

impl From<Vec<&str>> for Output<Vec<String>> {
    fn from(value: Vec<&str>) -> Self {
        Output::new(&value.into_iter().map(|s| s.to_string()).collect())
    }
}

impl From<Vec<&str>> for Output<Option<Vec<String>>> {
    fn from(value: Vec<&str>) -> Self {
        Output::new(&Some(value.into_iter().map(|s| s.to_string()).collect()))
    }
}

impl<T: serde::Serialize, const N: usize> From<[T; N]> for Output<Vec<T>>
where
    T: serde::Serialize,
{
    fn from(value: [T; N]) -> Self {
        Output::new(&value.into_iter().collect())
    }
}

impl<T: serde::Serialize, const N: usize> From<[T; N]> for Output<Option<Vec<T>>>
where
    T: serde::Serialize,
{
    fn from(value: [T; N]) -> Self {
        Output::new(&Some(value.into_iter().collect()))
    }
}

impl<const N: usize> From<[&str; N]> for Output<Vec<String>> {
    fn from(value: [&str; N]) -> Self {
        Output::new(&value.into_iter().map(|s| s.to_string()).collect())
    }
}

impl<const N: usize> From<[&str; N]> for Output<Option<Vec<String>>> {
    fn from(value: [&str; N]) -> Self {
        Output::new(&Some(value.into_iter().map(|s| s.to_string()).collect()))
    }
}

type Function = Box<dyn Fn(&String) -> Result<String, Error> + Send>;

pub(crate) static HASHMAP: Lazy<Mutex<HashMap<String, Function>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

static NONE_OUTPUT: Lazy<Output<Option<String>>> = Lazy::new(|| {
    let op = None::<String>;
    Output::new(&op)
});

impl<T> Output<T> {
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

        let wit = self.get_inner();
        let new_output = wit.map(uuid.as_str());

        Output {
            phantom: PhantomData,
            underlying_id: output_interface::Output::take_handle(&new_output),
        }
    }

    pub(crate) fn add_to_export(&self, name: &str) {
        add_export(name, &self.get_inner());
    }

    /// Forcefully changes apparent type of underlying Output
    ///
    /// Can be used to workaround Pulumi provider incorrect types
    ///
    /// # Safety
    ///
    /// Underlying output must be of type `F`.
    pub unsafe fn transmute<F: serde::Serialize>(&self) -> Output<F> {
        Output {
            phantom: PhantomData::<F>,
            underlying_id: self.underlying_id,
        }
    }

    #[doc(hidden)]
    ///
    /// # Safety
    ///
    /// Underlying output must be of type `F`.
    pub unsafe fn new_from_handle<F: serde::Serialize>(
        handle: output_interface::Output,
    ) -> Output<F> {
        Output {
            phantom: PhantomData::<F>,
            underlying_id: output_interface::Output::take_handle(&handle),
        }
    }

    #[doc(hidden)]
    pub fn get_inner(&self) -> ManuallyDrop<output_interface::Output> {
        unsafe { ManuallyDrop::new(output_interface::Output::from_handle(self.underlying_id)) }
    }
}

impl<T: serde::Serialize> Output<T> {
    pub fn new(value: &T) -> Self {
        let binding = serde_json::to_string(&value).unwrap();
        let resource = output_interface::Output::new(binding.as_str());
        Output {
            phantom: PhantomData,
            underlying_id: output_interface::Output::take_handle(&resource),
        }
    }

    /// Returns singleton Output containing serialized null
    pub fn empty() -> Output<Option<T>> {
        unsafe { NONE_OUTPUT.transmute::<Option<T>>() }
    }
}

include!(concat!(env!("OUT_DIR"), "/outputs.rs"));
