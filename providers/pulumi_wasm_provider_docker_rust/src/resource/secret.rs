//! 
//! 
//! ## Import
//! 
//! ```sh
//! # Docker secret cannot be imported as the secret data, once set, is never exposed again.
//! ```

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct SecretArgs {
    /// Base64-url-safe-encoded secret data
    #[builder(into)]
    pub data: pulumi_wasm_rust::Output<String>,
    /// User-defined key/value metadata
    #[builder(into, default)]
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::SecretLabel>>>,
    /// User-defined name of the secret
    #[builder(into, default)]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct SecretResult {
    /// Base64-url-safe-encoded secret data
    pub data: pulumi_wasm_rust::Output<String>,
    /// User-defined key/value metadata
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::SecretLabel>>>,
    /// User-defined name of the secret
    pub name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: SecretArgs) -> SecretResult {

    let result = crate::bindings::pulumi::docker::secret::invoke(name, &crate::bindings::pulumi::docker::secret::Args {
        data: &args.data.get_inner(),
        labels: &args.labels.get_inner(),
        name: &args.name.get_inner(),
    });

    SecretResult {
        data: crate::into_domain(result.data),
        labels: crate::into_domain(result.labels),
        name: crate::into_domain(result.name),
    }
}
