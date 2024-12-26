#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RecordArgs {
    #[builder(into, default)]
    pub allow_overwrite: pulumi_wasm_rust::Output<Option<bool>>,
    /// Comments or notes about the DNS record. This field has no effect on DNS responses.
    #[builder(into, default)]
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// The content of the record. Must provide only one of `data`, `content`, `value`.
    #[builder(into, default)]
    pub content: pulumi_wasm_rust::Output<Option<String>>,
    /// Map of attributes that constitute the record value. Must provide only one of `data`, `content`, `value`.
    #[builder(into, default)]
    pub data: pulumi_wasm_rust::Output<Option<super::types::RecordData>>,
    /// The name of the record. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The priority of the record.
    #[builder(into, default)]
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// Whether the record gets Cloudflare's origin protection.
    #[builder(into, default)]
    pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
    /// Custom tags for the DNS record.
    #[builder(into, default)]
    pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The TTL of the record.
    #[builder(into, default)]
    pub ttl: pulumi_wasm_rust::Output<Option<i32>>,
    /// The type of the record. Available values: `A`, `AAAA`, `CAA`, `CNAME`, `TXT`, `SRV`, `LOC`, `MX`, `NS`, `SPF`, `CERT`, `DNSKEY`, `DS`, `NAPTR`, `SMIMEA`, `SSHFP`, `TLSA`, `URI`, `PTR`, `HTTPS`, `SVCB`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The value of the record. Must provide only one of `data`, `content`, `value`.
    #[builder(into, default)]
    pub value: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct RecordResult {
    pub allow_overwrite: pulumi_wasm_rust::Output<Option<bool>>,
    /// Comments or notes about the DNS record. This field has no effect on DNS responses.
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// The content of the record. Must provide only one of `data`, `content`, `value`.
    pub content: pulumi_wasm_rust::Output<String>,
    /// The RFC3339 timestamp of when the record was created.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// Map of attributes that constitute the record value. Must provide only one of `data`, `content`, `value`.
    pub data: pulumi_wasm_rust::Output<Option<super::types::RecordData>>,
    /// The FQDN of the record.
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// A key-value map of string metadata Cloudflare associates with the record.
    pub metadata: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    /// The RFC3339 timestamp of when the record was last modified.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// The name of the record. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// The priority of the record.
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// Shows whether this record can be proxied.
    pub proxiable: pulumi_wasm_rust::Output<bool>,
    /// Whether the record gets Cloudflare's origin protection.
    pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
    /// Custom tags for the DNS record.
    pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The TTL of the record.
    pub ttl: pulumi_wasm_rust::Output<i32>,
    /// The type of the record. Available values: `A`, `AAAA`, `CAA`, `CNAME`, `TXT`, `SRV`, `LOC`, `MX`, `NS`, `SPF`, `CERT`, `DNSKEY`, `DS`, `NAPTR`, `SMIMEA`, `SSHFP`, `TLSA`, `URI`, `PTR`, `HTTPS`, `SVCB`. **Modifying this attribute will force creation of a new resource.**
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The value of the record. Must provide only one of `data`, `content`, `value`.
    pub value: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: RecordArgs) -> RecordResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let allow_overwrite_binding = args.allow_overwrite.get_inner();
    let comment_binding = args.comment.get_inner();
    let content_binding = args.content.get_inner();
    let data_binding = args.data.get_inner();
    let name_binding = args.name.get_inner();
    let priority_binding = args.priority.get_inner();
    let proxied_binding = args.proxied.get_inner();
    let tags_binding = args.tags.get_inner();
    let ttl_binding = args.ttl.get_inner();
    let type__binding = args.type_.get_inner();
    let value_binding = args.value.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/record:Record".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "allowOverwrite".into(),
                value: &allow_overwrite_binding,
            },
            register_interface::ObjectField {
                name: "comment".into(),
                value: &comment_binding,
            },
            register_interface::ObjectField {
                name: "content".into(),
                value: &content_binding,
            },
            register_interface::ObjectField {
                name: "data".into(),
                value: &data_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "priority".into(),
                value: &priority_binding,
            },
            register_interface::ObjectField {
                name: "proxied".into(),
                value: &proxied_binding,
            },
            register_interface::ObjectField {
                name: "tags".into(),
                value: &tags_binding,
            },
            register_interface::ObjectField {
                name: "ttl".into(),
                value: &ttl_binding,
            },
            register_interface::ObjectField {
                name: "type".into(),
                value: &type__binding,
            },
            register_interface::ObjectField {
                name: "value".into(),
                value: &value_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "allowOverwrite".into() },
            register_interface::ResultField { name : "comment".into() },
            register_interface::ResultField { name : "content".into() },
            register_interface::ResultField { name : "createdOn".into() },
            register_interface::ResultField { name : "data".into() },
            register_interface::ResultField { name : "hostname".into() },
            register_interface::ResultField { name : "metadata".into() },
            register_interface::ResultField { name : "modifiedOn".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "priority".into() },
            register_interface::ResultField { name : "proxiable".into() },
            register_interface::ResultField { name : "proxied".into() },
            register_interface::ResultField { name : "tags".into() },
            register_interface::ResultField { name : "ttl".into() },
            register_interface::ResultField { name : "type".into() },
            register_interface::ResultField { name : "value".into() },
            register_interface::ResultField { name : "zoneId".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    RecordResult {
        allow_overwrite: into_domain(hashmap.remove("allowOverwrite").unwrap()),
        comment: into_domain(hashmap.remove("comment").unwrap()),
        content: into_domain(hashmap.remove("content").unwrap()),
        created_on: into_domain(hashmap.remove("createdOn").unwrap()),
        data: into_domain(hashmap.remove("data").unwrap()),
        hostname: into_domain(hashmap.remove("hostname").unwrap()),
        metadata: into_domain(hashmap.remove("metadata").unwrap()),
        modified_on: into_domain(hashmap.remove("modifiedOn").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        priority: into_domain(hashmap.remove("priority").unwrap()),
        proxiable: into_domain(hashmap.remove("proxiable").unwrap()),
        proxied: into_domain(hashmap.remove("proxied").unwrap()),
        tags: into_domain(hashmap.remove("tags").unwrap()),
        ttl: into_domain(hashmap.remove("ttl").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
        value: into_domain(hashmap.remove("value").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
