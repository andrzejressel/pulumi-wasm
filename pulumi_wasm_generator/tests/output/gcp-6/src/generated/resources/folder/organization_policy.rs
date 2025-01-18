/// Allows management of Organization Policies for a Google Cloud Folder.
///
/// > **Warning:** This resource has been superseded by `gcp.orgpolicy.Policy`. `gcp.orgpolicy.Policy` uses Organization Policy API V2 instead of Cloud Resource Manager API V1 and it supports additional features such as tags and conditions.
///
/// To get more information about Organization Policies, see:
///
/// * [API documentation](https://cloud.google.com/resource-manager/reference/rest/v1/folders/setOrgPolicy)
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
///     let serialPortPolicy = organization_policy::create(
///         "serialPortPolicy",
///         OrganizationPolicyArgs::builder()
///             .boolean_policy(
///                 OrganizationPolicyBooleanPolicy::builder().enforced(true).build_struct(),
///             )
///             .constraint("compute.disableSerialPortAccess")
///             .folder("folders/123456789")
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
///     let servicesPolicy = organization_policy::create(
///         "servicesPolicy",
///         OrganizationPolicyArgs::builder()
///             .constraint("serviceuser.services")
///             .folder("folders/123456789")
///             .list_policy(
///                 OrganizationPolicyListPolicy::builder()
///                     .allow(
///                         OrganizationPolicyListPolicyAllow::builder()
///                             .all(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// Or to deny some services, use the following instead:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let servicesPolicy = organization_policy::create(
///         "servicesPolicy",
///         OrganizationPolicyArgs::builder()
///             .constraint("serviceuser.services")
///             .folder("folders/123456789")
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
///             .build_struct(),
///     );
/// }
/// ```
///
/// To restore the default folder organization policy, use the following instead:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let servicesPolicy = organization_policy::create(
///         "servicesPolicy",
///         OrganizationPolicyArgs::builder()
///             .constraint("serviceuser.services")
///             .folder("folders/123456789")
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
/// Folder organization policies can be imported using any of the follow formats:
///
/// * `folders/{{folder_id}}/constraints/serviceuser.services`
///
/// * `{{folder_id}}/serviceuser.services`
///
/// When using the `pulumi import` command, folder organization policies can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:folder/organizationPolicy:OrganizationPolicy * `google_folder_organization_policy.default folders/* ``{{folder_id}}/constraints/serviceuser.services`
/// ```
///
/// ```sh
/// $ pulumi import gcp:folder/organizationPolicy:OrganizationPolicy * `* `google_folder_organization_policy.default {{folder_id}}/``serviceuser.services
/// ```
///
pub mod organization_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationPolicyArgs {
        /// A boolean policy is a constraint that is either enforced or not. Structure is documented below.
        #[builder(into, default)]
        pub boolean_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::folder::OrganizationPolicyBooleanPolicy>,
        >,
        /// The name of the Constraint the Policy is configuring, for example, `serviceuser.services`. Check out the [complete list of available constraints](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-constraints#available_constraints).
        ///
        /// - - -
        #[builder(into)]
        pub constraint: pulumi_wasm_rust::Output<String>,
        /// The resource name of the folder to set the policy for. Its format is folders/{folder_id}.
        #[builder(into)]
        pub folder: pulumi_wasm_rust::Output<String>,
        /// A policy that can define specific values that are allowed or denied for the given constraint. It
        /// can also be used to allow or deny all values. Structure is documented below.
        #[builder(into, default)]
        pub list_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::folder::OrganizationPolicyListPolicy>,
        >,
        /// A restore policy is a constraint to restore the default policy. Structure is documented below.
        ///
        /// > **Note:** If none of [`boolean_policy`, `list_policy`, `restore_policy`] are defined the policy for a given constraint will
        /// effectively be unset. This is represented in the UI as the constraint being 'Inherited'.
        ///
        /// - - -
        #[builder(into, default)]
        pub restore_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::folder::OrganizationPolicyRestorePolicy>,
        >,
        /// Version of the Policy. Default version is 0.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationPolicyResult {
        /// A boolean policy is a constraint that is either enforced or not. Structure is documented below.
        pub boolean_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::folder::OrganizationPolicyBooleanPolicy>,
        >,
        /// The name of the Constraint the Policy is configuring, for example, `serviceuser.services`. Check out the [complete list of available constraints](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-constraints#available_constraints).
        ///
        /// - - -
        pub constraint: pulumi_wasm_rust::Output<String>,
        /// (Computed) The etag of the organization policy. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The resource name of the folder to set the policy for. Its format is folders/{folder_id}.
        pub folder: pulumi_wasm_rust::Output<String>,
        /// A policy that can define specific values that are allowed or denied for the given constraint. It
        /// can also be used to allow or deny all values. Structure is documented below.
        pub list_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::folder::OrganizationPolicyListPolicy>,
        >,
        /// A restore policy is a constraint to restore the default policy. Structure is documented below.
        ///
        /// > **Note:** If none of [`boolean_policy`, `list_policy`, `restore_policy`] are defined the policy for a given constraint will
        /// effectively be unset. This is represented in the UI as the constraint being 'Inherited'.
        ///
        /// - - -
        pub restore_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::folder::OrganizationPolicyRestorePolicy>,
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
    pub fn create(name: &str, args: OrganizationPolicyArgs) -> OrganizationPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let boolean_policy_binding = args.boolean_policy.get_inner();
        let constraint_binding = args.constraint.get_inner();
        let folder_binding = args.folder.get_inner();
        let list_policy_binding = args.list_policy.get_inner();
        let restore_policy_binding = args.restore_policy.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:folder/organizationPolicy:OrganizationPolicy".into(),
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
                    name: "folder".into(),
                    value: &folder_binding,
                },
                register_interface::ObjectField {
                    name: "listPolicy".into(),
                    value: &list_policy_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "booleanPolicy".into(),
                },
                register_interface::ResultField {
                    name: "constraint".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "folder".into(),
                },
                register_interface::ResultField {
                    name: "listPolicy".into(),
                },
                register_interface::ResultField {
                    name: "restorePolicy".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OrganizationPolicyResult {
            boolean_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("booleanPolicy").unwrap(),
            ),
            constraint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("constraint").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            folder: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("folder").unwrap(),
            ),
            list_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listPolicy").unwrap(),
            ),
            restore_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restorePolicy").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
