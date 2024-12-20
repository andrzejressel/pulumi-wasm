//! Check codegen of functions with all optional inputs.

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FuncWithAllOptionalInputsArgs {
    /// Property A
    #[builder(into, default)]
    pub a: pulumi_wasm_rust::Output<Option<crate::types::HelmReleaseSettings>>,
    /// Property B
    #[builder(into, default)]
    pub b: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct FuncWithAllOptionalInputsResult {
    pub r: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: FuncWithAllOptionalInputsArgs
) -> FuncWithAllOptionalInputsResult {

    let result = crate::bindings::pulumi::example::func_with_all_optional_inputs::invoke(
        &crate::bindings::pulumi::example::func_with_all_optional_inputs::Args {
                a: &args.a.get_inner(),
                b: &args.b.get_inner(),
        }
    );

    FuncWithAllOptionalInputsResult {
        r: crate::into_domain(result.r),
    }
}
