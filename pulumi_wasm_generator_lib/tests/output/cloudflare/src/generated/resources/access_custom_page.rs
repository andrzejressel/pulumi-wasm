#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AccessCustomPageArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Number of apps to display on the custom page.
    #[builder(into, default)]
    pub app_count: pulumi_wasm_rust::Output<Option<i32>>,
    /// Custom HTML to display on the custom page.
    #[builder(into, default)]
    pub custom_html: pulumi_wasm_rust::Output<Option<String>>,
    /// Friendly name of the Access Custom Page configuration.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Type of Access custom page to create. Available values: `identity_denied`, `forbidden`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct AccessCustomPageResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Number of apps to display on the custom page.
    pub app_count: pulumi_wasm_rust::Output<Option<i32>>,
    /// Custom HTML to display on the custom page.
    pub custom_html: pulumi_wasm_rust::Output<Option<String>>,
    /// Friendly name of the Access Custom Page configuration.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Type of Access custom page to create. Available values: `identity_denied`, `forbidden`.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: AccessCustomPageArgs) -> AccessCustomPageResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let app_count_binding = args.app_count.get_inner();
    let custom_html_binding = args.custom_html.get_inner();
    let name_binding = args.name.get_inner();
    let type__binding = args.type_.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/accessCustomPage:AccessCustomPage".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "appCount".into(),
                value: &app_count_binding,
            },
            register_interface::ObjectField {
                name: "customHtml".into(),
                value: &custom_html_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "type".into(),
                value: &type__binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "appCount".into() },
            register_interface::ResultField { name : "customHtml".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "type".into() },
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
    AccessCustomPageResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        app_count: into_domain(hashmap.remove("appCount").unwrap()),
        custom_html: into_domain(hashmap.remove("customHtml").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}