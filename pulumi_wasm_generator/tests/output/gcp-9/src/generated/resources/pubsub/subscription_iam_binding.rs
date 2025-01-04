/// Three different resources help you manage your IAM policy for pubsub subscription. Each of these resources serves a different use case:
///
/// * `gcp.pubsub.SubscriptionIAMPolicy`: Authoritative. Sets the IAM policy for the subscription and replaces any existing policy already attached.
/// * `gcp.pubsub.SubscriptionIAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the subscription are preserved.
/// * `gcp.pubsub.SubscriptionIAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the subscription are preserved.
///
/// > **Note:** `gcp.pubsub.SubscriptionIAMPolicy` **cannot** be used in conjunction with `gcp.pubsub.SubscriptionIAMBinding` and `gcp.pubsub.SubscriptionIAMMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.pubsub.SubscriptionIAMBinding` resources **can be** used in conjunction with `gcp.pubsub.SubscriptionIAMMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.pubsub.SubscriptionIAMPolicy
///
/// ```yaml
/// resources:
///   editor:
///     type: gcp:pubsub:SubscriptionIAMPolicy
///     properties:
///       subscription: your-subscription-name
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/editor
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.pubsub.SubscriptionIAMBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = subscription_iam_binding::create(
///         "editor",
///         SubscriptionIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .role("roles/editor")
///             .subscription("your-subscription-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.pubsub.SubscriptionIAMMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = subscription_iam_member::create(
///         "editor",
///         SubscriptionIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .role("roles/editor")
///             .subscription("your-subscription-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.pubsub.SubscriptionIAMBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = subscription_iam_binding::create(
///         "editor",
///         SubscriptionIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .role("roles/editor")
///             .subscription("your-subscription-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.pubsub.SubscriptionIAMMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = subscription_iam_member::create(
///         "editor",
///         SubscriptionIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .role("roles/editor")
///             .subscription("your-subscription-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Importing IAM policies
///
/// IAM policy imports use the identifier of the Pubsub Subscription resource. For example:
///
/// * `"projects/{{project_id}}/subscriptions/{{subscription}}"`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = "projects/{{project_id}}/subscriptions/{{subscription}}"
///
///   to = google_pubsub_subscription_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:pubsub/subscriptionIAMBinding:SubscriptionIAMBinding default projects/{{project_id}}/subscriptions/{{subscription}}
/// ```
///
pub mod subscription_iam_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionIAMBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::pubsub::SubscriptionIamBindingCondition>,
        >,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        #[builder(into)]
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.pubsub.SubscriptionIAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
        /// The subscription name or id to bind to attach IAM policy to.
        #[builder(into)]
        pub subscription: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionIAMBindingResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::pubsub::SubscriptionIamBindingCondition>,
        >,
        /// (Computed) The etag of the subscription's IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.pubsub.SubscriptionIAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
        /// The subscription name or id to bind to attach IAM policy to.
        pub subscription: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SubscriptionIAMBindingArgs,
    ) -> SubscriptionIAMBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let members_binding = args.members.get_inner();
        let project_binding = args.project.get_inner();
        let role_binding = args.role.get_inner();
        let subscription_binding = args.subscription.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:pubsub/subscriptionIAMBinding:SubscriptionIAMBinding".into(),
            name: name.to_string(),
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
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
                register_interface::ObjectField {
                    name: "subscription".into(),
                    value: &subscription_binding,
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
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
                },
                register_interface::ResultField {
                    name: "subscription".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SubscriptionIAMBindingResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
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
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
            subscription: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscription").unwrap(),
            ),
        }
    }
}
