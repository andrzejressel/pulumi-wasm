use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
pub(crate) mod bindings;

pub mod output;

lazy_static! {
    pub static ref HASHMAP: Mutex<HashMap<String, Box<dyn Fn(Vec<u8>) -> Result<Vec<u8>, anyhow::Error> + Send>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}
