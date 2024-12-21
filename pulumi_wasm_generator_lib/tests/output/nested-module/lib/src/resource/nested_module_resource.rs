
#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ResourceArgs {
    #[builder(into, default)]
    pub bar: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct ResourceResult {
    pub bar: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ResourceArgs) -> ResourceResult {

    let result = crate::bindings::pulumi::foo::nested_module_resource::invoke(name, &crate::bindings::pulumi::foo::nested_module_resource::Args {
        bar: &args.bar.get_inner(),
    });

    ResourceResult {
        bar: crate::into_domain(result.bar),
    }
}
