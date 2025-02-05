use pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::pulumi_engine::Engine as WitEngine;

pub struct PulumiContext {
    pub(crate) wit_engine: WitEngine,
}

impl PulumiContext {
    pub(crate) fn new(in_preview: bool) -> PulumiContext {
        PulumiContext {
            wit_engine: WitEngine::new(in_preview),
        }
    }

    #[doc(hidden)]
    pub fn get_inner(&self) -> &WitEngine {
        &self.wit_engine
    }
}
