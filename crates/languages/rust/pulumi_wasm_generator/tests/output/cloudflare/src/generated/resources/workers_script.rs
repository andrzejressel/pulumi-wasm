/// Provides a Cloudflare worker script resource. In order for a script to be active, you'll also need to setup a `cloudflare.WorkerRoute`.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   myNamespace:
///     type: cloudflare:WorkersKvNamespace
///     name: my_namespace
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       title: example
///   # Sets the script with the name "script_1"
///   myScript:
///     type: cloudflare:WorkersScript
///     name: my_script
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       name: script_1
///       content:
///         fn::invoke:
///           Function: std:file
///           Arguments:
///             input: script.js
///           Return: result
///       kvNamespaceBindings:
///         - name: MY_EXAMPLE_KV_NAMESPACE
///           namespaceId: ${myNamespace.id}
///       plainTextBindings:
///         - name: MY_EXAMPLE_PLAIN_TEXT
///           text: foobar
///       secretTextBindings:
///         - name: MY_EXAMPLE_SECRET_TEXT
///           text: ${secretFooValue}
///       webassemblyBindings:
///         - name: MY_EXAMPLE_WASM
///           module:
///             fn::invoke:
///               Function: std:filebase64
///               Arguments:
///                 input: example.wasm
///               Return: result
///       serviceBindings:
///         - name: MY_SERVICE_BINDING
///           service: MY_SERVICE
///           environment: production
///       r2BucketBindings:
///         - name: MY_BUCKET
///           bucketName: MY_BUCKET_NAME
///       analyticsEngineBindings:
///         - name: MY_DATASET
///           dataset: dataset1
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/workersScript:WorkersScript example <account_id>/<script_name>
/// ```
///
pub mod workers_script {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkersScriptArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub analytics_engine_bindings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::WorkersScriptAnalyticsEngineBinding>>,
        >,
        /// The date to use for the compatibility flag.
        #[builder(into, default)]
        pub compatibility_date: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Compatibility flags used for Worker Scripts.
        #[builder(into, default)]
        pub compatibility_flags: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The script content.
        #[builder(into)]
        pub content: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub d1_database_bindings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::WorkersScriptD1DatabaseBinding>>,
        >,
        /// Name of the Workers for Platforms dispatch namespace.
        #[builder(into, default)]
        pub dispatch_namespace: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub hyperdrive_config_bindings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::WorkersScriptHyperdriveConfigBinding>>,
        >,
        #[builder(into, default)]
        pub kv_namespace_bindings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::WorkersScriptKvNamespaceBinding>>,
        >,
        /// Enabling allows Worker events to be sent to a defined Logpush destination.
        #[builder(into, default)]
        pub logpush: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether to upload Worker as a module.
        #[builder(into, default)]
        pub module: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name for the script. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub placements: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::WorkersScriptPlacement>>,
        >,
        #[builder(into, default)]
        pub plain_text_bindings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::WorkersScriptPlainTextBinding>>,
        >,
        #[builder(into, default)]
        pub queue_bindings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::WorkersScriptQueueBinding>>,
        >,
        #[builder(into, default)]
        pub r2_bucket_bindings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::WorkersScriptR2BucketBinding>>,
        >,
        #[builder(into, default)]
        pub secret_text_bindings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::WorkersScriptSecretTextBinding>>,
        >,
        #[builder(into, default)]
        pub service_bindings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::WorkersScriptServiceBinding>>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        #[builder(into, default)]
        pub webassembly_bindings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::WorkersScriptWebassemblyBinding>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkersScriptResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub analytics_engine_bindings: pulumi_wasm_rust::Output<
            Option<Vec<super::types::WorkersScriptAnalyticsEngineBinding>>,
        >,
        /// The date to use for the compatibility flag.
        pub compatibility_date: pulumi_wasm_rust::Output<Option<String>>,
        /// Compatibility flags used for Worker Scripts.
        pub compatibility_flags: pulumi_wasm_rust::Output<Vec<String>>,
        /// The script content.
        pub content: pulumi_wasm_rust::Output<String>,
        pub d1_database_bindings: pulumi_wasm_rust::Output<
            Option<Vec<super::types::WorkersScriptD1DatabaseBinding>>,
        >,
        /// Name of the Workers for Platforms dispatch namespace.
        pub dispatch_namespace: pulumi_wasm_rust::Output<Option<String>>,
        pub hyperdrive_config_bindings: pulumi_wasm_rust::Output<
            Option<Vec<super::types::WorkersScriptHyperdriveConfigBinding>>,
        >,
        pub kv_namespace_bindings: pulumi_wasm_rust::Output<
            Option<Vec<super::types::WorkersScriptKvNamespaceBinding>>,
        >,
        /// Enabling allows Worker events to be sent to a defined Logpush destination.
        pub logpush: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to upload Worker as a module.
        pub module: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name for the script. **Modifying this attribute will force creation of a new resource.**
        pub name: pulumi_wasm_rust::Output<String>,
        pub placements: pulumi_wasm_rust::Output<
            Option<Vec<super::types::WorkersScriptPlacement>>,
        >,
        pub plain_text_bindings: pulumi_wasm_rust::Output<
            Option<Vec<super::types::WorkersScriptPlainTextBinding>>,
        >,
        pub queue_bindings: pulumi_wasm_rust::Output<
            Option<Vec<super::types::WorkersScriptQueueBinding>>,
        >,
        pub r2_bucket_bindings: pulumi_wasm_rust::Output<
            Option<Vec<super::types::WorkersScriptR2BucketBinding>>,
        >,
        pub secret_text_bindings: pulumi_wasm_rust::Output<
            Option<Vec<super::types::WorkersScriptSecretTextBinding>>,
        >,
        pub service_bindings: pulumi_wasm_rust::Output<
            Option<Vec<super::types::WorkersScriptServiceBinding>>,
        >,
        pub tags: pulumi_wasm_rust::Output<Vec<String>>,
        pub webassembly_bindings: pulumi_wasm_rust::Output<
            Option<Vec<super::types::WorkersScriptWebassemblyBinding>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: WorkersScriptArgs,
    ) -> WorkersScriptResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let analytics_engine_bindings_binding = args
            .analytics_engine_bindings
            .get_output(context)
            .get_inner();
        let compatibility_date_binding = args
            .compatibility_date
            .get_output(context)
            .get_inner();
        let compatibility_flags_binding = args
            .compatibility_flags
            .get_output(context)
            .get_inner();
        let content_binding = args.content.get_output(context).get_inner();
        let d1_database_bindings_binding = args
            .d1_database_bindings
            .get_output(context)
            .get_inner();
        let dispatch_namespace_binding = args
            .dispatch_namespace
            .get_output(context)
            .get_inner();
        let hyperdrive_config_bindings_binding = args
            .hyperdrive_config_bindings
            .get_output(context)
            .get_inner();
        let kv_namespace_bindings_binding = args
            .kv_namespace_bindings
            .get_output(context)
            .get_inner();
        let logpush_binding = args.logpush.get_output(context).get_inner();
        let module_binding = args.module.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let placements_binding = args.placements.get_output(context).get_inner();
        let plain_text_bindings_binding = args
            .plain_text_bindings
            .get_output(context)
            .get_inner();
        let queue_bindings_binding = args.queue_bindings.get_output(context).get_inner();
        let r2_bucket_bindings_binding = args
            .r2_bucket_bindings
            .get_output(context)
            .get_inner();
        let secret_text_bindings_binding = args
            .secret_text_bindings
            .get_output(context)
            .get_inner();
        let service_bindings_binding = args
            .service_bindings
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let webassembly_bindings_binding = args
            .webassembly_bindings
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/workersScript:WorkersScript".into(),
            name: name.to_string(),
            version: super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkersScriptResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            analytics_engine_bindings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("analyticsEngineBindings"),
            ),
            compatibility_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("compatibilityDate"),
            ),
            compatibility_flags: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("compatibilityFlags"),
            ),
            content: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            d1_database_bindings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("d1DatabaseBindings"),
            ),
            dispatch_namespace: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dispatchNamespace"),
            ),
            hyperdrive_config_bindings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hyperdriveConfigBindings"),
            ),
            kv_namespace_bindings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kvNamespaceBindings"),
            ),
            logpush: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logpush"),
            ),
            module: pulumi_wasm_rust::__private::into_domain(o.extract_field("module")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            placements: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("placements"),
            ),
            plain_text_bindings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("plainTextBindings"),
            ),
            queue_bindings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("queueBindings"),
            ),
            r2_bucket_bindings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("r2BucketBindings"),
            ),
            secret_text_bindings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secretTextBindings"),
            ),
            service_bindings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceBindings"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            webassembly_bindings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("webassemblyBindings"),
            ),
        }
    }
}
