#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ListItemArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Autonomous system number to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    #[builder(into, default)]
    pub asn: pulumi_wasm_rust::Output<Option<i32>>,
    /// An optional comment for the item.
    #[builder(into, default)]
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// Hostname to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    #[builder(into, default)]
    pub hostname: pulumi_wasm_rust::Output<Option<super::types::ListItemHostname>>,
    /// IP address to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    #[builder(into, default)]
    pub ip: pulumi_wasm_rust::Output<Option<String>>,
    /// The list identifier to target for the resource.
    #[builder(into)]
    pub list_id: pulumi_wasm_rust::Output<String>,
    /// Redirect configuration to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    #[builder(into, default)]
    pub redirect: pulumi_wasm_rust::Output<Option<super::types::ListItemRedirect>>,
}
pub struct ListItemResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Autonomous system number to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    pub asn: pulumi_wasm_rust::Output<Option<i32>>,
    /// An optional comment for the item.
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// Hostname to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    pub hostname: pulumi_wasm_rust::Output<Option<super::types::ListItemHostname>>,
    /// IP address to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    pub ip: pulumi_wasm_rust::Output<Option<String>>,
    /// The list identifier to target for the resource.
    pub list_id: pulumi_wasm_rust::Output<String>,
    /// Redirect configuration to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    pub redirect: pulumi_wasm_rust::Output<Option<super::types::ListItemRedirect>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: ListItemArgs) -> ListItemResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let asn_binding = args.asn.get_inner();
    let comment_binding = args.comment.get_inner();
    let hostname_binding = args.hostname.get_inner();
    let ip_binding = args.ip.get_inner();
    let list_id_binding = args.list_id.get_inner();
    let redirect_binding = args.redirect.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/listItem:ListItem".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "asn".into(),
                value: &asn_binding,
            },
            register_interface::ObjectField {
                name: "comment".into(),
                value: &comment_binding,
            },
            register_interface::ObjectField {
                name: "hostname".into(),
                value: &hostname_binding,
            },
            register_interface::ObjectField {
                name: "ip".into(),
                value: &ip_binding,
            },
            register_interface::ObjectField {
                name: "listId".into(),
                value: &list_id_binding,
            },
            register_interface::ObjectField {
                name: "redirect".into(),
                value: &redirect_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "asn".into() },
            register_interface::ResultField { name : "comment".into() },
            register_interface::ResultField { name : "hostname".into() },
            register_interface::ResultField { name : "ip".into() },
            register_interface::ResultField { name : "listId".into() },
            register_interface::ResultField { name : "redirect".into() },
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
    ListItemResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        asn: into_domain(hashmap.remove("asn").unwrap()),
        comment: into_domain(hashmap.remove("comment").unwrap()),
        hostname: into_domain(hashmap.remove("hostname").unwrap()),
        ip: into_domain(hashmap.remove("ip").unwrap()),
        list_id: into_domain(hashmap.remove("listId").unwrap()),
        redirect: into_domain(hashmap.remove("redirect").unwrap()),
    }
}
