#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_medtech_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMedtechServiceArgs {
        /// The name of the Healthcare Med Tech Service.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the Healthcare Workspace in which the Healthcare Med Tech Service exists.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetMedtechServiceResult {
        /// The Device Mappings of the Med Tech Service.
        pub device_mapping_json: pulumi_gestalt_rust::Output<String>,
        /// The Consumer Group of the Event Hub of the Healthcare Med Tech Service.
        pub eventhub_consumer_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Event Hub of the Healthcare Med Tech Service.
        pub eventhub_name: pulumi_gestalt_rust::Output<String>,
        /// The namespace name of the Event Hub of the Healthcare Med Tech Service.
        pub eventhub_namespace_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::healthcare::GetMedtechServiceIdentity>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetMedtechServiceArgs,
    ) -> GetMedtechServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:healthcare/getMedtechService:getMedtechService".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetMedtechServiceResult {
            device_mapping_json: o.get_field("deviceMappingJson"),
            eventhub_consumer_group_name: o.get_field("eventhubConsumerGroupName"),
            eventhub_name: o.get_field("eventhubName"),
            eventhub_namespace_name: o.get_field("eventhubNamespaceName"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            name: o.get_field("name"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
