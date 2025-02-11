#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetEnvironmentArgs,
    ) -> GetEnvironmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:containerapp/getEnvironment:getEnvironment".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetEnvironmentResult {
            custom_domain_verification_id: o.get_field("customDomainVerificationId"),
            default_domain: o.get_field("defaultDomain"),
            docker_bridge_cidr: o.get_field("dockerBridgeCidr"),
            id: o.get_field("id"),
            infrastructure_subnet_id: o.get_field("infrastructureSubnetId"),
            internal_load_balancer_enabled: o.get_field("internalLoadBalancerEnabled"),
            location: o.get_field("location"),
            log_analytics_workspace_name: o.get_field("logAnalyticsWorkspaceName"),
            name: o.get_field("name"),
            platform_reserved_cidr: o.get_field("platformReservedCidr"),
            platform_reserved_dns_ip_address: o
                .get_field("platformReservedDnsIpAddress"),
            resource_group_name: o.get_field("resourceGroupName"),
            static_ip_address: o.get_field("staticIpAddress"),
            tags: o.get_field("tags"),
        }
    }
}
