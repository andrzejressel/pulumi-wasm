/// Ensures that a member:role pairing does not exist in a project's IAM policy.
///
/// On create, this resource will modify the policy to remove the `member` from the
/// `role`. If the membership is ever re-added, the next refresh will clear this
/// resource from state, proposing re-adding it to correct the membership. Import is
/// not supported- this resource will acquire the current policy and modify it as
/// part of creating the resource.
///
/// This resource will conflict with `gcp.projects.IAMPolicy` and
/// `gcp.projects.IAMBinding` resources that share a role, as well as
/// `gcp.projects.IAMMember` resources that target the same membership. When
/// multiple resources conflict the final state is not guaranteed to include or omit
/// the membership. Subsequent `pulumi up` calls will always show a diff
/// until the configuration is corrected.
///
/// For more information see
/// [the official documentation](https://cloud.google.com/iam/docs/granting-changing-revoking-access)
/// and
/// [API reference](https://cloud.google.com/resource-manager/reference/rest/v1/projects/setIamPolicy).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   foo:
///     type: gcp:projects:IamMemberRemove
///     properties:
///       role: roles/editor
///       project: ${targetProjectGoogleProject.projectId}
///       member: serviceAccount:${targetProjectGoogleProject.number}-compute@developer.gserviceaccount.com
/// variables:
///   targetProject:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
pub mod iam_member_remove {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IamMemberRemoveArgs {
        /// The IAM principal that should not have the target role.
        /// Each entry can have one of the following values:
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        #[builder(into)]
        pub member: pulumi_wasm_rust::Output<String>,
        /// The project id of the target project.
        #[builder(into)]
        pub project: pulumi_wasm_rust::Output<String>,
        /// The target role that should be removed.
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct IamMemberRemoveResult {
        /// The IAM principal that should not have the target role.
        /// Each entry can have one of the following values:
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        pub member: pulumi_wasm_rust::Output<String>,
        /// The project id of the target project.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The target role that should be removed.
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: IamMemberRemoveArgs) -> IamMemberRemoveResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let member_binding = args.member.get_inner();
        let project_binding = args.project.get_inner();
        let role_binding = args.role.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:projects/iamMemberRemove:IamMemberRemove".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "member".into(),
                    value: &member_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "member".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
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
        IamMemberRemoveResult {
            member: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("member").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
        }
    }
}
