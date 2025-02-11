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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_endpoint_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallEndpointAssociationArgs {
        /// Whether the association is disabled. True indicates that traffic will not be intercepted.
        /// > **Note:** The API will reject the request if this value is set to true when creating the resource,
        /// otherwise on an update the association can be disabled.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The URL of the firewall endpoint that is being associated.
        #[builder(into)]
        pub firewall_endpoint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of key/value label pairs to assign to the resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location (zone) of the firewall endpoint association.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the firewall endpoint association resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URL of the network that is being associated.
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the parent this firewall endpoint association belongs to.
        /// Format: projects/{project_id}.
        #[builder(into, default)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URL of the TlsInspectionPolicy that is being associated.
        #[builder(into, default)]
        pub tls_inspection_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FirewallEndpointAssociationResult {
        /// Time the firewall endpoint was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Whether the association is disabled. True indicates that traffic will not be intercepted.
        /// > **Note:** The API will reject the request if this value is set to true when creating the resource,
        /// otherwise on an update the association can be disabled.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The URL of the firewall endpoint that is being associated.
        pub firewall_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A map of key/value label pairs to assign to the resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location (zone) of the firewall endpoint association.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the firewall endpoint association resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The URL of the network that is being associated.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// The name of the parent this firewall endpoint association belongs to.
        /// Format: projects/{project_id}.
        pub parent: pulumi_gestalt_rust::Output<Option<String>>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether reconciling is in progress, recommended per https://google.aip.dev/128.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// Server-defined URL of this resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The current state of the endpoint.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The URL of the TlsInspectionPolicy that is being associated.
        pub tls_inspection_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Time the firewall endpoint was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FirewallEndpointAssociationArgs,
    ) -> FirewallEndpointAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let disabled_binding = args.disabled.get_output(context);
        let firewall_endpoint_binding = args.firewall_endpoint.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let tls_inspection_policy_binding = args
            .tls_inspection_policy
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networksecurity/firewallEndpointAssociation:FirewallEndpointAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallEndpoint".into(),
                    value: &firewall_endpoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tlsInspectionPolicy".into(),
                    value: &tls_inspection_policy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallEndpointAssociationResult {
            create_time: o.get_field("createTime"),
            disabled: o.get_field("disabled"),
            effective_labels: o.get_field("effectiveLabels"),
            firewall_endpoint: o.get_field("firewallEndpoint"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            parent: o.get_field("parent"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reconciling: o.get_field("reconciling"),
            self_link: o.get_field("selfLink"),
            state: o.get_field("state"),
            tls_inspection_policy: o.get_field("tlsInspectionPolicy"),
            update_time: o.get_field("updateTime"),
        }
    }
}
