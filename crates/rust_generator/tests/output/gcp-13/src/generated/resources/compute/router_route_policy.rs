/// ## Example Usage
///
/// ### Router Route Policy Export
///
///
/// ```yaml
/// resources:
///   net:
///     type: gcp:compute:Network
///     properties:
///       name: my-network
///       autoCreateSubnetworks: false
///   subnet:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: my-subnetwork
///       network: ${net.id}
///       ipCidrRange: 10.0.0.0/16
///       region: us-central1
///   router:
///     type: gcp:compute:Router
///     properties:
///       name: my-router
///       region: ${subnet.region}
///       network: ${net.id}
///   rp-export:
///     type: gcp:compute:RouterRoutePolicy
///     properties:
///       router: ${router.name}
///       region: ${router.region}
///       name: my-rp1
///       type: ROUTE_POLICY_TYPE_EXPORT
///       terms:
///         - priority: 1
///           match:
///             expression: destination == '10.0.0.0/12'
///           actions:
///             - expression: accept()
/// ```
/// ### Router Route Policy Import
///
///
/// ```yaml
/// resources:
///   net:
///     type: gcp:compute:Network
///     properties:
///       name: my-network
///       autoCreateSubnetworks: false
///   subnet:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: my-subnetwork
///       network: ${net.id}
///       ipCidrRange: 10.0.0.0/16
///       region: us-central1
///   router:
///     type: gcp:compute:Router
///     properties:
///       name: my-router
///       region: ${subnet.region}
///       network: ${net.id}
///   rp-import:
///     type: gcp:compute:RouterRoutePolicy
///     properties:
///       name: my-rp2
///       router: ${router.name}
///       region: ${router.region}
///       type: ROUTE_POLICY_TYPE_IMPORT
///       terms:
///         - priority: 2
///           match:
///             expression: destination == '10.0.0.0/12'
///           actions:
///             - expression: accept()
/// ```
///
/// ## Import
///
/// RouterRoutePolicy can be imported using any of these accepted formats:
///
/// * `{{project}}/{{region}}/{{router}}/routePolicies/{{name}}`
///
/// * `{{project}}/{{region}}/{{router}}/{{name}}`
///
/// * `{{region}}/{{router}}/{{name}}`
///
/// * `{{router}}/{{name}}`
///
/// When using the `pulumi import` command, RouterRoutePolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/routerRoutePolicy:RouterRoutePolicy default {{project}}/{{region}}/{{router}}/routePolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerRoutePolicy:RouterRoutePolicy default {{project}}/{{region}}/{{router}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerRoutePolicy:RouterRoutePolicy default {{region}}/{{router}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerRoutePolicy:RouterRoutePolicy default {{router}}/{{name}}
/// ```
///
pub mod router_route_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouterRoutePolicyArgs {
        /// Name of the route policy. This policy's name, which must be a resource ID segment and unique within all policies owned by the Router
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region where the router and NAT reside.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Cloud Router in which this route policy will be configured.
        #[builder(into)]
        pub router: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of terms (the order in the list is not important, they are evaluated in order of priority).
        /// Structure is documented below.
        #[builder(into)]
        pub terms: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::compute::RouterRoutePolicyTerm>,
        >,
        /// This is policy's type, which is one of IMPORT or EXPORT Possible values: ["ROUTE_POLICY_TYPE_IMPORT",
        /// "ROUTE_POLICY_TYPE_EXPORT"]
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RouterRoutePolicyResult {
        /// The fingerprint used for optimistic locking of this resource.  Used
        /// internally during updates.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Name of the route policy. This policy's name, which must be a resource ID segment and unique within all policies owned by the Router
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Region where the router and NAT reside.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The name of the Cloud Router in which this route policy will be configured.
        pub router: pulumi_gestalt_rust::Output<String>,
        /// List of terms (the order in the list is not important, they are evaluated in order of priority).
        /// Structure is documented below.
        pub terms: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::RouterRoutePolicyTerm>,
        >,
        /// This is policy's type, which is one of IMPORT or EXPORT Possible values: ["ROUTE_POLICY_TYPE_IMPORT",
        /// "ROUTE_POLICY_TYPE_EXPORT"]
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RouterRoutePolicyArgs,
    ) -> RouterRoutePolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let router_binding = args.router.get_output(context).get_inner();
        let terms_binding = args.terms.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/routerRoutePolicy:RouterRoutePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "router".into(),
                    value: &router_binding,
                },
                register_interface::ObjectField {
                    name: "terms".into(),
                    value: &terms_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RouterRoutePolicyResult {
            fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            router: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("router"),
            ),
            terms: pulumi_gestalt_rust::__private::into_domain(o.extract_field("terms")),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
