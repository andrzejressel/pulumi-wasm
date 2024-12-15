//! n/a

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FuncWithEmptyOutputsArgs {
    /// The Name of the FeatureGroup.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct FuncWithEmptyOutputsResult {
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: FuncWithEmptyOutputsArgs
) -> FuncWithEmptyOutputsResult {

    let result = crate::bindings::pulumi::mypkg::func_with_empty_outputs::invoke(
        &crate::bindings::pulumi::mypkg::func_with_empty_outputs::Args {
                name: &args.name.get_inner(),
        }
    );

    FuncWithEmptyOutputsResult {
    }
}
