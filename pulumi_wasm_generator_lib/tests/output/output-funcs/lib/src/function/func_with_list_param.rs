//! Check codegen of functions with a List parameter.

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FuncWithListParamArgs {
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub a: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub b: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct FuncWithListParamResult {
    pub r: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: FuncWithListParamArgs
) -> FuncWithListParamResult {

    let result = crate::bindings::pulumi::mypkg::func_with_list_param::invoke(
        &crate::bindings::pulumi::mypkg::func_with_list_param::Args {
                a: &args.a.get_inner(),
                b: &args.b.get_inner(),
        }
    );

    FuncWithListParamResult {
        r: crate::into_domain(result.r),
    }
}
