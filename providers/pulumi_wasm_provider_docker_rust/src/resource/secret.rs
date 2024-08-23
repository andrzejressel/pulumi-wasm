pub struct SecretArgs {
    pub data: pulumi_wasm_rust::Output<String>,
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::SecretLabel>>>,
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct SecretResult {
    pub data: pulumi_wasm_rust::Output<String>,
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::SecretLabel>>>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: SecretArgs) -> SecretResult {
    let result = crate::bindings::pulumi::docker::secret::invoke(
        name,
        &crate::bindings::pulumi::docker::secret::Args {
            data: args.data.get_inner(),
            labels: args.labels.get_inner(),
            name: args.name.get_inner(),
        },
    );

    SecretResult {
        data: crate::into_domain(result.data),
        labels: crate::into_domain(result.labels),
        name: crate::into_domain(result.name),
    }
}
