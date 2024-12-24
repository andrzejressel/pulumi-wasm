#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TurnstileWidgetArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// If bot*fight*mode is set to true, Cloudflare issues computationally expensive challenges in response to malicious bots (Enterprise only).
    #[builder(into, default)]
    pub bot_fight_mode: pulumi_wasm_rust::Output<Option<bool>>,
    /// Domains where the widget is deployed
    #[builder(into)]
    pub domains: pulumi_wasm_rust::Output<Vec<String>>,
    /// Widget Mode. Available values: `non-interactive`, `invisible`, `managed`
    #[builder(into)]
    pub mode: pulumi_wasm_rust::Output<String>,
    /// Human readable widget name.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Do not show any Cloudflare branding on the widget (Enterprise only).
    #[builder(into, default)]
    pub offlabel: pulumi_wasm_rust::Output<Option<bool>>,
    /// Region where this widget can be used.
    #[builder(into, default)]
    pub region: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct TurnstileWidgetResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// If bot*fight*mode is set to true, Cloudflare issues computationally expensive challenges in response to malicious bots (Enterprise only).
    pub bot_fight_mode: pulumi_wasm_rust::Output<bool>,
    /// Domains where the widget is deployed
    pub domains: pulumi_wasm_rust::Output<Vec<String>>,
    /// Widget Mode. Available values: `non-interactive`, `invisible`, `managed`
    pub mode: pulumi_wasm_rust::Output<String>,
    /// Human readable widget name.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Do not show any Cloudflare branding on the widget (Enterprise only).
    pub offlabel: pulumi_wasm_rust::Output<bool>,
    /// Region where this widget can be used.
    pub region: pulumi_wasm_rust::Output<String>,
    /// Secret key for this widget.
    pub secret: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TurnstileWidgetArgs) -> TurnstileWidgetResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let bot_fight_mode_binding = args.bot_fight_mode.get_inner();
    let domains_binding = args.domains.get_inner();
    let mode_binding = args.mode.get_inner();
    let name_binding = args.name.get_inner();
    let offlabel_binding = args.offlabel.get_inner();
    let region_binding = args.region.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/turnstileWidget:TurnstileWidget".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "botFightMode".into(),
                value: &bot_fight_mode_binding,
            },
            register_interface::ObjectField {
                name: "domains".into(),
                value: &domains_binding,
            },
            register_interface::ObjectField {
                name: "mode".into(),
                value: &mode_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "offlabel".into(),
                value: &offlabel_binding,
            },
            register_interface::ObjectField {
                name: "region".into(),
                value: &region_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "botFightMode".into() },
            register_interface::ResultField { name : "domains".into() },
            register_interface::ResultField { name : "mode".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "offlabel".into() },
            register_interface::ResultField { name : "region".into() },
            register_interface::ResultField { name : "secret".into() },
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
    TurnstileWidgetResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        bot_fight_mode: into_domain(hashmap.remove("botFightMode").unwrap()),
        domains: into_domain(hashmap.remove("domains").unwrap()),
        mode: into_domain(hashmap.remove("mode").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        offlabel: into_domain(hashmap.remove("offlabel").unwrap()),
        region: into_domain(hashmap.remove("region").unwrap()),
        secret: into_domain(hashmap.remove("secret").unwrap()),
    }
}
