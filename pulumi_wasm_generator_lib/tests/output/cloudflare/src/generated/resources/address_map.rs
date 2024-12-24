#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AddressMapArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// If you have legacy TLS clients which do not send the TLS server name indicator, then you can specify one default SNI on the map.
    #[builder(into, default)]
    pub default_sni: pulumi_wasm_rust::Output<Option<String>>,
    /// Description of the address map.
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether the Address Map is enabled or not.
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The set of IPs on the Address Map.
    #[builder(into, default)]
    pub ips: pulumi_wasm_rust::Output<Option<Vec<super::types::AddressMapIp>>>,
    /// Zones and Accounts which will be assigned IPs on this Address Map.
    #[builder(into, default)]
    pub memberships: pulumi_wasm_rust::Output<
        Option<Vec<super::types::AddressMapMembership>>,
    >,
}
pub struct AddressMapResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// If set to false, then the Address Map cannot be deleted via API. This is true for Cloudflare-managed maps.
    pub can_delete: pulumi_wasm_rust::Output<bool>,
    /// If set to false, then the IPs on the Address Map cannot be modified via the API. This is true for Cloudflare-managed maps.
    pub can_modify_ips: pulumi_wasm_rust::Output<bool>,
    /// If you have legacy TLS clients which do not send the TLS server name indicator, then you can specify one default SNI on the map.
    pub default_sni: pulumi_wasm_rust::Output<Option<String>>,
    /// Description of the address map.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether the Address Map is enabled or not.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The set of IPs on the Address Map.
    pub ips: pulumi_wasm_rust::Output<Option<Vec<super::types::AddressMapIp>>>,
    /// Zones and Accounts which will be assigned IPs on this Address Map.
    pub memberships: pulumi_wasm_rust::Output<
        Option<Vec<super::types::AddressMapMembership>>,
    >,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AddressMapArgs) -> AddressMapResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let default_sni_binding = args.default_sni.get_inner();
    let description_binding = args.description.get_inner();
    let enabled_binding = args.enabled.get_inner();
    let ips_binding = args.ips.get_inner();
    let memberships_binding = args.memberships.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/addressMap:AddressMap".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "defaultSni".into(),
                value: &default_sni_binding,
            },
            register_interface::ObjectField {
                name: "description".into(),
                value: &description_binding,
            },
            register_interface::ObjectField {
                name: "enabled".into(),
                value: &enabled_binding,
            },
            register_interface::ObjectField {
                name: "ips".into(),
                value: &ips_binding,
            },
            register_interface::ObjectField {
                name: "memberships".into(),
                value: &memberships_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "canDelete".into() },
            register_interface::ResultField { name : "canModifyIps".into() },
            register_interface::ResultField { name : "defaultSni".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "enabled".into() },
            register_interface::ResultField { name : "ips".into() },
            register_interface::ResultField { name : "memberships".into() },
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
    AddressMapResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        can_delete: into_domain(hashmap.remove("canDelete").unwrap()),
        can_modify_ips: into_domain(hashmap.remove("canModifyIps").unwrap()),
        default_sni: into_domain(hashmap.remove("defaultSni").unwrap()),
        description: into_domain(hashmap.remove("description").unwrap()),
        enabled: into_domain(hashmap.remove("enabled").unwrap()),
        ips: into_domain(hashmap.remove("ips").unwrap()),
        memberships: into_domain(hashmap.remove("memberships").unwrap()),
    }
}
