pub mod get_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// The name of the private link service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the resource group in which the private link service resides.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        /// The alias is a globally unique name for your private link service which Azure generates for you. Your can use this alias to request a connection to your private link service.
        pub alias: pulumi_wasm_rust::Output<String>,
        /// The list of subscription(s) globally unique identifiers that will be auto approved to use the private link service.
        pub auto_approval_subscription_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Does the Private Link Service support the Proxy Protocol?
        pub enable_proxy_protocol: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The list of Standard Load Balancer(SLB) resource IDs. The Private Link service is tied to the frontend IP address of a SLB. All traffic destined for the private link service will reach the frontend of the SLB. You can configure SLB rules to direct this traffic to appropriate backend pools where your applications are running.
        pub load_balancer_frontend_ip_configuration_ids: pulumi_wasm_rust::Output<
            Vec<String>,
        >,
        /// The supported Azure location where the resource exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of private link service NAT IP configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The `nat_ip_configuration` block as defined below.
        pub nat_ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::privatelink::GetServiceNatIpConfiguration>,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The list of subscription(s) globally unique identifiers(GUID) that will be able to see the private link service.
        pub visibility_subscription_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:privatelink/getService:getService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
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
        GetServiceResult {
            alias: pulumi_wasm_rust::__private::into_domain(o.extract_field("alias")),
            auto_approval_subscription_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoApprovalSubscriptionIds"),
            ),
            enable_proxy_protocol: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableProxyProtocol"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            load_balancer_frontend_ip_configuration_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loadBalancerFrontendIpConfigurationIds"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            nat_ip_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("natIpConfigurations"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            visibility_subscription_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("visibilitySubscriptionIds"),
            ),
        }
    }
}
