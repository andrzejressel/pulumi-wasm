/// Represents a Router resource.
///
///
/// To get more information about Router, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/routers)
/// * How-to Guides
///     * [Google Cloud Router](https://cloud.google.com/router/docs/)
///
/// ## Example Usage
///
/// ### Router Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foobar = router::create(
///         "foobar",
///         RouterArgs::builder()
///             .bgp(
///                 RouterBgp::builder()
///                     .advertiseMode("CUSTOM")
///                     .advertisedGroups(vec!["ALL_SUBNETS",])
///                     .advertisedIpRanges(
///                         vec![
///                             RouterBgpAdvertisedIpRange::builder().range("1.2.3.4")
///                             .build_struct(), RouterBgpAdvertisedIpRange::builder()
///                             .range("6.7.0.0/16").build_struct(),
///                         ],
///                     )
///                     .asn(64514)
///                     .build_struct(),
///             )
///             .name("my-router")
///             .network("${foobarNetwork.name}")
///             .build_struct(),
///     );
///     let foobarNetwork = network::create(
///         "foobarNetwork",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("my-network")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Compute Router Encrypted Interconnect
///
///
/// ```yaml
/// resources:
///   encrypted-interconnect-router:
///     type: gcp:compute:Router
///     properties:
///       name: test-router
///       network: ${network.name}
///       encryptedInterconnectRouter: true
///       bgp:
///         asn: 64514
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: test-network
///       autoCreateSubnetworks: false
/// ```
///
/// ## Import
///
/// Router can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/routers/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Router can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/router:Router default projects/{{project}}/regions/{{region}}/routers/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/router:Router default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/router:Router default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/router:Router default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod router {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouterArgs {
        /// BGP information specific to this router.
        /// Structure is documented below.
        #[builder(into, default)]
        pub bgp: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RouterBgp>,
        >,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates if a router is dedicated for use with encrypted VLAN
        /// attachments (interconnectAttachments).
        #[builder(into, default)]
        pub encrypted_interconnect_router: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the network to which this router belongs.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region where the router resides.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RouterResult {
        /// BGP information specific to this router.
        /// Structure is documented below.
        pub bgp: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RouterBgp>,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates if a router is dedicated for use with encrypted VLAN
        /// attachments (interconnectAttachments).
        pub encrypted_interconnect_router: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A reference to the network to which this router belongs.
        ///
        ///
        /// - - -
        pub network: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Region where the router resides.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouterArgs,
    ) -> RouterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bgp_binding = args.bgp.get_output(context);
        let description_binding = args.description.get_output(context);
        let encrypted_interconnect_router_binding = args
            .encrypted_interconnect_router
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/router:Router".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgp".into(),
                    value: bgp_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptedInterconnectRouter".into(),
                    value: encrypted_interconnect_router_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: network_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RouterResult {
            bgp: o.get_field("bgp"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            encrypted_interconnect_router: o.get_field("encryptedInterconnectRouter"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            self_link: o.get_field("selfLink"),
        }
    }
}
