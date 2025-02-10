/// ## Example Usage
///
/// ### Active Directory Peering Basic
///
///
/// ```yaml
/// resources:
///   ad-domain-peering:
///     type: gcp:activedirectory:Peering
///     properties:
///       domainResource: ${["ad-domain"].name}
///       peeringId: ad-domain-peering
///       authorizedNetwork: ${["peered-network"].id}
///       deletionProtection: false
///       labels:
///         foo: bar
///   ad-domain:
///     type: gcp:activedirectory:Domain
///     properties:
///       domainName: ad.test.hashicorptest.com
///       locations:
///         - us-central1
///       reservedIpRange: 192.168.255.0/24
///       authorizedNetworks:
///         - ${["source-network"].id}
///       deletionProtection: false
///   peered-network:
///     type: gcp:compute:Network
///     properties:
///       project: ${compute.project}
///       name: ad-peered-network
///   source-network:
///     type: gcp:compute:Network
///     properties:
///       name: ad-network
///   compute:
///     type: gcp:projects:Service
///     properties:
///       project: ${["peered-project"].projectId}
///       service: compute.googleapis.com
///   peered-project:
///     type: gcp:organizations:Project
///     properties:
///       name: my-peered-project
///       projectId: my-peered-project
///       orgId: '123456789'
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod peering {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PeeringArgs {
        /// The full names of the Google Compute Engine networks to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail.
        #[builder(into)]
        pub authorized_network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form projects/{projectId}/locations/global/domains/{domainName}
        #[builder(into)]
        pub domain_resource: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resource labels that can contain user-provided metadata
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// - - -
        #[builder(into)]
        pub peering_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The current state of this Peering.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Additional information about the current status of this peering, if available.
        #[builder(into, default)]
        pub status_message: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PeeringResult {
        /// The full names of the Google Compute Engine networks to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail.
        pub authorized_network: pulumi_gestalt_rust::Output<String>,
        /// Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form projects/{projectId}/locations/global/domains/{domainName}
        pub domain_resource: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resource labels that can contain user-provided metadata
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Unique name of the peering in this scope including projects and location using the form: projects/{projectId}/locations/global/peerings/{peeringId}.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// - - -
        pub peering_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The current state of this Peering.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// Additional information about the current status of this peering, if available.
        pub status_message: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PeeringArgs,
    ) -> PeeringResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorized_network_binding = args.authorized_network.get_output(context);
        let domain_resource_binding = args.domain_resource.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let peering_id_binding = args.peering_id.get_output(context);
        let project_binding = args.project.get_output(context);
        let status_binding = args.status.get_output(context);
        let status_message_binding = args.status_message.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:activedirectory/peering:Peering".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizedNetwork".into(),
                    value: authorized_network_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainResource".into(),
                    value: domain_resource_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peeringId".into(),
                    value: peering_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "statusMessage".into(),
                    value: status_message_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PeeringResult {
            authorized_network: o.get_field("authorizedNetwork"),
            domain_resource: o.get_field("domainResource"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            peering_id: o.get_field("peeringId"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            status: o.get_field("status"),
            status_message: o.get_field("statusMessage"),
        }
    }
}
