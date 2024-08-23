pub struct R2BucketArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub location: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct R2BucketResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub location: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: R2BucketArgs) -> R2BucketResult {
    let result = crate::bindings::pulumi::cloudflare::r2_bucket::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::r2_bucket::Args {
            account_id: args.account_id.get_inner(),
            location: args.location.get_inner(),
            name: args.name.get_inner(),
        },
    );

    R2BucketResult {
        account_id: crate::into_domain(result.account_id),
        location: crate::into_domain(result.location),
        name: crate::into_domain(result.name),
    }
}
