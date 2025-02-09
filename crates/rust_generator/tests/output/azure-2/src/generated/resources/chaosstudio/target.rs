/// <!-- Note: This documentation is generated. Any manual changes will be overwritten -->
///
/// Manages a Chaos Studio Target.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:containerservice:KubernetesCluster
///     properties:
///       name: example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       dnsPrefix: acctestaksexample
///       defaultNodePool:
///         name: example-value
///         nodeCount: example-value
///         vmSize: example-value
///         upgradeSettings:
///           maxSurge: example-value
///       identity:
///         type: example-value
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleTarget:
///     type: azure:chaosstudio:Target
///     name: example
///     properties:
///       location: ${exampleResourceGroup.location}
///       targetResourceId: ${example.id}
///       targetType: example-value
/// ```
///
/// ## Import
///
/// An existing Chaos Studio Target can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:chaosstudio/target:Target example /{scope}/providers/Microsoft.Chaos/targets/{targetName}
/// ```
///
/// * Where `{scope}` is the ID of the Azure Resource under which the Chaos Studio Target exists. For example `/subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/some-resource-group`.
///
/// * Where `{targetName}` is the name of the Target. For example `targetValue`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetArgs {
        /// The Azure Region where the Chaos Studio Target should exist. Changing this forces a new Chaos Studio Target to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Target Resource Id within which this Chaos Studio Target should exist. Changing this forces a new Chaos Studio Target to be created.
        #[builder(into)]
        pub target_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Chaos Studio Target. This has the format of [publisher]-[targetType] e.g. `Microsoft-StorageAccount`. For supported values please see this Target Type column in [this table](https://learn.microsoft.com/azure/chaos-studio/chaos-studio-fault-providers). Changing this forces a new Chaos Studio Target to be created.
        #[builder(into)]
        pub target_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TargetResult {
        /// The Azure Region where the Chaos Studio Target should exist. Changing this forces a new Chaos Studio Target to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Target Resource Id within which this Chaos Studio Target should exist. Changing this forces a new Chaos Studio Target to be created.
        pub target_resource_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Chaos Studio Target. This has the format of [publisher]-[targetType] e.g. `Microsoft-StorageAccount`. For supported values please see this Target Type column in [this table](https://learn.microsoft.com/azure/chaos-studio/chaos-studio-fault-providers). Changing this forces a new Chaos Studio Target to be created.
        pub target_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TargetArgs,
    ) -> TargetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let target_resource_id_binding = args.target_resource_id.get_output(context);
        let target_type_binding = args.target_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:chaosstudio/target:Target".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetResourceId".into(),
                    value: target_resource_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetType".into(),
                    value: target_type_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TargetResult {
            location: o.get_field("location"),
            target_resource_id: o.get_field("targetResourceId"),
            target_type: o.get_field("targetType"),
        }
    }
}
