/// Manages an Arc Machine Automanage Configuration Profile Assignment.
///
/// ## Example Usage
///
/// ```yaml
/// configuration:
///   arcMachineName:
///     type: dynamic
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleConfiguration:
///     type: azure:automanage:Configuration
///     name: example
///     properties:
///       name: example-configuration
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///   exampleAutomanageConfigurationAssignment:
///     type: azure:arcmachine:AutomanageConfigurationAssignment
///     name: example
///     properties:
///       arcMachineId: ${example.id}
///       configurationId: ${exampleConfiguration.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:arcmachine:get
///       arguments:
///         name: ${arcMachineName}
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// Virtual Machine Automanage Configuration Profile Assignment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:arcmachine/automanageConfigurationAssignment:AutomanageConfigurationAssignment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.HybridCompute/machines/machine1/providers/Microsoft.AutoManage/configurationProfileAssignments/default
/// ```
///
pub mod automanage_configuration_assignment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutomanageConfigurationAssignmentArgs {
        /// The ARM resource ID of the Arc Machine to assign the Automanage Configuration to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub arc_machine_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ARM resource ID of the Automanage Configuration to assign to the Virtual Machine. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** For a successful creation of this resource, locate "Automanage API Access" app within your Entra ID tenant. Make sure it's granted access to the scope that includes the arc server.
        #[builder(into)]
        pub configuration_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AutomanageConfigurationAssignmentResult {
        /// The ARM resource ID of the Arc Machine to assign the Automanage Configuration to. Changing this forces a new resource to be created.
        pub arc_machine_id: pulumi_wasm_rust::Output<String>,
        /// The ARM resource ID of the Automanage Configuration to assign to the Virtual Machine. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** For a successful creation of this resource, locate "Automanage API Access" app within your Entra ID tenant. Make sure it's granted access to the scope that includes the arc server.
        pub configuration_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AutomanageConfigurationAssignmentArgs,
    ) -> AutomanageConfigurationAssignmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arc_machine_id_binding = args.arc_machine_id.get_output(context).get_inner();
        let configuration_id_binding = args
            .configuration_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:arcmachine/automanageConfigurationAssignment:AutomanageConfigurationAssignment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arcMachineId".into(),
                    value: &arc_machine_id_binding,
                },
                register_interface::ObjectField {
                    name: "configurationId".into(),
                    value: &configuration_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AutomanageConfigurationAssignmentResult {
            arc_machine_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("arcMachineId"),
            ),
            configuration_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configurationId"),
            ),
        }
    }
}
