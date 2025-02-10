/// A Firewall endpoint is a Cloud Firewall resource that enables
/// layer 7 advanced protection capabilities, such as intrusion prevention,
/// in your network.
///
///
/// To get more information about FirewallEndpoint, see:
///
/// * [API documentation](https://cloud.google.com/firewall/docs/reference/network-security/rest/v1/organizations.locations.firewallEndpoints)
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
/// ### Network Security Firewall Endpoint Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networksecurity:FirewallEndpoint
///     properties:
///       name: my-firewall-endpoint
///       parent: organizations/123456789
///       location: us-central1-a
///       billingProjectId: my-project-name
///       labels:
///         foo: bar
/// ```
///
/// ## Import
///
/// FirewallEndpoint can be imported using any of these accepted formats:
///
/// * `{{parent}}/locations/{{location}}/firewallEndpoints/{{name}}`
///
/// When using the `pulumi import` command, FirewallEndpoint can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/firewallEndpoint:FirewallEndpoint default {{parent}}/locations/{{location}}/firewallEndpoints/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallEndpointArgs {
        /// Project to bill on endpoint uptime usage.
        #[builder(into)]
        pub billing_project_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of key/value label pairs to assign to the resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location (zone) of the firewall endpoint.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the firewall endpoint resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the parent this firewall endpoint belongs to.
        /// Format: organizations/{organization_id}.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FirewallEndpointResult {
        /// List of networks that are associated with this endpoint in the local zone.
        /// This is a projection of the FirewallEndpointAssociations pointing at this
        /// endpoint. A network will only appear in this list after traffic routing is
        /// fully configured. Format: projects/{project}/global/networks/{name}.
        pub associated_networks: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Project to bill on endpoint uptime usage.
        pub billing_project_id: pulumi_gestalt_rust::Output<String>,
        /// Time the firewall endpoint was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A map of key/value label pairs to assign to the resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location (zone) of the firewall endpoint.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the firewall endpoint resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the parent this firewall endpoint belongs to.
        /// Format: organizations/{organization_id}.
        ///
        ///
        /// - - -
        pub parent: pulumi_gestalt_rust::Output<String>,
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
        args: FirewallEndpointArgs,
    ) -> FirewallEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let billing_project_id_binding = args.billing_project_id.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networksecurity/firewallEndpoint:FirewallEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "billingProjectId".into(),
                    value: billing_project_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
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
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallEndpointResult {
            associated_networks: o.get_field("associatedNetworks"),
            billing_project_id: o.get_field("billingProjectId"),
            create_time: o.get_field("createTime"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reconciling: o.get_field("reconciling"),
            self_link: o.get_field("selfLink"),
            state: o.get_field("state"),
            update_time: o.get_field("updateTime"),
        }
    }
}
