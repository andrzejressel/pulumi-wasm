/// Contains information about a GKEHub Feature Memberships. Feature Memberships configure GKEHub Features that apply to specific memberships rather than the project as a whole. The google_gke_hub is the Fleet API.
///
/// ## Example Usage
///
/// ### Config Management With Config Sync Auto-Upgrades And Without Git/OCI
///
/// With [Config Sync auto-upgrades](https://cloud.devsite.corp.google.com/kubernetes-engine/enterprise/config-sync/docs/how-to/upgrade-config-sync#auto-upgrade-config), Google assumes responsibility for automatically upgrading Config Sync versions
/// and overseeing the lifecycle of its components.
///
/// ```yaml
/// resources:
///   cluster:
///     type: gcp:container:Cluster
///     properties:
///       name: my-cluster
///       location: us-central1-a
///       initialNodeCount: 1
///   membership:
///     type: gcp:gkehub:Membership
///     properties:
///       membershipId: my-membership
///       endpoint:
///         gkeCluster:
///           resourceLink: //container.googleapis.com/${cluster.id}
///   feature:
///     type: gcp:gkehub:Feature
///     properties:
///       name: configmanagement
///       location: global
///       labels:
///         foo: bar
///   featureMember:
///     type: gcp:gkehub:FeatureMembership
///     name: feature_member
///     properties:
///       location: global
///       feature: ${feature.name}
///       membership: ${membership.membershipId}
///       configmanagement:
///         management: MANAGEMENT_AUTOMATIC
///         configSync:
///           enabled: true
/// ```
///
/// ### Config Management With Git
///
/// ```yaml
/// resources:
///   cluster:
///     type: gcp:container:Cluster
///     properties:
///       name: my-cluster
///       location: us-central1-a
///       initialNodeCount: 1
///   membership:
///     type: gcp:gkehub:Membership
///     properties:
///       membershipId: my-membership
///       endpoint:
///         gkeCluster:
///           resourceLink: //container.googleapis.com/${cluster.id}
///   feature:
///     type: gcp:gkehub:Feature
///     properties:
///       name: configmanagement
///       location: global
///       labels:
///         foo: bar
///   featureMember:
///     type: gcp:gkehub:FeatureMembership
///     name: feature_member
///     properties:
///       location: global
///       feature: ${feature.name}
///       membership: ${membership.membershipId}
///       configmanagement:
///         version: 1.19.0
///         configSync:
///           enabled: true
///           git:
///             syncRepo: https://github.com/hashicorp/terraform
/// ```
///
/// ### Config Management With OCI
///
/// ```yaml
/// resources:
///   cluster:
///     type: gcp:container:Cluster
///     properties:
///       name: my-cluster
///       location: us-central1-a
///       initialNodeCount: 1
///   membership:
///     type: gcp:gkehub:Membership
///     properties:
///       membershipId: my-membership
///       endpoint:
///         gkeCluster:
///           resourceLink: //container.googleapis.com/${cluster.id}
///   feature:
///     type: gcp:gkehub:Feature
///     properties:
///       name: configmanagement
///       location: global
///       labels:
///         foo: bar
///   featureMember:
///     type: gcp:gkehub:FeatureMembership
///     name: feature_member
///     properties:
///       location: global
///       feature: ${feature.name}
///       membership: ${membership.membershipId}
///       configmanagement:
///         version: 1.19.0
///         configSync:
///           enabled: true
///           oci:
///             syncRepo: us-central1-docker.pkg.dev/sample-project/config-repo/config-sync-gke:latest
///             policyDir: config-connector
///             syncWaitSecs: '20'
///             secretType: gcpserviceaccount
///             gcpServiceAccountEmail: sa@project-id.iam.gserviceaccount.com
/// ```
///
/// ### Config Management With Regional Membership
///
/// ```yaml
/// resources:
///   cluster:
///     type: gcp:container:Cluster
///     properties:
///       name: my-cluster
///       location: us-central1-a
///       initialNodeCount: 1
///   membership:
///     type: gcp:gkehub:Membership
///     properties:
///       membershipId: my-membership
///       location: us-central1
///       endpoint:
///         gkeCluster:
///           resourceLink: //container.googleapis.com/${cluster.id}
///   feature:
///     type: gcp:gkehub:Feature
///     properties:
///       name: configmanagement
///       location: global
///       labels:
///         foo: bar
///   featureMember:
///     type: gcp:gkehub:FeatureMembership
///     name: feature_member
///     properties:
///       location: global
///       feature: ${feature.name}
///       membership: ${membership.membershipId}
///       membershipLocation: ${membership.location}
///       configmanagement:
///         version: 1.19.0
///         configSync:
///           enabled: true
///           git:
///             syncRepo: https://github.com/hashicorp/terraform
/// ```
///
/// ### Multi Cluster Service Discovery
///
/// ```yaml
/// resources:
///   feature:
///     type: gcp:gkehub:Feature
///     properties:
///       name: multiclusterservicediscovery
///       location: global
///       labels:
///         foo: bar
/// ```
///
/// ### Service Mesh
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cluster = cluster::create(
///         "cluster",
///         ClusterArgs::builder()
///             .initial_node_count(1)
///             .location("us-central1-a")
///             .name("my-cluster")
///             .build_struct(),
///     );
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder().location("global").name("servicemesh").build_struct(),
///     );
///     let featureMember = feature_membership::create(
///         "featureMember",
///         FeatureMembershipArgs::builder()
///             .feature("${feature.name}")
///             .location("global")
///             .membership("${membership.membershipId}")
///             .mesh(
///                 FeatureMembershipMesh::builder()
///                     .management("MANAGEMENT_AUTOMATIC")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let membership = membership::create(
///         "membership",
///         MembershipArgs::builder()
///             .endpoint(
///                 MembershipEndpoint::builder()
///                     .gkeCluster(
///                         MembershipEndpointGkeCluster::builder()
///                             .resourceLink("//container.googleapis.com/${cluster.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .membership_id("my-membership")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Policy Controller With Minimal Configuration
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cluster = cluster::create(
///         "cluster",
///         ClusterArgs::builder()
///             .initial_node_count(1)
///             .location("us-central1-a")
///             .name("my-cluster")
///             .build_struct(),
///     );
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder().location("global").name("policycontroller").build_struct(),
///     );
///     let featureMember = feature_membership::create(
///         "featureMember",
///         FeatureMembershipArgs::builder()
///             .feature("${feature.name}")
///             .location("global")
///             .membership("${membership.membershipId}")
///             .policycontroller(
///                 FeatureMembershipPolicycontroller::builder()
///                     .policyControllerHubConfig(
///                         FeatureMembershipPolicycontrollerPolicyControllerHubConfig::builder()
///                             .installSpec("INSTALL_SPEC_ENABLED")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let membership = membership::create(
///         "membership",
///         MembershipArgs::builder()
///             .endpoint(
///                 MembershipEndpoint::builder()
///                     .gkeCluster(
///                         MembershipEndpointGkeCluster::builder()
///                             .resourceLink("//container.googleapis.com/${cluster.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .membership_id("my-membership")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Policy Controller With Custom Configurations
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cluster = cluster::create(
///         "cluster",
///         ClusterArgs::builder()
///             .initial_node_count(1)
///             .location("us-central1-a")
///             .name("my-cluster")
///             .build_struct(),
///     );
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder().location("global").name("policycontroller").build_struct(),
///     );
///     let featureMember = feature_membership::create(
///         "featureMember",
///         FeatureMembershipArgs::builder()
///             .feature("${feature.name}")
///             .location("global")
///             .membership("${membership.membershipId}")
///             .policycontroller(
///                 FeatureMembershipPolicycontroller::builder()
///                     .policyControllerHubConfig(
///                         FeatureMembershipPolicycontrollerPolicyControllerHubConfig::builder()
///                             .auditIntervalSeconds(120)
///                             .constraintViolationLimit(50)
///                             .installSpec("INSTALL_SPEC_SUSPENDED")
///                             .logDeniesEnabled(true)
///                             .mutationEnabled(true)
///                             .policyContent(
///                                 FeatureMembershipPolicycontrollerPolicyControllerHubConfigPolicyContent::builder()
///                                     .templateLibrary(
///                                         FeatureMembershipPolicycontrollerPolicyControllerHubConfigPolicyContentTemplateLibrary::builder()
///                                             .installation("NOT_INSTALLED")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .referentialRulesEnabled(true)
///                             .build_struct(),
///                     )
///                     .version("1.17.0")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let membership = membership::create(
///         "membership",
///         MembershipArgs::builder()
///             .endpoint(
///                 MembershipEndpoint::builder()
///                     .gkeCluster(
///                         MembershipEndpointGkeCluster::builder()
///                             .resourceLink("//container.googleapis.com/${cluster.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .membership_id("my-membership")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// FeatureMembership can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/features/{{feature}}/membershipId/{{membership}}`
///
/// * `{{project}}/{{location}}/{{feature}}/{{membership}}`
///
/// * `{{location}}/{{feature}}/{{membership}}`
///
/// When using the `pulumi import` command, FeatureMembership can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkehub/featureMembership:FeatureMembership default projects/{{project}}/locations/{{location}}/features/{{feature}}/membershipId/{{membership}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/featureMembership:FeatureMembership default {{project}}/{{location}}/{{feature}}/{{membership}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/featureMembership:FeatureMembership default {{location}}/{{feature}}/{{membership}}
/// ```
///
pub mod feature_membership {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FeatureMembershipArgs {
        /// Config Management-specific spec. Structure is documented below.
        #[builder(into, default)]
        pub configmanagement: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::gkehub::FeatureMembershipConfigmanagement>,
        >,
        /// The name of the feature
        #[builder(into)]
        pub feature: pulumi_wasm_rust::InputOrOutput<String>,
        /// The location of the feature
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the membership
        #[builder(into)]
        pub membership: pulumi_wasm_rust::InputOrOutput<String>,
        /// The location of the membership, for example, "us-central1". Default is "global".
        #[builder(into, default)]
        pub membership_location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Service mesh specific spec. Structure is documented below.
        #[builder(into, default)]
        pub mesh: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::gkehub::FeatureMembershipMesh>,
        >,
        /// Policy Controller-specific spec. Structure is documented below.
        #[builder(into, default)]
        pub policycontroller: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::gkehub::FeatureMembershipPolicycontroller>,
        >,
        /// The project of the feature
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FeatureMembershipResult {
        /// Config Management-specific spec. Structure is documented below.
        pub configmanagement: pulumi_wasm_rust::Output<
            Option<super::super::types::gkehub::FeatureMembershipConfigmanagement>,
        >,
        /// The name of the feature
        pub feature: pulumi_wasm_rust::Output<String>,
        /// The location of the feature
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the membership
        pub membership: pulumi_wasm_rust::Output<String>,
        /// The location of the membership, for example, "us-central1". Default is "global".
        pub membership_location: pulumi_wasm_rust::Output<Option<String>>,
        /// Service mesh specific spec. Structure is documented below.
        pub mesh: pulumi_wasm_rust::Output<
            Option<super::super::types::gkehub::FeatureMembershipMesh>,
        >,
        /// Policy Controller-specific spec. Structure is documented below.
        pub policycontroller: pulumi_wasm_rust::Output<
            Option<super::super::types::gkehub::FeatureMembershipPolicycontroller>,
        >,
        /// The project of the feature
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FeatureMembershipArgs,
    ) -> FeatureMembershipResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configmanagement_binding = args
            .configmanagement
            .get_output(context)
            .get_inner();
        let feature_binding = args.feature.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let membership_binding = args.membership.get_output(context).get_inner();
        let membership_location_binding = args
            .membership_location
            .get_output(context)
            .get_inner();
        let mesh_binding = args.mesh.get_output(context).get_inner();
        let policycontroller_binding = args
            .policycontroller
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gkehub/featureMembership:FeatureMembership".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configmanagement".into(),
                    value: &configmanagement_binding,
                },
                register_interface::ObjectField {
                    name: "feature".into(),
                    value: &feature_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "membership".into(),
                    value: &membership_binding,
                },
                register_interface::ObjectField {
                    name: "membershipLocation".into(),
                    value: &membership_location_binding,
                },
                register_interface::ObjectField {
                    name: "mesh".into(),
                    value: &mesh_binding,
                },
                register_interface::ObjectField {
                    name: "policycontroller".into(),
                    value: &policycontroller_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configmanagement".into(),
                },
                register_interface::ResultField {
                    name: "feature".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "membership".into(),
                },
                register_interface::ResultField {
                    name: "membershipLocation".into(),
                },
                register_interface::ResultField {
                    name: "mesh".into(),
                },
                register_interface::ResultField {
                    name: "policycontroller".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FeatureMembershipResult {
            configmanagement: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configmanagement").unwrap(),
            ),
            feature: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("feature").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            membership: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("membership").unwrap(),
            ),
            membership_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("membershipLocation").unwrap(),
            ),
            mesh: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mesh").unwrap(),
            ),
            policycontroller: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policycontroller").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
        }
    }
}
