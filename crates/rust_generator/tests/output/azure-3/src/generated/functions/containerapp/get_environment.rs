#[allow(clippy::doc_lazy_continuation)]
pub mod get_environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEnvironmentArgs {
        /// The name of the Container Apps Managed Environment.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where this Container App Environment exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetEnvironmentResult {
        /// The ID of the Custom Domain Verification for this Container App Environment.
        pub custom_domain_verification_id: pulumi_gestalt_rust::Output<String>,
        /// The default publicly resolvable name of this Container App Environment. This is generated at creation time to be globally unique.
        pub default_domain: pulumi_gestalt_rust::Output<String>,
        /// The network addressing in which the Container Apps in this Container App Environment will reside in CIDR notation.
        pub docker_bridge_cidr: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subnet in use by the Container Apps Control Plane.
        pub infrastructure_subnet_id: pulumi_gestalt_rust::Output<String>,
        /// Does the Container App Environment operate in Internal Load Balancing Mode?
        pub internal_load_balancer_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Azure Location where this Container App Environment exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Log Analytics Workspace this Container Apps Managed Environment is linked to.
        pub log_analytics_workspace_name: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The IP range, in CIDR notation, that is reserved for environment infrastructure IP addresses.
        pub platform_reserved_cidr: pulumi_gestalt_rust::Output<String>,
        /// The IP address from the IP range defined by `platform_reserved_cidr` that is reserved for the internal DNS server.
        pub platform_reserved_dns_ip_address: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Static IP address of the Environment.
        pub static_ip_address: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetEnvironmentArgs,
    ) -> GetEnvironmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:containerapp/getEnvironment:getEnvironment".into(),
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
        GetEnvironmentResult {
            custom_domain_verification_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customDomainVerificationId"),
            ),
            default_domain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultDomain"),
            ),
            docker_bridge_cidr: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dockerBridgeCidr"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            infrastructure_subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("infrastructureSubnetId"),
            ),
            internal_load_balancer_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("internalLoadBalancerEnabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            log_analytics_workspace_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logAnalyticsWorkspaceName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            platform_reserved_cidr: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platformReservedCidr"),
            ),
            platform_reserved_dns_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platformReservedDnsIpAddress"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            static_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("staticIpAddress"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
