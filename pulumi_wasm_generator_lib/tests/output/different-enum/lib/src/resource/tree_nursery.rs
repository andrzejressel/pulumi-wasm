
#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct NurseryArgs {
    /// The sizes of trees available
    #[builder(into, default)]
    pub sizes: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, crate::types::TreeSize>>>,
    /// The varieties available
    #[builder(into)]
    pub varieties: pulumi_wasm_rust::Output<Vec<crate::types::RubberTreeVariety>>,
}

pub struct NurseryResult {
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: NurseryArgs) -> NurseryResult {

    let result = crate::bindings::pulumi::plant::tree_nursery::invoke(name, &crate::bindings::pulumi::plant::tree_nursery::Args {
        sizes: &args.sizes.get_inner(),
        varieties: &args.varieties.get_inner(),
    });

    NurseryResult {
    }
}
