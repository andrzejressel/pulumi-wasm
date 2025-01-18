/// Allows creation and management of a single binding within IAM policy for
/// an existing Google Cloud Platform Organization.
///
/// > **Note:** This resource __must not__ be used in conjunction with
///    `gcp.organizations.IAMMember` for the __same role__ or they will fight over
///    what your policy should be.
///
/// > **Note:** On create, this resource will overwrite members of any existing roles.
///     Use `pulumi import` and inspect the `output to ensure
///     your existing members are preserved.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = iam_binding::create(
///         "binding",
///         IamBindingArgs::builder()
///             .members(vec!["user:alice@gmail.com",])
///             .org_id("123456789")
///             .role("roles/browser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// IAM binding imports use space-delimited identifiers; first the resource in question and then the role.  These bindings can be imported using the `org_id` and role, e.g.
///
/// ```sh
/// $ pulumi import gcp:organizations/iAMBinding:IAMBinding my_org "your-org-id roles/viewer"
/// ```
///
/// -> **Custom Roles**: If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod iam_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IAMBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::organizations::IamBindingCondition>,
        >,
        /// A list of users that the role should apply to. For more details on format and restrictions see https://cloud.google.com/billing/reference/rest/v1/Policy#Binding
        #[builder(into)]
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The numeric ID of the organization in which you want to create a custom role.
        #[builder(into)]
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.organizations.IAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct IAMBindingResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::organizations::IamBindingCondition>,
        >,
        /// (Computed) The etag of the organization's IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// A list of users that the role should apply to. For more details on format and restrictions see https://cloud.google.com/billing/reference/rest/v1/Policy#Binding
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The numeric ID of the organization in which you want to create a custom role.
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.organizations.IAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: IAMBindingArgs) -> IAMBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let members_binding = args.members.get_inner();
        let org_id_binding = args.org_id.get_inner();
        let role_binding = args.role.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:organizations/iAMBinding:IAMBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "members".into(),
                    value: &members_binding,
                },
                register_interface::ObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "condition".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "members".into(),
                },
                register_interface::ResultField {
                    name: "orgId".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IAMBindingResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("members").unwrap(),
            ),
            org_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("orgId").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
        }
    }
}
