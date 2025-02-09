/// Allows management of Organization Policies for a Google Cloud Project.
///
/// > **Warning:** This resource has been superseded by `gcp.orgpolicy.Policy`. `gcp.orgpolicy.Policy` uses Organization Policy API V2 instead of Cloud Resource Manager API V1 and it supports additional features such as tags and conditions.
///
/// To get more information about Organization Policies, see:
///
/// * [API documentation](https://cloud.google.com/resource-manager/reference/rest/v1/projects/setOrgPolicy)
/// * How-to Guides
///     * [Introduction to the Organization Policy Service](https://cloud.google.com/resource-manager/docs/organization-policy/overview)
///
/// ## Example Usage
///
/// To set policy with a [boolean constraint](https://cloud.google.com/resource-manager/docs/organization-policy/quickstart-boolean-constraints):
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let serialPortPolicy = organization_policy::create(
///         "serialPortPolicy",
///         OrganizationPolicyArgs::builder()
///             .boolean_policy(
///                 OrganizationPolicyBooleanPolicy::builder().enforced(true).build_struct(),
///             )
///             .constraint("compute.disableSerialPortAccess")
///             .project("your-project-id")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// To set a policy with a [list constraint](https://cloud.google.com/resource-manager/docs/organization-policy/quickstart-list-constraints):
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let servicesPolicy = organization_policy::create(
///         "servicesPolicy",
///         OrganizationPolicyArgs::builder()
///             .constraint("serviceuser.services")
///             .list_policy(
///                 OrganizationPolicyListPolicy::builder()
///                     .allow(
///                         OrganizationPolicyListPolicyAllow::builder()
///                             .all(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .project("your-project-id")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// Or to deny some services, use the following instead:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let servicesPolicy = organization_policy::create(
///         "servicesPolicy",
///         OrganizationPolicyArgs::builder()
///             .constraint("serviceuser.services")
///             .list_policy(
///                 OrganizationPolicyListPolicy::builder()
///                     .deny(
///                         OrganizationPolicyListPolicyDeny::builder()
///                             .values(vec!["cloudresourcemanager.googleapis.com",])
///                             .build_struct(),
///                     )
///                     .suggestedValue("compute.googleapis.com")
///                     .build_struct(),
///             )
///             .project("your-project-id")
///             .build_struct(),
///     );
/// }
/// ```
///
/// To restore the default project organization policy, use the following instead:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let servicesPolicy = organization_policy::create(
///         "servicesPolicy",
///         OrganizationPolicyArgs::builder()
///             .constraint("serviceuser.services")
///             .project("your-project-id")
///             .restore_policy(
///                 OrganizationPolicyRestorePolicy::builder().default(true).build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Project organization policies can be imported using any of the follow formats:
///
/// * `projects/{{project_id}}:constraints/{{constraint}}`
///
/// * `{{project_id}}:constraints/{{constraint}}`
///
/// * `{{project_id}}:{{constraint}}`
///
/// When using the `pulumi import` command, project organization policies can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:projects/organizationPolicy:OrganizationPolicy default projects/{{project_id}}:constraints/{{constraint}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:projects/organizationPolicy:OrganizationPolicy default {{project_id}}:constraints/{{constraint}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:projects/organizationPolicy:OrganizationPolicy default {{project_id}}:{{constraint}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationPolicyArgs {
        /// A boolean policy is a constraint that is either enforced or not. Structure is documented below.
        #[builder(into, default)]
        pub boolean_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::projects::OrganizationPolicyBooleanPolicy>,
        >,
        /// The name of the Constraint the Policy is configuring, for example, `serviceuser.services`. Check out the [complete list of available constraints](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-constraints#available_constraints).
        ///
        /// - - -
        #[builder(into)]
        pub constraint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A policy that can define specific values that are allowed or denied for the given constraint. It can also be used to allow or deny all values. Structure is documented below.
        #[builder(into, default)]
        pub list_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::projects::OrganizationPolicyListPolicy>,
        >,
        /// The project id of the project to set the policy for.
        #[builder(into)]
        pub project: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A restore policy is a constraint to restore the default policy. Structure is documented below.
        ///
        /// > **Note:** If none of [`boolean_policy`, `list_policy`, `restore_policy`] are defined the policy for a given constraint will
        /// effectively be unset. This is represented in the UI as the constraint being 'Inherited'.
        ///
        /// - - -
        #[builder(into, default)]
        pub restore_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::projects::OrganizationPolicyRestorePolicy>,
        >,
        /// Version of the Policy. Default version is 0.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationPolicyResult {
        /// A boolean policy is a constraint that is either enforced or not. Structure is documented below.
        pub boolean_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::projects::OrganizationPolicyBooleanPolicy>,
        >,
        /// The name of the Constraint the Policy is configuring, for example, `serviceuser.services`. Check out the [complete list of available constraints](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-constraints#available_constraints).
        ///
        /// - - -
        pub constraint: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the organization policy. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// A policy that can define specific values that are allowed or denied for the given constraint. It can also be used to allow or deny all values. Structure is documented below.
        pub list_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::projects::OrganizationPolicyListPolicy>,
        >,
        /// The project id of the project to set the policy for.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// A restore policy is a constraint to restore the default policy. Structure is documented below.
        ///
        /// > **Note:** If none of [`boolean_policy`, `list_policy`, `restore_policy`] are defined the policy for a given constraint will
        /// effectively be unset. This is represented in the UI as the constraint being 'Inherited'.
        ///
        /// - - -
        pub restore_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::projects::OrganizationPolicyRestorePolicy>,
        >,
        /// (Computed) The timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds, representing when the variable was last updated. Example: "2016-10-09T12:33:37.578138407Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Version of the Policy. Default version is 0.
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: OrganizationPolicyArgs,
    ) -> OrganizationPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let boolean_policy_binding_1 = args.boolean_policy.get_output(context);
        let boolean_policy_binding = boolean_policy_binding_1.get_inner();
        let constraint_binding_1 = args.constraint.get_output(context);
        let constraint_binding = constraint_binding_1.get_inner();
        let list_policy_binding_1 = args.list_policy.get_output(context);
        let list_policy_binding = list_policy_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let restore_policy_binding_1 = args.restore_policy.get_output(context);
        let restore_policy_binding = restore_policy_binding_1.get_inner();
        let version_binding_1 = args.version.get_output(context);
        let version_binding = version_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:projects/organizationPolicy:OrganizationPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "booleanPolicy".into(),
                    value: &boolean_policy_binding,
                },
                register_interface::ObjectField {
                    name: "constraint".into(),
                    value: &constraint_binding,
                },
                register_interface::ObjectField {
                    name: "listPolicy".into(),
                    value: &list_policy_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "restorePolicy".into(),
                    value: &restore_policy_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        OrganizationPolicyResult {
            boolean_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("booleanPolicy"),
            ),
            constraint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("constraint"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            list_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("listPolicy"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            restore_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("restorePolicy"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
