#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ByoIpPrefixArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether or not the prefix shall be announced. A prefix can be activated or deactivated once every 15 minutes (attempting more regular updates will trigger rate limiting). Available values: `on`, `off`.
    #[builder(into, default)]
    pub advertisement: pulumi_wasm_rust::Output<Option<String>>,
    /// Description of the BYO IP prefix.
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The assigned Bring-Your-Own-IP prefix ID. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub prefix_id: pulumi_wasm_rust::Output<String>,
}
pub struct ByoIpPrefixResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether or not the prefix shall be announced. A prefix can be activated or deactivated once every 15 minutes (attempting more regular updates will trigger rate limiting). Available values: `on`, `off`.
    pub advertisement: pulumi_wasm_rust::Output<String>,
    /// Description of the BYO IP prefix.
    pub description: pulumi_wasm_rust::Output<String>,
    /// The assigned Bring-Your-Own-IP prefix ID. **Modifying this attribute will force creation of a new resource.**
    pub prefix_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: ByoIpPrefixArgs) -> ByoIpPrefixResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let advertisement_binding = args.advertisement.get_inner();
    let description_binding = args.description.get_inner();
    let prefix_id_binding = args.prefix_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/byoIpPrefix:ByoIpPrefix".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "advertisement".into(),
                value: &advertisement_binding,
            },
            register_interface::ObjectField {
                name: "description".into(),
                value: &description_binding,
            },
            register_interface::ObjectField {
                name: "prefixId".into(),
                value: &prefix_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "advertisement".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "prefixId".into() },
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
    ByoIpPrefixResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        advertisement: into_domain(hashmap.remove("advertisement").unwrap()),
        description: into_domain(hashmap.remove("description").unwrap()),
        prefix_id: into_domain(hashmap.remove("prefixId").unwrap()),
    }
}
