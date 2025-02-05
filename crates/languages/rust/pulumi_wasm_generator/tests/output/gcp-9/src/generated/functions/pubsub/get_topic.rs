pub mod get_topic {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTopicArgs {
        /// The name of the Cloud Pub/Sub Topic.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetTopicResult {
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub ingestion_data_source_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::pubsub::GetTopicIngestionDataSourceSetting>,
        >,
        pub kms_key_name: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub message_retention_duration: pulumi_wasm_rust::Output<String>,
        pub message_storage_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::pubsub::GetTopicMessageStoragePolicy>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub schema_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::pubsub::GetTopicSchemaSetting>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTopicArgs,
    ) -> GetTopicResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:pubsub/getTopic:getTopic".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTopicResult {
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ingestion_data_source_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ingestionDataSourceSettings"),
            ),
            kms_key_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyName"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            message_retention_duration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("messageRetentionDuration"),
            ),
            message_storage_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("messageStoragePolicies"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            schema_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("schemaSettings"),
            ),
        }
    }
}
