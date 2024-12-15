
#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FuncWithSecretsArgs {
    #[builder(into)]
    pub crypto_key: pulumi_wasm_rust::Output<String>,
    #[builder(into)]
    pub plaintext: pulumi_wasm_rust::Output<String>,
}

pub struct FuncWithSecretsResult {
    pub ciphertext: pulumi_wasm_rust::Output<String>,
    pub crypto_key: pulumi_wasm_rust::Output<String>,
    pub id: pulumi_wasm_rust::Output<String>,
    pub plaintext: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: FuncWithSecretsArgs
) -> FuncWithSecretsResult {

    let result = crate::bindings::pulumi::mypkg::func_with_secrets::invoke(
        &crate::bindings::pulumi::mypkg::func_with_secrets::Args {
                crypto_key: &args.crypto_key.get_inner(),
                plaintext: &args.plaintext.get_inner(),
        }
    );

    FuncWithSecretsResult {
        ciphertext: crate::into_domain(result.ciphertext),
        crypto_key: crate::into_domain(result.crypto_key),
        id: crate::into_domain(result.id),
        plaintext: crate::into_domain(result.plaintext),
    }
}
