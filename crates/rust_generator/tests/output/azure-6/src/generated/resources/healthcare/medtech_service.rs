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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MedtechServiceArgs,
    ) -> MedtechServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let device_mapping_json_binding = args
            .device_mapping_json
            .get_output(context)
            .get_inner();
        let eventhub_consumer_group_name_binding = args
            .eventhub_consumer_group_name
            .get_output(context)
            .get_inner();
        let eventhub_name_binding = args.eventhub_name.get_output(context).get_inner();
        let eventhub_namespace_name_binding = args
            .eventhub_namespace_name
            .get_output(context)
            .get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let workspace_id_binding = args.workspace_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:healthcare/medtechService:MedtechService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deviceMappingJson".into(),
                    value: &device_mapping_json_binding,
                },
                register_interface::ObjectField {
                    name: "eventhubConsumerGroupName".into(),
                    value: &eventhub_consumer_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "eventhubName".into(),
                    value: &eventhub_name_binding,
                },
                register_interface::ObjectField {
                    name: "eventhubNamespaceName".into(),
                    value: &eventhub_namespace_name_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MedtechServiceResult {
            device_mapping_json: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deviceMappingJson"),
            ),
            eventhub_consumer_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventhubConsumerGroupName"),
            ),
            eventhub_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventhubName"),
            ),
            eventhub_namespace_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventhubNamespaceName"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceId"),
            ),
        }
    }
}
