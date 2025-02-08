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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation)]
pub mod membership {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MembershipArgs {
        /// Authority encodes how Google will recognize identities from this Membership.
        /// See the workload identity documentation for more details:
        /// https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity
        /// Structure is documented below.
        #[builder(into, default)]
        pub authority: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkehub::MembershipAuthority>,
        >,
        /// The name of this entity type to be displayed on the console. This field is unavailable in v1 of the API.
        ///
        /// > **Warning:** `description` is deprecated and will be removed in a future major release.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If this Membership is a Kubernetes API server hosted on GKE, this is a self link to its GCP resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub endpoint: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkehub::MembershipEndpoint>,
        >,
        /// Labels to apply to this membership.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of the membership.
        /// The default value is `global`.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The client-provided identifier of the membership.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub membership_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MembershipResult {
        /// Authority encodes how Google will recognize identities from this Membership.
        /// See the workload identity documentation for more details:
        /// https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity
        /// Structure is documented below.
        pub authority: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkehub::MembershipAuthority>,
        >,
        /// The name of this entity type to be displayed on the console. This field is unavailable in v1 of the API.
        ///
        /// > **Warning:** `description` is deprecated and will be removed in a future major release.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If this Membership is a Kubernetes API server hosted on GKE, this is a self link to its GCP resource.
        /// Structure is documented below.
        pub endpoint: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkehub::MembershipEndpoint>,
        >,
        /// Labels to apply to this membership.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of the membership.
        /// The default value is `global`.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The client-provided identifier of the membership.
        ///
        ///
        /// - - -
        pub membership_id: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier of the membership.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
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
        args: MembershipArgs,
    ) -> MembershipResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        MembershipResult {
            authority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authority"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            membership_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("membershipId"),
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
