#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_virtual_network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualNetworkArgs {
        /// Specifies the name of the Dev Test Lab.
        #[builder(into)]
        pub lab_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Virtual Network.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group that contains the Virtual Network.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualNetworkResult {
        /// The list of subnets enabled for the virtual network as defined below.
        pub allowed_subnets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::devtest::GetVirtualNetworkAllowedSubnet>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub lab_name: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The list of permission overrides for the subnets as defined below.
        pub subnet_overrides: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::devtest::GetVirtualNetworkSubnetOverride>,
        >,
        /// The unique immutable identifier of the virtual network.
        pub unique_identifier: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVirtualNetworkArgs,
    ) -> GetVirtualNetworkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let lab_name_binding_1 = args.lab_name.get_output(context);
        let lab_name_binding = lab_name_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:devtest/getVirtualNetwork:getVirtualNetwork".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "labName".into(),
                    value: &lab_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVirtualNetworkResult {
            allowed_subnets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowedSubnets"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            lab_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            subnet_overrides: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetOverrides"),
            ),
            unique_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("uniqueIdentifier"),
            ),
        }
    }
}
