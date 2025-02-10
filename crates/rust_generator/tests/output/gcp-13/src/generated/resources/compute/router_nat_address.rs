/// A resource used to set the list of IP addresses to be used in a NAT service and manage the draining of destroyed IPs.
///
/// > **Note:** This resource is to be used alongside a `gcp.compute.RouterNat` resource,
/// the router nat resource must have no defined `nat_ips` or `drain_nat_ips` parameters,
/// instead using the `initial_nat_ips` parameter to set at least one IP for the creation of the resource.
///
///
/// To get more information about RouterNatAddress, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/routers)
/// * How-to Guides
///     * [Google Cloud Router](https://cloud.google.com/router/docs/)
///
/// ## Example Usage
///
/// ## Import
///
/// RouterNatAddress can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/routers/{{router}}/{{router_nat}}`
///
/// * `{{project}}/{{region}}/{{router}}/{{router_nat}}`
///
/// * `{{region}}/{{router}}/{{router_nat}}`
///
/// * `{{router}}/{{router_nat}}`
///
/// When using the `pulumi import` command, RouterNatAddress can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/routerNatAddress:RouterNatAddress default projects/{{project}}/regions/{{region}}/routers/{{router}}/{{router_nat}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerNatAddress:RouterNatAddress default {{project}}/{{region}}/{{router}}/{{router_nat}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerNatAddress:RouterNatAddress default {{region}}/{{router}}/{{router_nat}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerNatAddress:RouterNatAddress default {{router}}/{{router_nat}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod router_nat_address {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouterNatAddressArgs {
        /// A list of URLs of the IP resources to be drained. These IPs must be
        /// valid static external IPs that have been assigned to the NAT.
        #[builder(into, default)]
        pub drain_nat_ips: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Self-links of NAT IPs to be used in a Nat service. Only valid if the referenced RouterNat
        /// natIpAllocateOption is set to MANUAL_ONLY.
        #[builder(into)]
        pub nat_ips: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region where the NAT service reside.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Cloud Router in which the referenced NAT service is configured.
        #[builder(into)]
        pub router: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Nat service in which this address will be configured.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub router_nat: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RouterNatAddressResult {
        /// A list of URLs of the IP resources to be drained. These IPs must be
        /// valid static external IPs that have been assigned to the NAT.
        pub drain_nat_ips: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Self-links of NAT IPs to be used in a Nat service. Only valid if the referenced RouterNat
        /// natIpAllocateOption is set to MANUAL_ONLY.
        pub nat_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Region where the NAT service reside.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The name of the Cloud Router in which the referenced NAT service is configured.
        pub router: pulumi_gestalt_rust::Output<String>,
        /// The name of the Nat service in which this address will be configured.
        ///
        ///
        /// - - -
        pub router_nat: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouterNatAddressArgs,
    ) -> RouterNatAddressResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let drain_nat_ips_binding = args.drain_nat_ips.get_output(context);
        let nat_ips_binding = args.nat_ips.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let router_binding = args.router.get_output(context);
        let router_nat_binding = args.router_nat.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/routerNatAddress:RouterNatAddress".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "drainNatIps".into(),
                    value: drain_nat_ips_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "natIps".into(),
                    value: nat_ips_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "router".into(),
                    value: router_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routerNat".into(),
                    value: router_nat_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RouterNatAddressResult {
            drain_nat_ips: o.get_field("drainNatIps"),
            nat_ips: o.get_field("natIps"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            router: o.get_field("router"),
            router_nat: o.get_field("routerNat"),
        }
    }
}
