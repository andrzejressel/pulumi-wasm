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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSubnetworkArgs,
    ) -> GetSubnetworkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let region_binding_1 = args.region.get_output(context);
        let region_binding = region_binding_1.get_inner();
        let self_link_binding_1 = args.self_link.get_output(context);
        let self_link_binding = self_link_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getSubnetwork:getSubnetwork".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "selfLink".into(),
                    value: &self_link_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSubnetworkResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            gateway_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayAddress"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            internal_ipv6_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("internalIpv6Prefix"),
            ),
            ip_cidr_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipCidrRange"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            private_ip_google_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpGoogleAccess"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            secondary_ip_ranges: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryIpRanges"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
        }
    }
}
