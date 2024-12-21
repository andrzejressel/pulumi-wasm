
#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct moduleTestArgs {
    #[builder(into, default)]
    pub mod1: pulumi_wasm_rust::Output<Option<crate::types::mod1::Typ>>,
    #[builder(into, default)]
    pub val: pulumi_wasm_rust::Output<Option<crate::types::Typ>>,
}

pub struct moduleTestResult {
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: moduleTestArgs) -> moduleTestResult {

    let result = crate::bindings::pulumi::example::module_test::invoke(name, &crate::bindings::pulumi::example::module_test::Args {
        mod1: &args.mod1.get_inner(),
        val: &args.val.get_inner(),
    });

    moduleTestResult {
    }
}
