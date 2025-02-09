/// Provides an Amazon Managed Grafana workspace role association resource.
///
/// ## Example Usage
///
/// ### Basic configuration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:grafana:RoleAssociation
///     properties:
///       role: ADMIN
///       userIds:
///         - USER_ID_1
///         - USER_ID_2
///       workspaceId: ${exampleWorkspace.id}
///   exampleWorkspace:
///     type: aws:grafana:Workspace
///     name: example
///     properties:
///       accountAccessType: CURRENT_ACCOUNT
///       authenticationProviders:
///         - SAML
///       permissionType: SERVICE_MANAGED
///       roleArn: ${assume.arn}
///   assume:
///     type: aws:iam:Role
///     properties:
///       name: grafana-assume
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Sid: ""
///               Principal:
///                 Service: grafana.amazonaws.com
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod role_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RoleAssociationArgs {
        /// The AWS SSO group ids to be assigned the role given in `role`.
        #[builder(into, default)]
        pub group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The grafana role. Valid values can be found [here](https://docs.aws.amazon.com/grafana/latest/APIReference/API_UpdateInstruction.html#ManagedGrafana-Type-UpdateInstruction-role).
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The AWS SSO user ids to be assigned the role given in `role`.
        #[builder(into, default)]
        pub user_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The workspace id.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RoleAssociationResult {
        /// The AWS SSO group ids to be assigned the role given in `role`.
        pub group_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The grafana role. Valid values can be found [here](https://docs.aws.amazon.com/grafana/latest/APIReference/API_UpdateInstruction.html#ManagedGrafana-Type-UpdateInstruction-role).
        pub role: pulumi_gestalt_rust::Output<String>,
        /// The AWS SSO user ids to be assigned the role given in `role`.
        pub user_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The workspace id.
        ///
        /// The following arguments are optional:
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RoleAssociationArgs,
    ) -> RoleAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let group_ids_binding_1 = args.group_ids.get_output(context);
        let group_ids_binding = group_ids_binding_1.get_inner();
        let role_binding_1 = args.role.get_output(context);
        let role_binding = role_binding_1.get_inner();
        let user_ids_binding_1 = args.user_ids.get_output(context);
        let user_ids_binding = user_ids_binding_1.get_inner();
        let workspace_id_binding_1 = args.workspace_id.get_output(context);
        let workspace_id_binding = workspace_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:grafana/roleAssociation:RoleAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "groupIds".into(),
                    value: &group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
                register_interface::ObjectField {
                    name: "userIds".into(),
                    value: &user_ids_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RoleAssociationResult {
            group_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("groupIds"),
            ),
            role: pulumi_gestalt_rust::__private::into_domain(o.extract_field("role")),
            user_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userIds"),
            ),
            workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceId"),
            ),
        }
    }
}
