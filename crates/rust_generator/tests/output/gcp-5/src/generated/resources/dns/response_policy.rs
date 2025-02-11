/// A Response Policy is a collection of selectors that apply to queries
/// made against one or more Virtual Private Cloud networks.
///
///
///
/// ## Example Usage
///
/// ### Dns Response Policy Basic
///
///
/// ```yaml
/// resources:
///   network-1:
///     type: gcp:compute:Network
///     properties:
///       name: network-1
///       autoCreateSubnetworks: false
///   network-2:
///     type: gcp:compute:Network
///     properties:
///       name: network-2
///       autoCreateSubnetworks: false
///   subnetwork-1:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: ${["network-1"].name}
///       network: ${["network-1"].name}
///       ipCidrRange: 10.0.36.0/24
///       region: us-central1
///       privateIpGoogleAccess: true
///       secondaryIpRanges:
///         - rangeName: pod
///           ipCidrRange: 10.0.0.0/19
///         - rangeName: svc
///           ipCidrRange: 10.0.32.0/22
///   cluster-1:
///     type: gcp:container:Cluster
///     properties:
///       name: cluster-1
///       location: us-central1-c
///       initialNodeCount: 1
///       networkingMode: VPC_NATIVE
///       defaultSnatStatus:
///         disabled: true
///       network: ${["network-1"].name}
///       subnetwork: ${["subnetwork-1"].name}
///       privateClusterConfig:
///         enablePrivateEndpoint: true
///         enablePrivateNodes: true
///         masterIpv4CidrBlock: 10.42.0.0/28
///         masterGlobalAccessConfig:
///           enabled: true
///       masterAuthorizedNetworksConfig: {}
///       ipAllocationPolicy:
///         clusterSecondaryRangeName: ${["subnetwork-1"].secondaryIpRanges[0].rangeName}
///         servicesSecondaryRangeName: ${["subnetwork-1"].secondaryIpRanges[1].rangeName}
///       deletionProtection: true
///   example-response-policy:
///     type: gcp:dns:ResponsePolicy
///     properties:
///       responsePolicyName: example-response-policy
///       networks:
///         - networkUrl: ${["network-1"].id}
///         - networkUrl: ${["network-2"].id}
///       gkeClusters:
///         - gkeClusterName: ${["cluster-1"].id}
/// ```
///
/// ## Import
///
/// ResponsePolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/responsePolicies/{{response_policy_name}}`
///
/// * `{{project}}/{{response_policy_name}}`
///
/// * `{{response_policy_name}}`
///
/// When using the `pulumi import` command, ResponsePolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dns/responsePolicy:ResponsePolicy default projects/{{project}}/responsePolicies/{{response_policy_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dns/responsePolicy:ResponsePolicy default {{project}}/{{response_policy_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dns/responsePolicy:ResponsePolicy default {{response_policy_name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod response_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResponsePolicyArgs {
        /// The description of the response policy, such as `My new response policy`.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The list of Google Kubernetes Engine clusters that can see this zone.
        /// Structure is documented below.
        #[builder(into, default)]
        pub gke_clusters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::dns::ResponsePolicyGkeCluster>>,
        >,
        /// The list of network names specifying networks to which this policy is applied.
        /// Structure is documented below.
        #[builder(into, default)]
        pub networks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::dns::ResponsePolicyNetwork>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user assigned name for this Response Policy, such as `myresponsepolicy`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub response_policy_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResponsePolicyResult {
        /// The description of the response policy, such as `My new response policy`.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The list of Google Kubernetes Engine clusters that can see this zone.
        /// Structure is documented below.
        pub gke_clusters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::dns::ResponsePolicyGkeCluster>>,
        >,
        /// The list of network names specifying networks to which this policy is applied.
        /// Structure is documented below.
        pub networks: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::dns::ResponsePolicyNetwork>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The user assigned name for this Response Policy, such as `myresponsepolicy`.
        ///
        ///
        /// - - -
        pub response_policy_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResponsePolicyArgs,
    ) -> ResponsePolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let gke_clusters_binding = args.gke_clusters.get_output(context);
        let networks_binding = args.networks.get_output(context);
        let project_binding = args.project.get_output(context);
        let response_policy_name_binding = args.response_policy_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dns/responsePolicy:ResponsePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gkeClusters".into(),
                    value: &gke_clusters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networks".into(),
                    value: &networks_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "responsePolicyName".into(),
                    value: &response_policy_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResponsePolicyResult {
            description: o.get_field("description"),
            gke_clusters: o.get_field("gkeClusters"),
            networks: o.get_field("networks"),
            project: o.get_field("project"),
            response_policy_name: o.get_field("responsePolicyName"),
        }
    }
}
