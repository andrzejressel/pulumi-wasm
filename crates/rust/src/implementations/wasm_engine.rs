use crate::implementations::engine::GestaltEngine;
use anyhow::Error;
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::output_interface;
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::pulumi_engine::Engine as WitEngine;
use std::collections::HashMap;

type Function = Box<dyn Fn(&String) -> Result<String, Error> + Send>;

#[derive(Copy, Clone, Debug)]
pub(crate) struct WasmOutput {
    underlying_id: u32,
}

pub(crate) struct WasmEngine {
    wit_engine: WitEngine,
    functions: HashMap<String, Function>,
}

impl GestaltEngine for WasmEngine {
    type Output = WasmOutput;
    fn new(&self, value: String, secret: bool) -> WasmOutput {
        let resource = output_interface::Output::new(&self.wit_engine, value.as_str(), secret);
        let underlying_id = output_interface::Output::take_handle(&resource);
        WasmOutput { underlying_id }
    }
}
