/// ## Example Usage
///
/// ### Gkehub Membership Rbac Role Binding Basic
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
///       membershipId: tf-test-membership_75092
///       endpoint:
///         gkeCluster:
///           resourceLink: //container.googleapis.com/${primary.id}
///     options:
///       dependsOn:
///         - ${primary}
///   membershipRbacRoleBinding:
///     type: gcp:gkehub:MembershipRbacRoleBinding
///     name: membership_rbac_role_binding
///     properties:
///       membershipRbacRoleBindingId: tf-test-membership-rbac-role-binding_2605
///       membershipId: ${membership.membershipId}
///       user: service-${project.number}@gcp-sa-anthossupport.iam.gserviceaccount.com
///       role:
///         predefinedRole: ANTHOS_SUPPORT
///       location: global
///     options:
///       dependsOn:
///         - ${membership}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// MembershipRBACRoleBinding can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/memberships/{{membership_id}}/rbacrolebindings/{{membership_rbac_role_binding_id}}`
///
/// * `{{project}}/{{location}}/{{membership_id}}/{{membership_rbac_role_binding_id}}`
///
/// * `{{location}}/{{membership_id}}/{{membership_rbac_role_binding_id}}`
///
/// When using the `pulumi import` command, MembershipRBACRoleBinding can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkehub/membershipRbacRoleBinding:MembershipRbacRoleBinding default projects/{{project}}/locations/{{location}}/memberships/{{membership_id}}/rbacrolebindings/{{membership_rbac_role_binding_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/membershipRbacRoleBinding:MembershipRbacRoleBinding default {{project}}/{{location}}/{{membership_id}}/{{membership_rbac_role_binding_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/membershipRbacRoleBinding:MembershipRbacRoleBinding default {{location}}/{{membership_id}}/{{membership_rbac_role_binding_id}}
/// ```
///
pub mod membership_rbac_role_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MembershipRbacRoleBindingArgs {
        /// Location of the Membership
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Id of the membership
        #[builder(into)]
        pub membership_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The client-provided identifier of the RBAC Role Binding.
        #[builder(into)]
        pub membership_rbac_role_binding_id: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Role to bind to the principal.
        /// Structure is documented below.
        #[builder(into)]
        pub role: pulumi_wasm_rust::InputOrOutput<
            super::super::types::gkehub::MembershipRbacRoleBindingRole,
        >,
        /// Principal that is be authorized in the cluster (at least of one the oneof
        /// is required). Updating one will unset the other automatically.
        /// user is the name of the user as seen by the kubernetes cluster, example
        /// "alice" or "alice@domain.tld"
        #[builder(into)]
        pub user: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MembershipRbacRoleBindingResult {
        /// Time the RBAC Role Binding was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Time the RBAC Role Binding was deleted in UTC.
        pub delete_time: pulumi_wasm_rust::Output<String>,
        /// Location of the Membership
        pub location: pulumi_wasm_rust::Output<String>,
        /// Id of the membership
        pub membership_id: pulumi_wasm_rust::Output<String>,
        /// The client-provided identifier of the RBAC Role Binding.
        pub membership_rbac_role_binding_id: pulumi_wasm_rust::Output<String>,
        /// The resource name for the RBAC Role Binding
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// Role to bind to the principal.
        /// Structure is documented below.
        pub role: pulumi_wasm_rust::Output<
            super::super::types::gkehub::MembershipRbacRoleBindingRole,
        >,
        /// State of the RBAC Role Binding resource.
        /// Structure is documented below.
        pub states: pulumi_wasm_rust::Output<
            Vec<super::super::types::gkehub::MembershipRbacRoleBindingState>,
        >,
        /// Google-generated UUID for this resource.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Time the RBAC Role Binding was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// Principal that is be authorized in the cluster (at least of one the oneof
        /// is required). Updating one will unset the other automatically.
        /// user is the name of the user as seen by the kubernetes cluster, example
        /// "alice" or "alice@domain.tld"
        pub user: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MembershipRbacRoleBindingArgs,
    ) -> MembershipRbacRoleBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let membership_id_binding = args.membership_id.get_output(context).get_inner();
        let membership_rbac_role_binding_id_binding = args
            .membership_rbac_role_binding_id
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let role_binding = args.role.get_output(context).get_inner();
        let user_binding = args.user.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gkehub/membershipRbacRoleBinding:MembershipRbacRoleBinding"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "membershipId".into(),
                    value: &membership_id_binding,
                },
                register_interface::ObjectField {
                    name: "membershipRbacRoleBindingId".into(),
                    value: &membership_rbac_role_binding_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
                register_interface::ObjectField {
                    name: "user".into(),
                    value: &user_binding,
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
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "membershipId".into(),
                },
                register_interface::ResultField {
                    name: "membershipRbacRoleBindingId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
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
                register_interface::ResultField {
                    name: "user".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MembershipRbacRoleBindingResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            delete_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteTime").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            membership_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("membershipId").unwrap(),
            ),
            membership_rbac_role_binding_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("membershipRbacRoleBindingId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
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
            user: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("user").unwrap(),
            ),
        }
    }
}
