/// Manages a Virtual Network within a DevTest Lab.
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
///   exampleLab:
///     type: azure:devtest:Lab
///     name: example
///     properties:
///       name: example-devtestlab
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tags:
///         Sydney: Australia
///   exampleVirtualNetwork:
///     type: azure:devtest:VirtualNetwork
///     name: example
///     properties:
///       name: example-network
///       labName: ${exampleLab.name}
///       resourceGroupName: ${example.name}
///       subnet:
///         usePublicIpAddress: Allow
///         useInVirtualMachineCreation: Allow
/// ```
///
/// ## Import
///
/// DevTest Virtual Networks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devtest/virtualNetwork:VirtualNetwork network1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DevTestLab/labs/lab1/virtualNetworks/network1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkArgs {
        /// A description for the Virtual Network.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Dev Test Lab in which the Virtual Network should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub lab_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Dev Test Virtual Network. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Dev Test Lab resource exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `subnet` block as defined below.
        #[builder(into, default)]
        pub subnet: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::devtest::VirtualNetworkSubnet>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkResult {
        /// A description for the Virtual Network.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Dev Test Lab in which the Virtual Network should be created. Changing this forces a new resource to be created.
        pub lab_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Dev Test Virtual Network. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the Dev Test Lab resource exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `subnet` block as defined below.
        pub subnet: pulumi_gestalt_rust::Output<
            super::super::types::devtest::VirtualNetworkSubnet,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The unique immutable identifier of the Dev Test Virtual Network.
        pub unique_identifier: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualNetworkArgs,
    ) -> VirtualNetworkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let lab_name_binding = args.lab_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let subnet_binding = args.subnet.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:devtest/virtualNetwork:VirtualNetwork".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labName".into(),
                    value: lab_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnet".into(),
                    value: subnet_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualNetworkResult {
            description: o.get_field("description"),
            lab_name: o.get_field("labName"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            subnet: o.get_field("subnet"),
            tags: o.get_field("tags"),
            unique_identifier: o.get_field("uniqueIdentifier"),
        }
    }
}
