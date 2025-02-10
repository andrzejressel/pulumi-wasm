/// Tunnel destination groups represent resources that have the same tunnel access restrictions.
///
///
/// To get more information about TunnelDestGroup, see:
///
/// * [API documentation](https://cloud.google.com/iap/docs/reference/rest/v1/projects.iap_tunnel.locations.destGroups)
/// * How-to Guides
///     * [Set up IAP TCP forwarding with an IP address or hostname in a Google Cloud or non-Google Cloud environment](https://cloud.google.com/iap/docs/tcp-by-host)
///
/// ## Example Usage
///
/// ### Iap Destgroup
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let destGroup = tunnel_dest_group::create(
///         "destGroup",
///         TunnelDestGroupArgs::builder()
///             .cidrs(vec!["10.1.0.0/16", "192.168.10.0/24",])
///             .group_name("testgroup_2067")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// TunnelDestGroup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/iap_tunnel/locations/{{region}}/destGroups/{{group_name}}`
///
/// * `{{project}}/iap_tunnel/locations/{{region}}/destGroups/{{group_name}}`
///
/// * `{{project}}/{{region}}/{{group_name}}`
///
/// * `{{region}}/destGroups/{{group_name}}`
///
/// * `{{region}}/{{group_name}}`
///
/// * `{{group_name}}`
///
/// When using the `pulumi import` command, TunnelDestGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:iap/tunnelDestGroup:TunnelDestGroup default projects/{{project}}/iap_tunnel/locations/{{region}}/destGroups/{{group_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iap/tunnelDestGroup:TunnelDestGroup default {{project}}/iap_tunnel/locations/{{region}}/destGroups/{{group_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iap/tunnelDestGroup:TunnelDestGroup default {{project}}/{{region}}/{{group_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iap/tunnelDestGroup:TunnelDestGroup default {{region}}/destGroups/{{group_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iap/tunnelDestGroup:TunnelDestGroup default {{region}}/{{group_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iap/tunnelDestGroup:TunnelDestGroup default {{group_name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tunnel_dest_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TunnelDestGroupArgs {
        /// List of CIDRs that this group applies to.
        #[builder(into, default)]
        pub cidrs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of FQDNs that this group applies to.
        #[builder(into, default)]
        pub fqdns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Unique tunnel destination group name.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the tunnel group. Must be the same as the network resources in the group.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TunnelDestGroupResult {
        /// List of CIDRs that this group applies to.
        pub cidrs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of FQDNs that this group applies to.
        pub fqdns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Unique tunnel destination group name.
        ///
        ///
        /// - - -
        pub group_name: pulumi_gestalt_rust::Output<String>,
        /// Full resource name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The region of the tunnel group. Must be the same as the network resources in the group.
        pub region: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TunnelDestGroupArgs,
    ) -> TunnelDestGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cidrs_binding = args.cidrs.get_output(context);
        let fqdns_binding = args.fqdns.get_output(context);
        let group_name_binding = args.group_name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:iap/tunnelDestGroup:TunnelDestGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidrs".into(),
                    value: cidrs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fqdns".into(),
                    value: fqdns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupName".into(),
                    value: group_name_binding.get_id(),
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
        TunnelDestGroupResult {
            cidrs: o.get_field("cidrs"),
            fqdns: o.get_field("fqdns"),
            group_name: o.get_field("groupName"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            region: o.get_field("region"),
        }
    }
}
