/// RBACRoleBinding represents a rbacrolebinding across the Fleet.
///
///
/// To get more information about ScopeRBACRoleBinding, see:
///
/// * [API documentation](https://cloud.google.com/anthos/fleet-management/docs/reference/rest/v1/projects.locations.scopes.rbacrolebindings)
/// * How-to Guides
///     * [Registering a Cluster](https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster#register_cluster)
///
/// ## Example Usage
///
/// ### Gkehub Scope Rbac Role Binding Basic
///
///
/// ```yaml
/// resources:
///   scope:
///     type: gcp:gkehub:Scope
///     properties:
///       scopeId: tf-test-scope_29439
///   scopeRbacRoleBinding:
///     type: gcp:gkehub:ScopeRbacRoleBinding
///     name: scope_rbac_role_binding
///     properties:
///       scopeRbacRoleBindingId: tf-test-scope-rbac-role-binding_87786
///       scopeId: ${scope.scopeId}
///       user: test-email@gmail.com
///       role:
///         predefinedRole: ADMIN
///       labels:
///         key: value
///     options:
///       dependsOn:
///         - ${scope}
/// ```
///
/// ## Import
///
/// ScopeRBACRoleBinding can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/scopes/{{scope_id}}/rbacrolebindings/{{scope_rbac_role_binding_id}}`
///
/// * `{{project}}/{{scope_id}}/{{scope_rbac_role_binding_id}}`
///
/// * `{{scope_id}}/{{scope_rbac_role_binding_id}}`
///
/// When using the `pulumi import` command, ScopeRBACRoleBinding can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkehub/scopeRbacRoleBinding:ScopeRbacRoleBinding default projects/{{project}}/locations/global/scopes/{{scope_id}}/rbacrolebindings/{{scope_rbac_role_binding_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/scopeRbacRoleBinding:ScopeRbacRoleBinding default {{project}}/{{scope_id}}/{{scope_rbac_role_binding_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/scopeRbacRoleBinding:ScopeRbacRoleBinding default {{scope_id}}/{{scope_rbac_role_binding_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod scope_rbac_role_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScopeRbacRoleBindingArgs {
        /// Principal that is be authorized in the cluster (at least of one the oneof is required). Updating one will unset the
        /// other automatically. group is the group, as seen by the kubernetes cluster.
        #[builder(into, default)]
        pub group: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Labels for this ScopeRBACRoleBinding. **Note**: This field is non-authoritative, and will only manage the labels present
        /// in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Role to bind to the principal.
        /// Structure is documented below.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::gkehub::ScopeRbacRoleBindingRole,
        >,
        /// Id of the scope
        #[builder(into)]
        pub scope_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The client-provided identifier of the RBAC Role Binding.
        #[builder(into)]
        pub scope_rbac_role_binding_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Principal that is be authorized in the cluster (at least of one the oneof is required). Updating one will unset the
        /// other automatically. user is the name of the user as seen by the kubernetes cluster, example "alice" or
        /// "alice@domain.tld"
        #[builder(into, default)]
        pub user: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ScopeRbacRoleBindingResult {
        /// Time the RBAC Role Binding was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Time the RBAC Role Binding was deleted in UTC.
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Principal that is be authorized in the cluster (at least of one the oneof is required). Updating one will unset the
        /// other automatically. group is the group, as seen by the kubernetes cluster.
        pub group: pulumi_gestalt_rust::Output<Option<String>>,
        /// Labels for this ScopeRBACRoleBinding. **Note**: This field is non-authoritative, and will only manage the labels present
        /// in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name for the RBAC Role Binding
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Role to bind to the principal.
        /// Structure is documented below.
        pub role: pulumi_gestalt_rust::Output<
            super::super::types::gkehub::ScopeRbacRoleBindingRole,
        >,
        /// Id of the scope
        pub scope_id: pulumi_gestalt_rust::Output<String>,
        /// The client-provided identifier of the RBAC Role Binding.
        pub scope_rbac_role_binding_id: pulumi_gestalt_rust::Output<String>,
        /// State of the RBAC Role Binding resource.
        /// Structure is documented below.
        pub states: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gkehub::ScopeRbacRoleBindingState>,
        >,
        /// Google-generated UUID for this resource.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Time the RBAC Role Binding was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Principal that is be authorized in the cluster (at least of one the oneof is required). Updating one will unset the
        /// other automatically. user is the name of the user as seen by the kubernetes cluster, example "alice" or
        /// "alice@domain.tld"
        pub user: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScopeRbacRoleBindingArgs,
    ) -> ScopeRbacRoleBindingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let group_binding = args.group.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let project_binding = args.project.get_output(context);
        let role_binding = args.role.get_output(context);
        let scope_id_binding = args.scope_id.get_output(context);
        let scope_rbac_role_binding_id_binding = args
            .scope_rbac_role_binding_id
            .get_output(context);
        let user_binding = args.user.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:gkehub/scopeRbacRoleBinding:ScopeRbacRoleBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "group".into(),
                    value: &group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: &role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopeId".into(),
                    value: &scope_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopeRbacRoleBindingId".into(),
                    value: &scope_rbac_role_binding_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "user".into(),
                    value: &user_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ScopeRbacRoleBindingResult {
            create_time: o.get_field("createTime"),
            delete_time: o.get_field("deleteTime"),
            effective_labels: o.get_field("effectiveLabels"),
            group: o.get_field("group"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            role: o.get_field("role"),
            scope_id: o.get_field("scopeId"),
            scope_rbac_role_binding_id: o.get_field("scopeRbacRoleBindingId"),
            states: o.get_field("states"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
            user: o.get_field("user"),
        }
    }
}
