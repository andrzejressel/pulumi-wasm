/// Manages a Chaos Studio Experiment.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example
///       location: westeurope
///   exampleUserAssignedIdentity:
///     type: azure:authorization:UserAssignedIdentity
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       name: example
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: internal
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleNetworkInterface:
///     type: azure:network:NetworkInterface
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       ipConfigurations:
///         - name: example
///           subnetId: ${exampleSubnet.id}
///           privateIpAddressAllocation: Dynamic
///   exampleLinuxVirtualMachine:
///     type: azure:compute:LinuxVirtualMachine
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       size: Standard_F2
///       adminUsername: adminuser
///       adminPassword: example
///       disablePasswordAuthentication: false
///       networkInterfaceIds:
///         - ${exampleNetworkInterface.id}
///       osDisk:
///         caching: ReadWrite
///         storageAccountType: Standard_LRS
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///   exampleTarget:
///     type: azure:chaosstudio:Target
///     name: example
///     properties:
///       location: ${example.location}
///       targetResourceId: ${exampleLinuxVirtualMachine.id}
///       targetType: Microsoft-VirtualMachine
///   exampleCapability:
///     type: azure:chaosstudio:Capability
///     name: example
///     properties:
///       chaosStudioTargetId: ${exampleTarget.id}
///       capabilityType: Shutdown-1.0
///   exampleExperiment:
///     type: azure:chaosstudio:Experiment
///     name: example
///     properties:
///       location: ${example.location}
///       name: example
///       resourceGroupName: ${example.name}
///       identity:
///         type: SystemAssigned
///       selectors:
///         - name: Selector1
///           chaosStudioTargetIds:
///             - ${exampleTarget.id}
///       steps:
///         - name: example
///           branches:
///             - name: example
///               actions:
///                 - urn: ${exampleCapability.capabilityUrn}
///                   selectorName: Selector1
///                   parameters:
///                     abruptShutdown: 'false'
///                   actionType: continuous
///                   duration: PT10M
/// ```
///
/// ## Import
///
/// Chaos Studio Experiments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:chaosstudio/experiment:Experiment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Chaos/experiments/experiment1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod experiment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExperimentArgs {
        /// A `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::chaosstudio::ExperimentIdentity>,
        >,
        /// The Azure Region where the Chaos Studio Experiment should exist. Changing this forces a new Chaos Studio Experiment to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Chaos Studio Experiment. Changing this forces a new Chaos Studio Experiment to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Chaos Studio Experiment should exist. Changing this forces a new Chaos Studio Experiment to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `selectors` blocks as defined below.
        #[builder(into)]
        pub selectors: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::chaosstudio::ExperimentSelector>,
        >,
        /// One or more `steps` blocks as defined below.
        #[builder(into)]
        pub steps: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::chaosstudio::ExperimentStep>,
        >,
    }
    #[allow(dead_code)]
    pub struct ExperimentResult {
        /// A `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::chaosstudio::ExperimentIdentity>,
        >,
        /// The Azure Region where the Chaos Studio Experiment should exist. Changing this forces a new Chaos Studio Experiment to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Chaos Studio Experiment. Changing this forces a new Chaos Studio Experiment to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Chaos Studio Experiment should exist. Changing this forces a new Chaos Studio Experiment to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `selectors` blocks as defined below.
        pub selectors: pulumi_gestalt_rust::Output<
            Vec<super::super::types::chaosstudio::ExperimentSelector>,
        >,
        /// One or more `steps` blocks as defined below.
        pub steps: pulumi_gestalt_rust::Output<
            Vec<super::super::types::chaosstudio::ExperimentStep>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExperimentArgs,
    ) -> ExperimentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let selectors_binding = args.selectors.get_output(context);
        let steps_binding = args.steps.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:chaosstudio/experiment:Experiment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selectors".into(),
                    value: &selectors_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "steps".into(),
                    value: &steps_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExperimentResult {
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            selectors: o.get_field("selectors"),
            steps: o.get_field("steps"),
        }
    }
}
