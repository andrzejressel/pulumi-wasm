/// Firewall endpoint association links a firewall endpoint to a VPC network in
/// the same zone. After you define this association, Cloud Firewall forwards the
/// zonal workload traffic in your VPC network that requires layer 7 inspection to
/// the attached firewall endpoint.
///
///
/// To get more information about FirewallEndpointAssociation, see:
///
/// * [API documentation](https://cloud.google.com/firewall/docs/reference/network-security/rest/v1/projects.locations.firewallEndpointAssociations#FirewallEndpointAssociation)
/// * How-to Guides
///     * [Create and associate firewall endpoints](https://cloud.google.com/firewall/docs/configure-firewall-endpoints)
///     * [Firewall endpoint overview](https://cloud.google.com/firewall/docs/about-firewall-endpoints)
///
/// > **Warning:** If you are using User ADCs (Application Default Credentials) with this resource,
/// you must specify a `billing_project_id` and set `user_project_override` to true
/// in the provider configuration. Otherwise the ACM API will return a 403 error.
/// Your account must have the `serviceusage.services.use` permission on the
/// `billing_project_id` you defined.
///
/// ## Example Usage
///
/// ## Import
///
/// FirewallEndpointAssociation can be imported using any of these accepted formats:
///
/// * `{{parent}}/locations/{{location}}/firewallEndpointAssociations/{{name}}`
///
/// When using the `pulumi import` command, FirewallEndpointAssociation can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/firewallEndpointAssociation:FirewallEndpointAssociation default {{parent}}/locations/{{location}}/firewallEndpointAssociations/{{name}}
/// ```
///
pub mod firewall_endpoint_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallEndpointAssociationArgs {
        /// Whether the association is disabled. True indicates that traffic will not be intercepted.
        /// > **Note:** The API will reject the request if this value is set to true when creating the resource,
        /// otherwise on an update the association can be disabled.
        #[builder(into, default)]
        pub disabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The URL of the firewall endpoint that is being associated.
        #[builder(into)]
        pub firewall_endpoint: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of key/value label pairs to assign to the resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location (zone) of the firewall endpoint association.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the firewall endpoint association resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The URL of the network that is being associated.
        #[builder(into)]
        pub network: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the parent this firewall endpoint association belongs to.
        /// Format: projects/{project_id}.
        #[builder(into, default)]
        pub parent: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The URL of the TlsInspectionPolicy that is being associated.
        #[builder(into, default)]
        pub tls_inspection_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FirewallEndpointAssociationResult {
        /// Time the firewall endpoint was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Whether the association is disabled. True indicates that traffic will not be intercepted.
        /// > **Note:** The API will reject the request if this value is set to true when creating the resource,
        /// otherwise on an update the association can be disabled.
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The URL of the firewall endpoint that is being associated.
        pub firewall_endpoint: pulumi_wasm_rust::Output<String>,
        /// A map of key/value label pairs to assign to the resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location (zone) of the firewall endpoint association.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the firewall endpoint association resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The URL of the network that is being associated.
        pub network: pulumi_wasm_rust::Output<String>,
        /// The name of the parent this firewall endpoint association belongs to.
        /// Format: projects/{project_id}.
        pub parent: pulumi_wasm_rust::Output<Option<String>>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether reconciling is in progress, recommended per https://google.aip.dev/128.
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        /// Server-defined URL of this resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// The current state of the endpoint.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The URL of the TlsInspectionPolicy that is being associated.
        pub tls_inspection_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Time the firewall endpoint was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FirewallEndpointAssociationArgs,
    ) -> FirewallEndpointAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let disabled_binding = args.disabled.get_output(context).get_inner();
        let firewall_endpoint_binding = args
            .firewall_endpoint
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let tls_inspection_policy_binding = args
            .tls_inspection_policy
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networksecurity/firewallEndpointAssociation:FirewallEndpointAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
                },
                register_interface::ObjectField {
                    name: "firewallEndpoint".into(),
                    value: &firewall_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
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
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "tlsInspectionPolicy".into(),
                    value: &tls_inspection_policy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FirewallEndpointAssociationResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            disabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("disabled"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            firewall_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firewallEndpoint"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(o.extract_field("parent")),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reconciling"),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            tls_inspection_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tlsInspectionPolicy"),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
