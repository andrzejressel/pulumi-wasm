#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_subnetwork {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubnetworkArgs {
        /// The name of the subnetwork. One of `name` or `self_link`
        /// must be specified.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region this subnetwork has been created in. If
        /// unspecified, this defaults to the region configured in the provider.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The self link of the subnetwork. If `self_link` is
        /// specified, `name`, `project`, and `region` are ignored.
        #[builder(into, default)]
        pub self_link: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSubnetworkResult {
        /// Description of this subnetwork.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The IP address of the gateway.
        pub gateway_address: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The internal IPv6 address range that is assigned to this subnetwork.
        pub internal_ipv6_prefix: pulumi_gestalt_rust::Output<String>,
        /// The range of IP addresses belonging to this subnetwork
        /// secondary range.
        pub ip_cidr_range: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The network name or resource link to the parent
        /// network of this subnetwork.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// Whether the VMs in this subnet
        /// can access Google services without assigned external IP
        /// addresses.
        pub private_ip_google_access: pulumi_gestalt_rust::Output<bool>,
        pub project: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<String>,
        /// An array of configurations for secondary IP ranges for
        /// VM instances contained in this subnetwork. Structure is documented below.
        pub secondary_ip_ranges: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetSubnetworkSecondaryIpRange>,
        >,
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSubnetworkArgs,
    ) -> GetSubnetworkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let self_link_binding = args.self_link.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getSubnetwork:getSubnetwork".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selfLink".into(),
                    value: &self_link_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSubnetworkResult {
            description: o.get_field("description"),
            gateway_address: o.get_field("gatewayAddress"),
            id: o.get_field("id"),
            internal_ipv6_prefix: o.get_field("internalIpv6Prefix"),
            ip_cidr_range: o.get_field("ipCidrRange"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            private_ip_google_access: o.get_field("privateIpGoogleAccess"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            secondary_ip_ranges: o.get_field("secondaryIpRanges"),
            self_link: o.get_field("selfLink"),
        }
    }
}
