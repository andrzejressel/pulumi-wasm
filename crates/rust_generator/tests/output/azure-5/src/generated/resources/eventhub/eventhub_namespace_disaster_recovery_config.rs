/// Manages an Disaster Recovery Config for an Event Hub Namespace.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("eventhub-replication")
///             .build_struct(),
///     );
///     let exampleEventhubNamespaceDisasterRecoveryConfig = eventhub_namespace_disaster_recovery_config::create(
///         "exampleEventhubNamespaceDisasterRecoveryConfig",
///         EventhubNamespaceDisasterRecoveryConfigArgs::builder()
///             .name("replicate-eventhub")
///             .namespace_name("${primary.name}")
///             .partner_namespace_id("${secondary.id}")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let primary = event_hub_namespace::create(
///         "primary",
///         EventHubNamespaceArgs::builder()
///             .location("${example.location}")
///             .name("eventhub-primary")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let secondary = event_hub_namespace::create(
///         "secondary",
///         EventHubNamespaceArgs::builder()
///             .location("${example.location}")
///             .name("eventhub-secondary")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// EventHubs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventhub/eventhubNamespaceDisasterRecoveryConfig:EventhubNamespaceDisasterRecoveryConfig config1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1/disasterRecoveryConfigs/config1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod eventhub_namespace_disaster_recovery_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventhubNamespaceDisasterRecoveryConfigArgs {
        /// Specifies the name of the Disaster Recovery Config. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the primary EventHub Namespace to replicate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the EventHub Namespace to replicate to.
        #[builder(into)]
        pub partner_namespace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which the Disaster Recovery Config exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EventhubNamespaceDisasterRecoveryConfigResult {
        /// Specifies the name of the Disaster Recovery Config. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the primary EventHub Namespace to replicate. Changing this forces a new resource to be created.
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the EventHub Namespace to replicate to.
        pub partner_namespace_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the Disaster Recovery Config exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventhubNamespaceDisasterRecoveryConfigArgs,
    ) -> EventhubNamespaceDisasterRecoveryConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let namespace_name_binding = args.namespace_name.get_output(context);
        let partner_namespace_id_binding = args.partner_namespace_id.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:eventhub/eventhubNamespaceDisasterRecoveryConfig:EventhubNamespaceDisasterRecoveryConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceName".into(),
                    value: namespace_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partnerNamespaceId".into(),
                    value: partner_namespace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventhubNamespaceDisasterRecoveryConfigResult {
            name: o.get_field("name"),
            namespace_name: o.get_field("namespaceName"),
            partner_namespace_id: o.get_field("partnerNamespaceId"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
