//! Codegen demo with const inputs

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FuncWithConstInputArgs {
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub plain_input: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct FuncWithConstInputResult {
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: FuncWithConstInputArgs
) -> FuncWithConstInputResult {

    let result = crate::bindings::pulumi::mypkg::func_with_const_input::invoke(
        &crate::bindings::pulumi::mypkg::func_with_const_input::Args {
                plain_input: &args.plain_input.get_inner(),
        }
    );

    FuncWithConstInputResult {
    }
}
