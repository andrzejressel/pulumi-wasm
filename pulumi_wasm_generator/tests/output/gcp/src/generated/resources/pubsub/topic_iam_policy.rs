/// Three different resources help you manage your IAM policy for Cloud Pub/Sub Topic. Each of these resources serves a different use case:
///
/// * `gcp.pubsub.TopicIAMPolicy`: Authoritative. Sets the IAM policy for the topic and replaces any existing policy already attached.
/// * `gcp.pubsub.TopicIAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the topic are preserved.
/// * `gcp.pubsub.TopicIAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the topic are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.pubsub.TopicIAMPolicy`: Retrieves the IAM policy for the topic
///
/// > **Note:** `gcp.pubsub.TopicIAMPolicy` **cannot** be used in conjunction with `gcp.pubsub.TopicIAMBinding` and `gcp.pubsub.TopicIAMMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.pubsub.TopicIAMBinding` resources **can be** used in conjunction with `gcp.pubsub.TopicIAMMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.pubsub.TopicIAMPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:pubsub:TopicIAMPolicy
///     properties:
///       project: ${example.project}
///       topic: ${example.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.pubsub.TopicIAMBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = topic_iam_binding::create(
///         "binding",
///         TopicIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("${example.project}")
///             .role("roles/viewer")
///             .topic("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.pubsub.TopicIAMMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = topic_iam_member::create(
///         "member",
///         TopicIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("${example.project}")
///             .role("roles/viewer")
///             .topic("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for Cloud Pub/Sub Topic
/// Three different resources help you manage your IAM policy for Cloud Pub/Sub Topic. Each of these resources serves a different use case:
///
/// * `gcp.pubsub.TopicIAMPolicy`: Authoritative. Sets the IAM policy for the topic and replaces any existing policy already attached.
/// * `gcp.pubsub.TopicIAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the topic are preserved.
/// * `gcp.pubsub.TopicIAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the topic are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.pubsub.TopicIAMPolicy`: Retrieves the IAM policy for the topic
///
/// > **Note:** `gcp.pubsub.TopicIAMPolicy` **cannot** be used in conjunction with `gcp.pubsub.TopicIAMBinding` and `gcp.pubsub.TopicIAMMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.pubsub.TopicIAMBinding` resources **can be** used in conjunction with `gcp.pubsub.TopicIAMMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.pubsub.TopicIAMPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:pubsub:TopicIAMPolicy
///     properties:
///       project: ${example.project}
///       topic: ${example.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.pubsub.TopicIAMBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = topic_iam_binding::create(
///         "binding",
///         TopicIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("${example.project}")
///             .role("roles/viewer")
///             .topic("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.pubsub.TopicIAMMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = topic_iam_member::create(
///         "member",
///         TopicIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("${example.project}")
///             .role("roles/viewer")
///             .topic("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/topics/{{name}}
///
/// * {{project}}/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Cloud Pub/Sub topic IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:pubsub/topicIAMPolicy:TopicIAMPolicy editor "projects/{{project}}/topics/{{topic}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:pubsub/topicIAMPolicy:TopicIAMPolicy editor "projects/{{project}}/topics/{{topic}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:pubsub/topicIAMPolicy:TopicIAMPolicy editor projects/{{project}}/topics/{{topic}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod topic_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicIAMPolicyArgs {
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub topic: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TopicIAMPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub topic: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TopicIAMPolicyArgs) -> TopicIAMPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_data_binding = args.policy_data.get_inner();
        let project_binding = args.project.get_inner();
        let topic_binding = args.topic.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:pubsub/topicIAMPolicy:TopicIAMPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "topic".into(),
                    value: &topic_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "policyData".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "topic".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TopicIAMPolicyResult {
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyData").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            topic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("topic").unwrap(),
            ),
        }
    }
}
