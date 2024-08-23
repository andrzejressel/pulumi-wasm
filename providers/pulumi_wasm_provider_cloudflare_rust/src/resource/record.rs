pub struct RecordArgs {
    pub allow_overwrite: pulumi_wasm_rust::Output<Option<bool>>,
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    pub data: pulumi_wasm_rust::Output<Option<crate::types::RecordData>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
    pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub ttl: pulumi_wasm_rust::Output<Option<i32>>,
    pub type_: pulumi_wasm_rust::Output<String>,
    pub value: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct RecordResult {
    pub allow_overwrite: pulumi_wasm_rust::Output<Option<bool>>,
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    pub created_on: pulumi_wasm_rust::Output<String>,
    pub data: pulumi_wasm_rust::Output<Option<crate::types::RecordData>>,
    pub hostname: pulumi_wasm_rust::Output<String>,
    pub metadata: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub modified_on: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    pub proxiable: pulumi_wasm_rust::Output<bool>,
    pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
    pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub ttl: pulumi_wasm_rust::Output<i32>,
    pub type_: pulumi_wasm_rust::Output<String>,
    pub value: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: RecordArgs) -> RecordResult {
    let result = crate::bindings::pulumi::cloudflare::record::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::record::Args {
            allow_overwrite: args.allow_overwrite.get_inner(),
            comment: args.comment.get_inner(),
            data: args.data.get_inner(),
            name: args.name.get_inner(),
            priority: args.priority.get_inner(),
            proxied: args.proxied.get_inner(),
            tags: args.tags.get_inner(),
            ttl: args.ttl.get_inner(),
            type_: args.type_.get_inner(),
            value: args.value.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    RecordResult {
        allow_overwrite: crate::into_domain(result.allow_overwrite),
        comment: crate::into_domain(result.comment),
        created_on: crate::into_domain(result.created_on),
        data: crate::into_domain(result.data),
        hostname: crate::into_domain(result.hostname),
        metadata: crate::into_domain(result.metadata),
        modified_on: crate::into_domain(result.modified_on),
        name: crate::into_domain(result.name),
        priority: crate::into_domain(result.priority),
        proxiable: crate::into_domain(result.proxiable),
        proxied: crate::into_domain(result.proxied),
        tags: crate::into_domain(result.tags),
        ttl: crate::into_domain(result.ttl),
        type_: crate::into_domain(result.type_),
        value: crate::into_domain(result.value),
        zone_id: crate::into_domain(result.zone_id),
    }
}
