/// Membership contains information about a member cluster.
///
///
/// To get more information about Membership, see:
///
/// * [API documentation](https://cloud.google.com/anthos/multicluster-management/reference/rest/v1/projects.locations.memberships)
/// * How-to Guides
///     * [Registering a Cluster](https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster#register_cluster)
///
/// ## Example Usage
///
/// ### Gkehub Membership Regional
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let membership = membership::create(
///         "membership",
///         MembershipArgs::builder()
///             .endpoint(
///                 MembershipEndpoint::builder()
///                     .gkeCluster(
///                         MembershipEndpointGkeCluster::builder()
///                             .resourceLink("//container.googleapis.com/${primary.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .location("us-west1")
///             .membership_id("basic")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .deletion_protection(false)
///             .initial_node_count(1)
///             .location("us-central1-a")
///             .name("basic-cluster")
///             .network("default")
///             .subnetwork("default")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkehub Membership Basic
///
///
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:Cluster
///     properties:
///       name: basic-cluster
///       location: us-central1-a
///       initialNodeCount: 1
///       deletionProtection: true
///       network: default
///       subnetwork: default
///   membership:
///     type: gcp:gkehub:Membership
///     properties:
///       membershipId: basic
///       endpoint:
///         gkeCluster:
///           resourceLink: //container.googleapis.com/${primary.id}
///       labels:
///         env: test
/// ```
/// ### Gkehub Membership Issuer
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let membership = membership::create(
///         "membership",
///         MembershipArgs::builder()
///             .authority(
///                 MembershipAuthority::builder()
///                     .issuer("https://container.googleapis.com/v1/${primary.id}")
///                     .build_struct(),
///             )
///             .endpoint(
///                 MembershipEndpoint::builder()
///                     .gkeCluster(
///                         MembershipEndpointGkeCluster::builder()
///                             .resourceLink("${primary.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .membership_id("basic")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .location("us-central1-a")
///             .name("basic-cluster")
///             .network("default")
///             .subnetwork("default")
///             .workload_identity_config(
///                 ClusterWorkloadIdentityConfig::builder()
///                     .workloadPool("my-project-name.svc.id.goog")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Membership can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/memberships/{{membership_id}}`
///
/// * `{{project}}/{{location}}/{{membership_id}}`
///
/// * `{{location}}/{{membership_id}}`
///
/// When using the `pulumi import` command, Membership can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkehub/membership:Membership default projects/{{project}}/locations/{{location}}/memberships/{{membership_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/membership:Membership default {{project}}/{{location}}/{{membership_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/membership:Membership default {{location}}/{{membership_id}}
/// ```
///
pub mod membership {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MembershipArgs {
        /// Authority encodes how Google will recognize identities from this Membership.
        /// See the workload identity documentation for more details:
        /// https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity
        /// Structure is documented below.
        #[builder(into, default)]
        pub authority: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::gkehub::MembershipAuthority>,
        >,
        /// The name of this entity type to be displayed on the console. This field is unavailable in v1 of the API.
        ///
        /// > **Warning:** `description` is deprecated and will be removed in a future major release.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// If this Membership is a Kubernetes API server hosted on GKE, this is a self link to its GCP resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub endpoint: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::gkehub::MembershipEndpoint>,
        >,
        /// Labels to apply to this membership.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of the membership.
        /// The default value is `global`.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The client-provided identifier of the membership.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub membership_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MembershipResult {
        /// Authority encodes how Google will recognize identities from this Membership.
        /// See the workload identity documentation for more details:
        /// https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity
        /// Structure is documented below.
        pub authority: pulumi_wasm_rust::Output<
            Option<super::super::types::gkehub::MembershipAuthority>,
        >,
        /// The name of this entity type to be displayed on the console. This field is unavailable in v1 of the API.
        ///
        /// > **Warning:** `description` is deprecated and will be removed in a future major release.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If this Membership is a Kubernetes API server hosted on GKE, this is a self link to its GCP resource.
        /// Structure is documented below.
        pub endpoint: pulumi_wasm_rust::Output<
            Option<super::super::types::gkehub::MembershipEndpoint>,
        >,
        /// Labels to apply to this membership.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of the membership.
        /// The default value is `global`.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The client-provided identifier of the membership.
        ///
        ///
        /// - - -
        pub membership_id: pulumi_wasm_rust::Output<String>,
        /// The unique identifier of the membership.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MembershipArgs,
    ) -> MembershipResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authority_binding = args.authority.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let endpoint_binding = args.endpoint.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let membership_id_binding = args.membership_id.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gkehub/membership:Membership".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authority".into(),
                    value: &authority_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "endpoint".into(),
                    value: &endpoint_binding,
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
                    name: "membershipId".into(),
                    value: &membership_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authority".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "membershipId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MembershipResult {
            authority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authority").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            membership_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("membershipId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
        }
    }
}
