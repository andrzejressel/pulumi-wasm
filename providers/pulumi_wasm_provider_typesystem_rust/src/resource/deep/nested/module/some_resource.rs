
#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct SomeResourceArgs {
    #[builder(into, default)]
    pub string_input: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct SomeResourceResult {
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: SomeResourceArgs) -> SomeResourceResult {

    let result = crate::bindings::pulumi::typesystem::deep_nested_module_some_resource::invoke(name, &crate::bindings::pulumi::typesystem::deep_nested_module_some_resource::Args {
        string_input: &args.string_input.get_inner(),
    });

    SomeResourceResult {
    }
}
