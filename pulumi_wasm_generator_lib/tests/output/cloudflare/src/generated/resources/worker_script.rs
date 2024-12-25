#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkerScriptArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    #[builder(into, default)]
    pub analytics_engine_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptAnalyticsEngineBinding>>,
    >,
    /// The date to use for the compatibility flag.
    #[builder(into, default)]
    pub compatibility_date: pulumi_wasm_rust::Output<Option<String>>,
    /// Compatibility flags used for Worker Scripts.
    #[builder(into, default)]
    pub compatibility_flags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The script content.
    #[builder(into)]
    pub content: pulumi_wasm_rust::Output<String>,
    #[builder(into, default)]
    pub d1_database_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptD1DatabaseBinding>>,
    >,
    /// Name of the Workers for Platforms dispatch namespace.
    #[builder(into, default)]
    pub dispatch_namespace: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default)]
    pub hyperdrive_config_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptHyperdriveConfigBinding>>,
    >,
    #[builder(into, default)]
    pub kv_namespace_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptKvNamespaceBinding>>,
    >,
    /// Enabling allows Worker events to be sent to a defined Logpush destination.
    #[builder(into, default)]
    pub logpush: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to upload Worker as a module.
    #[builder(into, default)]
    pub module: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name for the script. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    #[builder(into, default)]
    pub placements: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptPlacement>>,
    >,
    #[builder(into, default)]
    pub plain_text_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptPlainTextBinding>>,
    >,
    #[builder(into, default)]
    pub queue_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptQueueBinding>>,
    >,
    #[builder(into, default)]
    pub r2_bucket_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptR2BucketBinding>>,
    >,
    #[builder(into, default)]
    pub secret_text_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptSecretTextBinding>>,
    >,
    #[builder(into, default)]
    pub service_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptServiceBinding>>,
    >,
    #[builder(into, default)]
    pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    #[builder(into, default)]
    pub webassembly_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptWebassemblyBinding>>,
    >,
}
pub struct WorkerScriptResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub analytics_engine_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptAnalyticsEngineBinding>>,
    >,
    /// The date to use for the compatibility flag.
    pub compatibility_date: pulumi_wasm_rust::Output<Option<String>>,
    /// Compatibility flags used for Worker Scripts.
    pub compatibility_flags: pulumi_wasm_rust::Output<Vec<String>>,
    /// The script content.
    pub content: pulumi_wasm_rust::Output<String>,
    pub d1_database_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptD1DatabaseBinding>>,
    >,
    /// Name of the Workers for Platforms dispatch namespace.
    pub dispatch_namespace: pulumi_wasm_rust::Output<Option<String>>,
    pub hyperdrive_config_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptHyperdriveConfigBinding>>,
    >,
    pub kv_namespace_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptKvNamespaceBinding>>,
    >,
    /// Enabling allows Worker events to be sent to a defined Logpush destination.
    pub logpush: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to upload Worker as a module.
    pub module: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name for the script. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    pub placements: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptPlacement>>,
    >,
    pub plain_text_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptPlainTextBinding>>,
    >,
    pub queue_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptQueueBinding>>,
    >,
    pub r2_bucket_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptR2BucketBinding>>,
    >,
    pub secret_text_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptSecretTextBinding>>,
    >,
    pub service_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptServiceBinding>>,
    >,
    pub tags: pulumi_wasm_rust::Output<Vec<String>>,
    pub webassembly_bindings: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WorkerScriptWebassemblyBinding>>,
    >,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: WorkerScriptArgs) -> WorkerScriptResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let analytics_engine_bindings_binding = args.analytics_engine_bindings.get_inner();
    let compatibility_date_binding = args.compatibility_date.get_inner();
    let compatibility_flags_binding = args.compatibility_flags.get_inner();
    let content_binding = args.content.get_inner();
    let d1_database_bindings_binding = args.d1_database_bindings.get_inner();
    let dispatch_namespace_binding = args.dispatch_namespace.get_inner();
    let hyperdrive_config_bindings_binding = args.hyperdrive_config_bindings.get_inner();
    let kv_namespace_bindings_binding = args.kv_namespace_bindings.get_inner();
    let logpush_binding = args.logpush.get_inner();
    let module_binding = args.module.get_inner();
    let name_binding = args.name.get_inner();
    let placements_binding = args.placements.get_inner();
    let plain_text_bindings_binding = args.plain_text_bindings.get_inner();
    let queue_bindings_binding = args.queue_bindings.get_inner();
    let r2_bucket_bindings_binding = args.r2_bucket_bindings.get_inner();
    let secret_text_bindings_binding = args.secret_text_bindings.get_inner();
    let service_bindings_binding = args.service_bindings.get_inner();
    let tags_binding = args.tags.get_inner();
    let webassembly_bindings_binding = args.webassembly_bindings.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/workerScript:WorkerScript".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "analyticsEngineBindings".into(),
                value: &analytics_engine_bindings_binding,
            },
            register_interface::ObjectField {
                name: "compatibilityDate".into(),
                value: &compatibility_date_binding,
            },
            register_interface::ObjectField {
                name: "compatibilityFlags".into(),
                value: &compatibility_flags_binding,
            },
            register_interface::ObjectField {
                name: "content".into(),
                value: &content_binding,
            },
            register_interface::ObjectField {
                name: "d1DatabaseBindings".into(),
                value: &d1_database_bindings_binding,
            },
            register_interface::ObjectField {
                name: "dispatchNamespace".into(),
                value: &dispatch_namespace_binding,
            },
            register_interface::ObjectField {
                name: "hyperdriveConfigBindings".into(),
                value: &hyperdrive_config_bindings_binding,
            },
            register_interface::ObjectField {
                name: "kvNamespaceBindings".into(),
                value: &kv_namespace_bindings_binding,
            },
            register_interface::ObjectField {
                name: "logpush".into(),
                value: &logpush_binding,
            },
            register_interface::ObjectField {
                name: "module".into(),
                value: &module_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "placements".into(),
                value: &placements_binding,
            },
            register_interface::ObjectField {
                name: "plainTextBindings".into(),
                value: &plain_text_bindings_binding,
            },
            register_interface::ObjectField {
                name: "queueBindings".into(),
                value: &queue_bindings_binding,
            },
            register_interface::ObjectField {
                name: "r2BucketBindings".into(),
                value: &r2_bucket_bindings_binding,
            },
            register_interface::ObjectField {
                name: "secretTextBindings".into(),
                value: &secret_text_bindings_binding,
            },
            register_interface::ObjectField {
                name: "serviceBindings".into(),
                value: &service_bindings_binding,
            },
            register_interface::ObjectField {
                name: "tags".into(),
                value: &tags_binding,
            },
            register_interface::ObjectField {
                name: "webassemblyBindings".into(),
                value: &webassembly_bindings_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "analyticsEngineBindings".into() },
            register_interface::ResultField { name : "compatibilityDate".into() },
            register_interface::ResultField { name : "compatibilityFlags".into() },
            register_interface::ResultField { name : "content".into() },
            register_interface::ResultField { name : "d1DatabaseBindings".into() },
            register_interface::ResultField { name : "dispatchNamespace".into() },
            register_interface::ResultField { name : "hyperdriveConfigBindings".into() },
            register_interface::ResultField { name : "kvNamespaceBindings".into() },
            register_interface::ResultField { name : "logpush".into() },
            register_interface::ResultField { name : "module".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "placements".into() },
            register_interface::ResultField { name : "plainTextBindings".into() },
            register_interface::ResultField { name : "queueBindings".into() },
            register_interface::ResultField { name : "r2BucketBindings".into() },
            register_interface::ResultField { name : "secretTextBindings".into() },
            register_interface::ResultField { name : "serviceBindings".into() },
            register_interface::ResultField { name : "tags".into() },
            register_interface::ResultField { name : "webassemblyBindings".into() },
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
    WorkerScriptResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        analytics_engine_bindings: into_domain(
            hashmap.remove("analyticsEngineBindings").unwrap(),
        ),
        compatibility_date: into_domain(hashmap.remove("compatibilityDate").unwrap()),
        compatibility_flags: into_domain(hashmap.remove("compatibilityFlags").unwrap()),
        content: into_domain(hashmap.remove("content").unwrap()),
        d1_database_bindings: into_domain(hashmap.remove("d1DatabaseBindings").unwrap()),
        dispatch_namespace: into_domain(hashmap.remove("dispatchNamespace").unwrap()),
        hyperdrive_config_bindings: into_domain(
            hashmap.remove("hyperdriveConfigBindings").unwrap(),
        ),
        kv_namespace_bindings: into_domain(
            hashmap.remove("kvNamespaceBindings").unwrap(),
        ),
        logpush: into_domain(hashmap.remove("logpush").unwrap()),
        module: into_domain(hashmap.remove("module").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        placements: into_domain(hashmap.remove("placements").unwrap()),
        plain_text_bindings: into_domain(hashmap.remove("plainTextBindings").unwrap()),
        queue_bindings: into_domain(hashmap.remove("queueBindings").unwrap()),
        r2_bucket_bindings: into_domain(hashmap.remove("r2BucketBindings").unwrap()),
        secret_text_bindings: into_domain(hashmap.remove("secretTextBindings").unwrap()),
        service_bindings: into_domain(hashmap.remove("serviceBindings").unwrap()),
        tags: into_domain(hashmap.remove("tags").unwrap()),
        webassembly_bindings: into_domain(hashmap.remove("webassemblyBindings").unwrap()),
    }
}
