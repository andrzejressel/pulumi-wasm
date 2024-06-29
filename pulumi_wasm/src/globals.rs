use crate::pulumi_connector_impl::PulumiConnectorImpl;
use pulumi_wasm_core::{Engine, PulumiServiceImpl};
use std::cell::{OnceCell, RefCell};
use std::rc::Rc;

thread_local! {
    static PULUMI_ENGINE: OnceCell<Rc<RefCell<Engine>>> = const { OnceCell::new() };
}

pub(crate) fn get_pulumi_engine() -> Rc<RefCell<Engine>> {
    PULUMI_ENGINE.with(|e| {
        e.get_or_init(|| {
            Rc::new(RefCell::new(Engine::new(PulumiServiceImpl::new(
                PulumiConnectorImpl {},
            ))))
        })
        .clone()
    })
}
