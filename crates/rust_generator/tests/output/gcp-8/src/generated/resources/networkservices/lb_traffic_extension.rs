/// LbTrafficExtension is a resource that lets the extension service modify the headers and payloads of both requests and responses without impacting the choice of backend services or any other security policies associated with the backend service.
///
///
/// To get more information about LbTrafficExtension, see:
///
/// * [API documentation](https://cloud.google.com/service-extensions/docs/reference/rest/v1beta1/projects.locations.lbTrafficExtensions)
/// * How-to Guides
///     * [Configure a traffic extension](https://cloud.google.com/service-extensions/docs/configure-callout#configure_a_traffic_extension)
///
/// ## Example Usage
///
/// ## Import
///
/// LbTrafficExtension can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/lbTrafficExtensions/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, LbTrafficExtension can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/lbTrafficExtension:LbTrafficExtension default projects/{{project}}/locations/{{location}}/lbTrafficExtensions/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/lbTrafficExtension:LbTrafficExtension default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/lbTrafficExtension:LbTrafficExtension default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lb_traffic_extension {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LbTrafficExtensionArgs {
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
            Vec<super::super::types::networkservices::LbTrafficExtensionExtensionChain>,
        >,
        /// A list of references to the forwarding rules to which this service extension is attached to.
        /// At least one forwarding rule is required. There can be only one LBTrafficExtension resource per forwarding rule.
        #[builder(into)]
        pub forwarding_rules: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Set of labels associated with the LbTrafficExtension resource. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. For
        /// more information, refer to [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service) and
        /// [Supported application load
        /// balancers](https://cloud.google.com/service-extensions/docs/callouts-overview#supported-lbs). Possible values:
        /// ["INTERNAL_MANAGED", "EXTERNAL_MANAGED"]
        #[builder(into, default)]
        pub load_balancing_scheme: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location of the traffic extension
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the LbTrafficExtension resource in the following format: projects/{project}/locations/{location}/lbTrafficExtensions/{lbTrafficExtension}.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LbTrafficExtensionResult {
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
            Vec<super::super::types::networkservices::LbTrafficExtensionExtensionChain>,
        >,
        /// A list of references to the forwarding rules to which this service extension is attached to.
        /// At least one forwarding rule is required. There can be only one LBTrafficExtension resource per forwarding rule.
        pub forwarding_rules: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of labels associated with the LbTrafficExtension resource. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. For
        /// more information, refer to [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service) and
        /// [Supported application load
        /// balancers](https://cloud.google.com/service-extensions/docs/callouts-overview#supported-lbs). Possible values:
        /// ["INTERNAL_MANAGED", "EXTERNAL_MANAGED"]
        pub load_balancing_scheme: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location of the traffic extension
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Name of the LbTrafficExtension resource in the following format: projects/{project}/locations/{location}/lbTrafficExtensions/{lbTrafficExtension}.
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LbTrafficExtensionArgs,
    ) -> LbTrafficExtensionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let extension_chains_binding = args.extension_chains.get_output(context);
        let forwarding_rules_binding = args.forwarding_rules.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let load_balancing_scheme_binding = args
            .load_balancing_scheme
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networkservices/lbTrafficExtension:LbTrafficExtension".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extensionChains".into(),
                    value: extension_chains_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forwardingRules".into(),
                    value: forwarding_rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancingScheme".into(),
                    value: load_balancing_scheme_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LbTrafficExtensionResult {
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            extension_chains: o.get_field("extensionChains"),
            forwarding_rules: o.get_field("forwardingRules"),
            labels: o.get_field("labels"),
            load_balancing_scheme: o.get_field("loadBalancingScheme"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
        }
    }
}
