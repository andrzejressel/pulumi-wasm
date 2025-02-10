#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_instance_storage_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceStorageConfigArgs {
        /// The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.
        #[builder(into)]
        pub association_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A valid resource type. Valid Values: `AGENT_EVENTS` | `ATTACHMENTS` | `CALL_RECORDINGS` | `CHAT_TRANSCRIPTS` | `CONTACT_EVALUATIONS` | `CONTACT_TRACE_RECORDS` | `MEDIA_STREAMS` | `REAL_TIME_CONTACT_ANALYSIS_SEGMENTS` | `SCHEDULED_REPORTS` |  `SCREEN_RECORDINGS`.
        #[builder(into)]
        pub resource_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceStorageConfigResult {
        pub association_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the storage configuration options for the Connect Instance. Documented below.
        pub storage_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::connect::GetInstanceStorageConfigStorageConfig,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetInstanceStorageConfigArgs,
    ) -> GetInstanceStorageConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let association_id_binding = args.association_id.get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let resource_type_binding = args.resource_type.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:connect/getInstanceStorageConfig:getInstanceStorageConfig"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "associationId".into(),
                    value: association_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceType".into(),
                    value: resource_type_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInstanceStorageConfigResult {
            association_id: o.get_field("associationId"),
            id: o.get_field("id"),
            instance_id: o.get_field("instanceId"),
            resource_type: o.get_field("resourceType"),
            storage_configs: o.get_field("storageConfigs"),
        }
    }
}
