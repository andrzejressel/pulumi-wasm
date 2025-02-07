/// LbRouteExtension is a resource that lets you control where traffic is routed to for a given request.
///
///
/// To get more information about LbRouteExtension, see:
///
/// * [API documentation](https://cloud.google.com/service-extensions/docs/reference/rest/v1beta1/projects.locations.lbRouteExtensions)
/// * How-to Guides
///     * [Configure a route extension](https://cloud.google.com/service-extensions/docs/configure-callout#configure_a_route_extension)
///
/// ## Example Usage
///
/// ## Import
///
/// LbRouteExtension can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/lbRouteExtensions/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, LbRouteExtension can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/lbRouteExtension:LbRouteExtension default projects/{{project}}/locations/{{location}}/lbRouteExtensions/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/lbRouteExtension:LbRouteExtension default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/lbRouteExtension:LbRouteExtension default {{location}}/{{name}}
/// ```
///
pub mod lb_route_extension {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LbRouteExtensionArgs {
        /// A human-readable description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A set of ordered extension chains that contain the match conditions and extensions to execute.
        /// Match conditions for each extension chain are evaluated in sequence for a given request.
        /// The first extension chain that has a condition that matches the request is executed.
        /// Any subsequent extension chains do not execute. Limited to 5 extension chains per resource.
        /// Structure is documented below.
        #[builder(into)]
        pub extension_chains: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::networkservices::LbRouteExtensionExtensionChain>,
        >,
        /// A list of references to the forwarding rules to which this service extension is attached to.
        /// At least one forwarding rule is required. There can be only one LbRouteExtension resource per forwarding rule.
        #[builder(into)]
        pub forwarding_rules: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Set of labels associated with the LbRouteExtension resource. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// All backend services and forwarding rules referenced by this extension must share the same load balancing scheme.
        /// For more information, refer to [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service) and
        /// [Supported application load balancers](https://cloud.google.com/service-extensions/docs/callouts-overview#supported-lbs).
        /// Possible values are: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`.
        #[builder(into)]
        pub load_balancing_scheme: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location of the route extension
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the LbRouteExtension resource in the following format: projects/{project}/locations/{location}/lbRouteExtensions/{lbRouteExtension}
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LbRouteExtensionResult {
        /// A human-readable description of the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A set of ordered extension chains that contain the match conditions and extensions to execute.
        /// Match conditions for each extension chain are evaluated in sequence for a given request.
        /// The first extension chain that has a condition that matches the request is executed.
        /// Any subsequent extension chains do not execute. Limited to 5 extension chains per resource.
        /// Structure is documented below.
        pub extension_chains: pulumi_gestalt_rust::Output<
            Vec<super::super::types::networkservices::LbRouteExtensionExtensionChain>,
        >,
        /// A list of references to the forwarding rules to which this service extension is attached to.
        /// At least one forwarding rule is required. There can be only one LbRouteExtension resource per forwarding rule.
        pub forwarding_rules: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of labels associated with the LbRouteExtension resource. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// All backend services and forwarding rules referenced by this extension must share the same load balancing scheme.
        /// For more information, refer to [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service) and
        /// [Supported application load balancers](https://cloud.google.com/service-extensions/docs/callouts-overview#supported-lbs).
        /// Possible values are: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`.
        pub load_balancing_scheme: pulumi_gestalt_rust::Output<String>,
        /// The location of the route extension
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Name of the LbRouteExtension resource in the following format: projects/{project}/locations/{location}/lbRouteExtensions/{lbRouteExtension}
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LbRouteExtensionArgs,
    ) -> LbRouteExtensionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let extension_chains_binding = args
            .extension_chains
            .get_output(context)
            .get_inner();
        let forwarding_rules_binding = args
            .forwarding_rules
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let load_balancing_scheme_binding = args
            .load_balancing_scheme
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkservices/lbRouteExtension:LbRouteExtension".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "extensionChains".into(),
                    value: &extension_chains_binding,
                },
                register_interface::ObjectField {
                    name: "forwardingRules".into(),
                    value: &forwarding_rules_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancingScheme".into(),
                    value: &load_balancing_scheme_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LbRouteExtensionResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            extension_chains: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("extensionChains"),
            ),
            forwarding_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forwardingRules"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            load_balancing_scheme: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loadBalancingScheme"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
        }
    }
}
