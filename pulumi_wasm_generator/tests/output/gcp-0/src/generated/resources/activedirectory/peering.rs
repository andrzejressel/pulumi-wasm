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
pub mod peering {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PeeringArgs {
        /// The full names of the Google Compute Engine networks to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail.
        #[builder(into)]
        pub authorized_network: pulumi_wasm_rust::Output<String>,
        /// Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form projects/{projectId}/locations/global/domains/{domainName}
        #[builder(into)]
        pub domain_resource: pulumi_wasm_rust::Output<String>,
        /// Resource labels that can contain user-provided metadata
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// - - -
        #[builder(into)]
        pub peering_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The current state of this Peering.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Additional information about the current status of this peering, if available.
        #[builder(into, default)]
        pub status_message: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PeeringResult {
        /// The full names of the Google Compute Engine networks to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail.
        pub authorized_network: pulumi_wasm_rust::Output<String>,
        /// Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form projects/{projectId}/locations/global/domains/{domainName}
        pub domain_resource: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resource labels that can contain user-provided metadata
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Unique name of the peering in this scope including projects and location using the form: projects/{projectId}/locations/global/peerings/{peeringId}.
        pub name: pulumi_wasm_rust::Output<String>,
        /// - - -
        pub peering_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The current state of this Peering.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Additional information about the current status of this peering, if available.
        pub status_message: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PeeringArgs) -> PeeringResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authorized_network_binding = args.authorized_network.get_inner();
        let domain_resource_binding = args.domain_resource.get_inner();
        let labels_binding = args.labels.get_inner();
        let peering_id_binding = args.peering_id.get_inner();
        let project_binding = args.project.get_inner();
        let status_binding = args.status.get_inner();
        let status_message_binding = args.status_message.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:activedirectory/peering:Peering".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authorizedNetwork".into(),
                    value: &authorized_network_binding,
                },
                register_interface::ObjectField {
                    name: "domainResource".into(),
                    value: &domain_resource_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "peeringId".into(),
                    value: &peering_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "statusMessage".into(),
                    value: &status_message_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authorizedNetwork".into(),
                },
                register_interface::ResultField {
                    name: "domainResource".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "peeringId".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "statusMessage".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PeeringResult {
            authorized_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizedNetwork").unwrap(),
            ),
            domain_resource: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainResource").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            peering_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peeringId").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            status_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusMessage").unwrap(),
            ),
        }
    }
}
