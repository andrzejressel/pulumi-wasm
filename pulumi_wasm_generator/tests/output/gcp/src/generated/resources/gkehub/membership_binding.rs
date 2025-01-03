/// MembershipBinding is a subresource of a Membership, representing what Fleet Scopes (or other, future Fleet resources) a Membership is bound to.
///
///
/// To get more information about MembershipBinding, see:
///
/// * [API documentation](https://cloud.google.com/anthos/fleet-management/docs/reference/rest/v1/projects.locations.memberships.bindings)
/// * How-to Guides
///     * [Registering a Cluster](https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster#register_cluster)
///
/// ## Example Usage
///
/// ### Gkehub Membership Binding Basic
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
///       membershipId: tf-test-membership_27169
///       endpoint:
///         gkeCluster:
///           resourceLink: //container.googleapis.com/${primary.id}
///     options:
///       dependsOn:
///         - ${primary}
///   scope:
///     type: gcp:gkehub:Scope
///     properties:
///       scopeId: tf-test-scope_75223
///   membershipBinding:
///     type: gcp:gkehub:MembershipBinding
///     name: membership_binding
///     properties:
///       membershipBindingId: tf-test-membership-binding_41819
///       scope: ${scope.name}
///       membershipId: ${membership.membershipId}
///       location: global
///       labels:
///         keyb: valueb
///         keya: valuea
///         keyc: valuec
///     options:
///       dependsOn:
///         - ${membership}
///         - ${scope}
/// ```
///
/// ## Import
///
/// MembershipBinding can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/memberships/{{membership_id}}/bindings/{{membership_binding_id}}`
///
/// * `{{project}}/{{location}}/{{membership_id}}/{{membership_binding_id}}`
///
/// * `{{location}}/{{membership_id}}/{{membership_binding_id}}`
///
/// When using the `pulumi import` command, MembershipBinding can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkehub/membershipBinding:MembershipBinding default projects/{{project}}/locations/{{location}}/memberships/{{membership_id}}/bindings/{{membership_binding_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/membershipBinding:MembershipBinding default {{project}}/{{location}}/{{membership_id}}/{{membership_binding_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/membershipBinding:MembershipBinding default {{location}}/{{membership_id}}/{{membership_binding_id}}
/// ```
///
pub mod membership_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MembershipBindingArgs {
        /// Labels for this Membership binding.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of the membership
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The client-provided identifier of the membership binding.
        #[builder(into)]
        pub membership_binding_id: pulumi_wasm_rust::Output<String>,
        /// Id of the membership
        #[builder(into)]
        pub membership_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A Workspace resource name in the format
        /// `projects/*/locations/*/scopes/*`.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MembershipBindingResult {
        /// Time the MembershipBinding was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Time the MembershipBinding was deleted in UTC.
        pub delete_time: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Labels for this Membership binding.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of the membership
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// The client-provided identifier of the membership binding.
        pub membership_binding_id: pulumi_wasm_rust::Output<String>,
        /// Id of the membership
        pub membership_id: pulumi_wasm_rust::Output<String>,
        /// The resource name for the membershipbinding itself
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A Workspace resource name in the format
        /// `projects/*/locations/*/scopes/*`.
        pub scope: pulumi_wasm_rust::Output<String>,
        /// State of the membership binding resource.
        /// Structure is documented below.
        pub states: pulumi_wasm_rust::Output<
            Vec<super::super::types::gkehub::MembershipBindingState>,
        >,
        /// Google-generated UUID for this resource.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Time the MembershipBinding was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MembershipBindingArgs) -> MembershipBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let membership_binding_id_binding = args.membership_binding_id.get_inner();
        let membership_id_binding = args.membership_id.get_inner();
        let project_binding = args.project.get_inner();
        let scope_binding = args.scope.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gkehub/membershipBinding:MembershipBinding".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "membershipBindingId".into(),
                    value: &membership_binding_id_binding,
                },
                register_interface::ObjectField {
                    name: "membershipId".into(),
                    value: &membership_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "deleteTime".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "membershipBindingId".into(),
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
                register_interface::ResultField {
                    name: "scope".into(),
                },
                register_interface::ResultField {
                    name: "states".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MembershipBindingResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            delete_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteTime").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            membership_binding_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("membershipBindingId").unwrap(),
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
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
            states: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("states").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
