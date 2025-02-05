/// Allows management of Organization Policies for a Google Cloud Organization.
///
/// > **Warning:** This resource has been superseded by `gcp.orgpolicy.Policy`. `gcp.orgpolicy.Policy` uses Organization Policy API V2 instead of Cloud Resource Manager API V1 and it supports additional features such as tags and conditions.
///
/// To get more information about Organization Policies, see:
///
/// * [API documentation](https://cloud.google.com/resource-manager/reference/rest/v1/organizations/setOrgPolicy)
/// * How-to Guides
///     * [Introduction to the Organization Policy Service](https://cloud.google.com/resource-manager/docs/organization-policy/overview)
///
/// ## Example Usage
///
/// To set policy with a [boolean constraint](https://cloud.google.com/resource-manager/docs/organization-policy/quickstart-boolean-constraints):
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let serialPortPolicy = policy::create(
///         "serialPortPolicy",
///         PolicyArgs::builder()
///             .boolean_policy(PolicyBooleanPolicy::builder().enforced(true).build_struct())
///             .constraint("compute.disableSerialPortAccess")
///             .org_id("123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// To set a policy with a [list constraint](https://cloud.google.com/resource-manager/docs/organization-policy/quickstart-list-constraints):
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let servicesPolicy = policy::create(
///         "servicesPolicy",
///         PolicyArgs::builder()
///             .constraint("serviceuser.services")
///             .list_policy(
///                 PolicyListPolicy::builder()
///                     .allow(PolicyListPolicyAllow::builder().all(true).build_struct())
///                     .build_struct(),
///             )
///             .org_id("123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Or to deny some services, use the following instead:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let servicesPolicy = policy::create(
///         "servicesPolicy",
///         PolicyArgs::builder()
///             .constraint("serviceuser.services")
///             .list_policy(
///                 PolicyListPolicy::builder()
///                     .deny(
///                         PolicyListPolicyDeny::builder()
///                             .values(vec!["cloudresourcemanager.googleapis.com",])
///                             .build_struct(),
///                     )
///                     .suggestedValue("compute.googleapis.com")
///                     .build_struct(),
///             )
///             .org_id("123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
/// To restore the default organization policy, use the following instead:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let servicesPolicy = policy::create(
///         "servicesPolicy",
///         PolicyArgs::builder()
///             .constraint("serviceuser.services")
///             .org_id("123456789")
///             .restore_policy(PolicyRestorePolicy::builder().default(true).build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Organization Policies can be imported using the `org_id` and the `constraint`, e.g.
///
/// * `{{org_id}}/constraints/{{constraint}}`
///
/// When using the `pulumi import` command, Organization Policies can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:organizations/policy:Policy default {{org_id}}/constraints/{{constraint}}
/// ```
///
/// It is all right if the constraint contains a slash, as in the example above.
///
pub mod policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// A boolean policy is a constraint that is either enforced or not. Structure is documented
        /// below.
        #[builder(into, default)]
        pub boolean_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::organizations::PolicyBooleanPolicy>,
        >,
        /// The name of the Constraint the Policy is configuring, for example, `serviceuser.services`. Check out the [complete list of available constraints](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-constraints#available_constraints).
        ///
        /// - - -
        #[builder(into)]
        pub constraint: pulumi_wasm_rust::InputOrOutput<String>,
        /// A policy that can define specific values that are allowed or denied for the given constraint. It can also be used to allow or deny all values. Structure is documented below.
        #[builder(into, default)]
        pub list_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::organizations::PolicyListPolicy>,
        >,
        /// The numeric ID of the organization to set the policy for.
        #[builder(into)]
        pub org_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A restore policy is a constraint to restore the default policy. Structure is documented below.
        ///
        /// > **Note:** If none of [`boolean_policy`, `list_policy`, `restore_policy`] are defined the policy for a given constraint will
        /// effectively be unset. This is represented in the UI as the constraint being 'Inherited'.
        ///
        /// - - -
        #[builder(into, default)]
        pub restore_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::organizations::PolicyRestorePolicy>,
        >,
        /// Version of the Policy. Default version is 0.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// A boolean policy is a constraint that is either enforced or not. Structure is documented
        /// below.
        pub boolean_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::organizations::PolicyBooleanPolicy>,
        >,
        /// The name of the Constraint the Policy is configuring, for example, `serviceuser.services`. Check out the [complete list of available constraints](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-constraints#available_constraints).
        ///
        /// - - -
        pub constraint: pulumi_wasm_rust::Output<String>,
        /// (Computed) The etag of the organization policy. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// A policy that can define specific values that are allowed or denied for the given constraint. It can also be used to allow or deny all values. Structure is documented below.
        pub list_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::organizations::PolicyListPolicy>,
        >,
        /// The numeric ID of the organization to set the policy for.
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// A restore policy is a constraint to restore the default policy. Structure is documented below.
        ///
        /// > **Note:** If none of [`boolean_policy`, `list_policy`, `restore_policy`] are defined the policy for a given constraint will
        /// effectively be unset. This is represented in the UI as the constraint being 'Inherited'.
        ///
        /// - - -
        pub restore_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::organizations::PolicyRestorePolicy>,
        >,
        /// (Computed) The timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds, representing when the variable was last updated. Example: "2016-10-09T12:33:37.578138407Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// Version of the Policy. Default version is 0.
        pub version: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PolicyArgs,
    ) -> PolicyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let boolean_policy_binding = args.boolean_policy.get_output(context).get_inner();
        let constraint_binding = args.constraint.get_output(context).get_inner();
        let list_policy_binding = args.list_policy.get_output(context).get_inner();
        let org_id_binding = args.org_id.get_output(context).get_inner();
        let restore_policy_binding = args.restore_policy.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:organizations/policy:Policy".into(),
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
                    name: "orgId".into(),
                    value: &org_id_binding,
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
        PolicyResult {
            boolean_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("booleanPolicy"),
            ),
            constraint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("constraint"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            list_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("listPolicy"),
            ),
            org_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("orgId")),
            restore_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("restorePolicy"),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            version: pulumi_wasm_rust::__private::into_domain(o.extract_field("version")),
        }
    }
}
