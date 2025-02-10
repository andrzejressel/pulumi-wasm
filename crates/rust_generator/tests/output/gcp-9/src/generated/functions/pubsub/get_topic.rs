#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_topic {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTopicArgs {
        /// The name of the Cloud Pub/Sub Topic.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetTopicResult {
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ingestion_data_source_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::pubsub::GetTopicIngestionDataSourceSetting>,
        >,
        pub kms_key_name: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub message_retention_duration: pulumi_gestalt_rust::Output<String>,
        pub message_storage_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::pubsub::GetTopicMessageStoragePolicy>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub schema_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::pubsub::GetTopicSchemaSetting>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTopicArgs,
    ) -> GetTopicResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:pubsub/getTopic:getTopic".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTopicResult {
            effective_labels: o.get_field("effectiveLabels"),
            id: o.get_field("id"),
            ingestion_data_source_settings: o.get_field("ingestionDataSourceSettings"),
            kms_key_name: o.get_field("kmsKeyName"),
            labels: o.get_field("labels"),
            message_retention_duration: o.get_field("messageRetentionDuration"),
            message_storage_policies: o.get_field("messageStoragePolicies"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            schema_settings: o.get_field("schemaSettings"),
        }
    }
}
