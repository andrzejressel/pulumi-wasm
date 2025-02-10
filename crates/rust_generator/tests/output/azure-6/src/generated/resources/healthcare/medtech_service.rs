/// Manages a Healthcare Med Tech Service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: east us
///   exampleWorkspace:
///     type: azure:healthcare:Workspace
///     name: example
///     properties:
///       name: examplewkspace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleMedtechService:
///     type: azure:healthcare:MedtechService
///     name: example
///     properties:
///       name: examplemed
///       workspaceId: ${exampleWorkspace.id}
///       location: east us
///       identity:
///         type: SystemAssigned
///       eventhubNamespaceName: example-eventhub-namespace
///       eventhubName: example-eventhub
///       eventhubConsumerGroupName: $Default
///       deviceMappingJson:
///         fn::toJSON:
///           templateType: CollectionContent
///           template:
///             - templateType: JsonPathContent
///               template:
///                 typeName: heartrate
///                 typeMatchExpression: $..[?(@heartrate)]
///                 deviceIdExpression: $.deviceid
///                 timestampExpression: $.measurementdatetime
///                 values:
///                   - required: 'true'
///                     valueExpression: $.heartrate
///                     valueName: hr
/// ```
///
/// ## Import
///
/// Healthcare Med Tech Service can be imported using the resource`id`, e.g.
///
/// ```sh
/// $ pulumi import azure:healthcare/medtechService:MedtechService example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.HealthcareApis/workspaces/workspace1/iotConnectors/iotconnector1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod medtech_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MedtechServiceArgs {
        /// Specifies the Device Mappings of the Med Tech Service.
        #[builder(into)]
        pub device_mapping_json: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Consumer Group of the Event Hub to connect to.
        #[builder(into)]
        pub eventhub_consumer_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Event Hub to connect to.
        #[builder(into)]
        pub eventhub_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the namespace name of the Event Hub to connect to.
        #[builder(into)]
        pub eventhub_namespace_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::healthcare::MedtechServiceIdentity>,
        >,
        /// Specifies the Azure Region where the Healthcare Med Tech Service should be created. Changing this forces a new Healthcare Med Tech Service to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Healthcare Med Tech Service. Changing this forces a new Healthcare Med Tech Service to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the Healthcare Med Tech Service.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the id of the Healthcare Workspace where the Healthcare Med Tech Service should exist. Changing this forces a new Healthcare Med Tech Service to be created.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MedtechServiceResult {
        /// Specifies the Device Mappings of the Med Tech Service.
        pub device_mapping_json: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Consumer Group of the Event Hub to connect to.
        pub eventhub_consumer_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Event Hub to connect to.
        pub eventhub_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the namespace name of the Event Hub to connect to.
        pub eventhub_namespace_name: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::healthcare::MedtechServiceIdentity>,
        >,
        /// Specifies the Azure Region where the Healthcare Med Tech Service should be created. Changing this forces a new Healthcare Med Tech Service to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Healthcare Med Tech Service. Changing this forces a new Healthcare Med Tech Service to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the Healthcare Med Tech Service.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the id of the Healthcare Workspace where the Healthcare Med Tech Service should exist. Changing this forces a new Healthcare Med Tech Service to be created.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MedtechServiceArgs,
    ) -> MedtechServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let device_mapping_json_binding = args.device_mapping_json.get_output(context);
        let eventhub_consumer_group_name_binding = args
            .eventhub_consumer_group_name
            .get_output(context);
        let eventhub_name_binding = args.eventhub_name.get_output(context);
        let eventhub_namespace_name_binding = args
            .eventhub_namespace_name
            .get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:healthcare/medtechService:MedtechService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceMappingJson".into(),
                    value: device_mapping_json_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubConsumerGroupName".into(),
                    value: eventhub_consumer_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubName".into(),
                    value: eventhub_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubNamespaceName".into(),
                    value: eventhub_namespace_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MedtechServiceResult {
            device_mapping_json: o.get_field("deviceMappingJson"),
            eventhub_consumer_group_name: o.get_field("eventhubConsumerGroupName"),
            eventhub_name: o.get_field("eventhubName"),
            eventhub_namespace_name: o.get_field("eventhubNamespaceName"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
