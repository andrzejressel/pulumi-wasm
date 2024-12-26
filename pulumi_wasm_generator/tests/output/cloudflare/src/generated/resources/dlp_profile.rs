#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct DlpProfileArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Related DLP policies will trigger when the match count exceeds the number set.
    #[builder(into)]
    pub allowed_match_count: pulumi_wasm_rust::Output<i32>,
    /// Scan the context of predefined entries to only return matches surrounded by keywords.
    #[builder(into, default)]
    pub context_awareness: pulumi_wasm_rust::Output<
        Option<super::types::DlpProfileContextAwareness>,
    >,
    /// Brief summary of the profile and its intended use.
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// List of entries to apply to the profile.
    #[builder(into)]
    pub entries: pulumi_wasm_rust::Output<Vec<super::types::DlpProfileEntry>>,
    /// Name of the profile. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// If true, scan images via OCR to determine if any text present matches filters.
    #[builder(into, default)]
    pub ocr_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
}
pub struct DlpProfileResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Related DLP policies will trigger when the match count exceeds the number set.
    pub allowed_match_count: pulumi_wasm_rust::Output<i32>,
    /// Scan the context of predefined entries to only return matches surrounded by keywords.
    pub context_awareness: pulumi_wasm_rust::Output<
        super::types::DlpProfileContextAwareness,
    >,
    /// Brief summary of the profile and its intended use.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// List of entries to apply to the profile.
    pub entries: pulumi_wasm_rust::Output<Vec<super::types::DlpProfileEntry>>,
    /// Name of the profile. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// If true, scan images via OCR to determine if any text present matches filters.
    pub ocr_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**
    pub type_: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: DlpProfileArgs) -> DlpProfileResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let allowed_match_count_binding = args.allowed_match_count.get_inner();
    let context_awareness_binding = args.context_awareness.get_inner();
    let description_binding = args.description.get_inner();
    let entries_binding = args.entries.get_inner();
    let name_binding = args.name.get_inner();
    let ocr_enabled_binding = args.ocr_enabled.get_inner();
    let type__binding = args.type_.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/dlpProfile:DlpProfile".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "allowedMatchCount".into(),
                value: &allowed_match_count_binding,
            },
            register_interface::ObjectField {
                name: "contextAwareness".into(),
                value: &context_awareness_binding,
            },
            register_interface::ObjectField {
                name: "description".into(),
                value: &description_binding,
            },
            register_interface::ObjectField {
                name: "entries".into(),
                value: &entries_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "ocrEnabled".into(),
                value: &ocr_enabled_binding,
            },
            register_interface::ObjectField {
                name: "type".into(),
                value: &type__binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "allowedMatchCount".into() },
            register_interface::ResultField { name : "contextAwareness".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "entries".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "ocrEnabled".into() },
            register_interface::ResultField { name : "type".into() },
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
    DlpProfileResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        allowed_match_count: into_domain(hashmap.remove("allowedMatchCount").unwrap()),
        context_awareness: into_domain(hashmap.remove("contextAwareness").unwrap()),
        description: into_domain(hashmap.remove("description").unwrap()),
        entries: into_domain(hashmap.remove("entries").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        ocr_enabled: into_domain(hashmap.remove("ocrEnabled").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
    }
}
