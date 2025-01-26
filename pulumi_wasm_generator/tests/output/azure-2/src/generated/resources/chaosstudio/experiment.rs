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
pub mod experiment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExperimentArgs {
        /// A `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::chaosstudio::ExperimentIdentity>,
        >,
        /// The Azure Region where the Chaos Studio Experiment should exist. Changing this forces a new Chaos Studio Experiment to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Chaos Studio Experiment. Changing this forces a new Chaos Studio Experiment to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Chaos Studio Experiment should exist. Changing this forces a new Chaos Studio Experiment to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// One or more `selectors` blocks as defined below.
        #[builder(into)]
        pub selectors: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::chaosstudio::ExperimentSelector>,
        >,
        /// One or more `steps` blocks as defined below.
        #[builder(into)]
        pub steps: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::chaosstudio::ExperimentStep>,
        >,
    }
    #[allow(dead_code)]
    pub struct ExperimentResult {
        /// A `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::chaosstudio::ExperimentIdentity>,
        >,
        /// The Azure Region where the Chaos Studio Experiment should exist. Changing this forces a new Chaos Studio Experiment to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Chaos Studio Experiment. Changing this forces a new Chaos Studio Experiment to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Chaos Studio Experiment should exist. Changing this forces a new Chaos Studio Experiment to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `selectors` blocks as defined below.
        pub selectors: pulumi_wasm_rust::Output<
            Vec<super::super::types::chaosstudio::ExperimentSelector>,
        >,
        /// One or more `steps` blocks as defined below.
        pub steps: pulumi_wasm_rust::Output<
            Vec<super::super::types::chaosstudio::ExperimentStep>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ExperimentArgs,
    ) -> ExperimentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let selectors_binding = args.selectors.get_output(context).get_inner();
        let steps_binding = args.steps.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:chaosstudio/experiment:Experiment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "selectors".into(),
                    value: &selectors_binding,
                },
                register_interface::ObjectField {
                    name: "steps".into(),
                    value: &steps_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "selectors".into(),
                },
                register_interface::ResultField {
                    name: "steps".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ExperimentResult {
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            selectors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selectors").unwrap(),
            ),
            steps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("steps").unwrap(),
            ),
        }
    }
}
