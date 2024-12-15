//! Check codegen of functions with a Dict<str,str> parameter.

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FuncWithDictParamArgs {
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub a: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub b: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct FuncWithDictParamResult {
    pub r: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: FuncWithDictParamArgs
) -> FuncWithDictParamResult {

    let result = crate::bindings::pulumi::mypkg::func_with_dict_param::invoke(
        &crate::bindings::pulumi::mypkg::func_with_dict_param::Args {
                a: &args.a.get_inner(),
                b: &args.b.get_inner(),
        }
    );

    FuncWithDictParamResult {
        r: crate::into_domain(result.r),
    }
}
