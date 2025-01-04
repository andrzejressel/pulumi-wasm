/// Three different resources help you manage your IAM policy for Identity-Aware Proxy TunnelDestGroup. Each of these resources serves a different use case:
///
/// * `gcp.iap.TunnelDestGroupIamPolicy`: Authoritative. Sets the IAM policy for the tunneldestgroup and replaces any existing policy already attached.
/// * `gcp.iap.TunnelDestGroupIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the tunneldestgroup are preserved.
/// * `gcp.iap.TunnelDestGroupIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the tunneldestgroup are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.iap.TunnelDestGroupIamPolicy`: Retrieves the IAM policy for the tunneldestgroup
///
/// > **Note:** `gcp.iap.TunnelDestGroupIamPolicy` **cannot** be used in conjunction with `gcp.iap.TunnelDestGroupIamBinding` and `gcp.iap.TunnelDestGroupIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.iap.TunnelDestGroupIamBinding` resources **can be** used in conjunction with `gcp.iap.TunnelDestGroupIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.iap.TunnelDestGroupIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:TunnelDestGroupIamPolicy
///     properties:
///       project: ${destGroup.project}
///       region: ${destGroup.region}
///       destGroup: ${destGroup.groupName}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.tunnelResourceAccessor
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:TunnelDestGroupIamPolicy
///     properties:
///       project: ${destGroup.project}
///       region: ${destGroup.region}
///       destGroup: ${destGroup.groupName}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.tunnelResourceAccessor
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.iap.TunnelDestGroupIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = tunnel_dest_group_iam_binding::create(
///         "binding",
///         TunnelDestGroupIamBindingArgs::builder()
///             .dest_group("${destGroup.groupName}")
///             .members(vec!["user:jane@example.com",])
///             .project("${destGroup.project}")
///             .region("${destGroup.region}")
///             .role("roles/iap.tunnelResourceAccessor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = tunnel_dest_group_iam_binding::create(
///         "binding",
///         TunnelDestGroupIamBindingArgs::builder()
///             .condition(
///                 TunnelDestGroupIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .dest_group("${destGroup.groupName}")
///             .members(vec!["user:jane@example.com",])
///             .project("${destGroup.project}")
///             .region("${destGroup.region}")
///             .role("roles/iap.tunnelResourceAccessor")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.iap.TunnelDestGroupIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = tunnel_dest_group_iam_member::create(
///         "member",
///         TunnelDestGroupIamMemberArgs::builder()
///             .dest_group("${destGroup.groupName}")
///             .member("user:jane@example.com")
///             .project("${destGroup.project}")
///             .region("${destGroup.region}")
///             .role("roles/iap.tunnelResourceAccessor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = tunnel_dest_group_iam_member::create(
///         "member",
///         TunnelDestGroupIamMemberArgs::builder()
///             .condition(
///                 TunnelDestGroupIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .dest_group("${destGroup.groupName}")
///             .member("user:jane@example.com")
///             .project("${destGroup.project}")
///             .region("${destGroup.region}")
///             .role("roles/iap.tunnelResourceAccessor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for Identity-Aware Proxy TunnelDestGroup
/// Three different resources help you manage your IAM policy for Identity-Aware Proxy TunnelDestGroup. Each of these resources serves a different use case:
///
/// * `gcp.iap.TunnelDestGroupIamPolicy`: Authoritative. Sets the IAM policy for the tunneldestgroup and replaces any existing policy already attached.
/// * `gcp.iap.TunnelDestGroupIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the tunneldestgroup are preserved.
/// * `gcp.iap.TunnelDestGroupIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the tunneldestgroup are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.iap.TunnelDestGroupIamPolicy`: Retrieves the IAM policy for the tunneldestgroup
///
/// > **Note:** `gcp.iap.TunnelDestGroupIamPolicy` **cannot** be used in conjunction with `gcp.iap.TunnelDestGroupIamBinding` and `gcp.iap.TunnelDestGroupIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.iap.TunnelDestGroupIamBinding` resources **can be** used in conjunction with `gcp.iap.TunnelDestGroupIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.iap.TunnelDestGroupIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:TunnelDestGroupIamPolicy
///     properties:
///       project: ${destGroup.project}
///       region: ${destGroup.region}
///       destGroup: ${destGroup.groupName}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.tunnelResourceAccessor
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:TunnelDestGroupIamPolicy
///     properties:
///       project: ${destGroup.project}
///       region: ${destGroup.region}
///       destGroup: ${destGroup.groupName}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.tunnelResourceAccessor
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.iap.TunnelDestGroupIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = tunnel_dest_group_iam_binding::create(
///         "binding",
///         TunnelDestGroupIamBindingArgs::builder()
///             .dest_group("${destGroup.groupName}")
///             .members(vec!["user:jane@example.com",])
///             .project("${destGroup.project}")
///             .region("${destGroup.region}")
///             .role("roles/iap.tunnelResourceAccessor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = tunnel_dest_group_iam_binding::create(
///         "binding",
///         TunnelDestGroupIamBindingArgs::builder()
///             .condition(
///                 TunnelDestGroupIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .dest_group("${destGroup.groupName}")
///             .members(vec!["user:jane@example.com",])
///             .project("${destGroup.project}")
///             .region("${destGroup.region}")
///             .role("roles/iap.tunnelResourceAccessor")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.iap.TunnelDestGroupIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = tunnel_dest_group_iam_member::create(
///         "member",
///         TunnelDestGroupIamMemberArgs::builder()
///             .dest_group("${destGroup.groupName}")
///             .member("user:jane@example.com")
///             .project("${destGroup.project}")
///             .region("${destGroup.region}")
///             .role("roles/iap.tunnelResourceAccessor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = tunnel_dest_group_iam_member::create(
///         "member",
///         TunnelDestGroupIamMemberArgs::builder()
///             .condition(
///                 TunnelDestGroupIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .dest_group("${destGroup.groupName}")
///             .member("user:jane@example.com")
///             .project("${destGroup.project}")
///             .region("${destGroup.region}")
///             .role("roles/iap.tunnelResourceAccessor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/iap_tunnel/locations/{{region}}/destGroups/{{dest_group}}
///
/// * {{project}}/iap_tunnel/locations/{{region}}/destGroups/{{dest_group}}
///
/// * {{project}}/{{region}}/{{dest_group}}
///
/// * {{region}}/{{dest_group}}
///
/// * {{dest_group}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Identity-Aware Proxy tunneldestgroup IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:iap/tunnelDestGroupIamBinding:TunnelDestGroupIamBinding editor "projects/{{project}}/iap_tunnel/locations/{{region}}/destGroups/{{dest_group}} roles/iap.tunnelResourceAccessor user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:iap/tunnelDestGroupIamBinding:TunnelDestGroupIamBinding editor "projects/{{project}}/iap_tunnel/locations/{{region}}/destGroups/{{dest_group}} roles/iap.tunnelResourceAccessor"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:iap/tunnelDestGroupIamBinding:TunnelDestGroupIamBinding editor projects/{{project}}/iap_tunnel/locations/{{region}}/destGroups/{{dest_group}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod tunnel_dest_group_iam_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TunnelDestGroupIamBindingArgs {
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::iap::TunnelDestGroupIamBindingCondition>,
        >,
        #[builder(into)]
        pub dest_group: pulumi_wasm_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        #[builder(into)]
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region of the tunnel group. Must be the same as the network resources in the group.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no region is provided in the parent identifier and no
        /// region is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.iap.TunnelDestGroupIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TunnelDestGroupIamBindingResult {
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::iap::TunnelDestGroupIamBindingCondition>,
        >,
        pub dest_group: pulumi_wasm_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The region of the tunnel group. Must be the same as the network resources in the group.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no region is provided in the parent identifier and no
        /// region is specified, it is taken from the provider configuration.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.iap.TunnelDestGroupIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: TunnelDestGroupIamBindingArgs,
    ) -> TunnelDestGroupIamBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let dest_group_binding = args.dest_group.get_inner();
        let members_binding = args.members.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let role_binding = args.role.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:iap/tunnelDestGroupIamBinding:TunnelDestGroupIamBinding".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "destGroup".into(),
                    value: &dest_group_binding,
                },
                register_interface::ObjectField {
                    name: "members".into(),
                    value: &members_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
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
                    name: "destGroup".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "members".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
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
        TunnelDestGroupIamBindingResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            dest_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destGroup").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("members").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
        }
    }
}
