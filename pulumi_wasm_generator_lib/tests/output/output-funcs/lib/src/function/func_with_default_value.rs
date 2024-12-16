//! Check codegen of functions with default values.

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FuncWithDefaultValueArgs {
    #[builder(into)]
    pub a: pulumi_wasm_rust::Output<String>,
    #[builder(into, default)]
    pub b: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct FuncWithDefaultValueResult {
    pub r: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: FuncWithDefaultValueArgs
) -> FuncWithDefaultValueResult {

    let result = crate::bindings::pulumi::mypkg::func_with_default_value::invoke(
        &crate::bindings::pulumi::mypkg::func_with_default_value::Args {
                a: &args.a.get_inner(),
                b: &args.b.get_inner(),
        }
    );

    FuncWithDefaultValueResult {
        r: crate::into_domain(result.r),
    }
}
