/// Manages a Event Hubs Consumer Group as a nested resource within an Event Hub.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleEventHubNamespace:
///     type: azure:eventhub:EventHubNamespace
///     name: example
///     properties:
///       name: acceptanceTestEventHubNamespace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Basic
///       capacity: 2
///       tags:
///         environment: Production
///   exampleEventHub:
///     type: azure:eventhub:EventHub
///     name: example
///     properties:
///       name: acceptanceTestEventHub
///       namespaceName: ${exampleEventHubNamespace.name}
///       resourceGroupName: ${example.name}
///       partitionCount: 2
///       messageRetention: 2
///   exampleConsumerGroup:
///     type: azure:eventhub:ConsumerGroup
///     name: example
///     properties:
///       name: acceptanceTestEventHubConsumerGroup
///       namespaceName: ${exampleEventHubNamespace.name}
///       eventhubName: ${exampleEventHub.name}
///       resourceGroupName: ${example.name}
///       userMetadata: some-meta-data
/// ```
///
/// ## Import
///
/// EventHub Consumer Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventhub/consumerGroup:ConsumerGroup consumerGroup1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1/eventhubs/eventhub1/consumerGroups/consumerGroup1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod consumer_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConsumerGroupArgs {
        /// Specifies the name of the EventHub. Changing this forces a new resource to be created.
        #[builder(into)]
        pub eventhub_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the EventHub Consumer Group resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the grandparent EventHub Namespace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which the EventHub Consumer Group's grandparent Namespace exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the user metadata.
        #[builder(into, default)]
        pub user_metadata: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConsumerGroupResult {
        /// Specifies the name of the EventHub. Changing this forces a new resource to be created.
        pub eventhub_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the EventHub Consumer Group resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the grandparent EventHub Namespace. Changing this forces a new resource to be created.
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the EventHub Consumer Group's grandparent Namespace exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the user metadata.
        pub user_metadata: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConsumerGroupArgs,
    ) -> ConsumerGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let eventhub_name_binding = args.eventhub_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let namespace_name_binding = args.namespace_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let user_metadata_binding = args.user_metadata.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:eventhub/consumerGroup:ConsumerGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubName".into(),
                    value: eventhub_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceName".into(),
                    value: namespace_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userMetadata".into(),
                    value: user_metadata_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConsumerGroupResult {
            eventhub_name: o.get_field("eventhubName"),
            name: o.get_field("name"),
            namespace_name: o.get_field("namespaceName"),
            resource_group_name: o.get_field("resourceGroupName"),
            user_metadata: o.get_field("userMetadata"),
        }
    }
}
