/// Provides an IAM policy attached to a user.
///
/// > **NOTE:** We suggest using explicit JSON encoding or `aws.iam.getPolicyDocument` when assigning a value to `policy`. They seamlessly translate configuration to JSON, enabling you to maintain consistency within your configuration without the need for context switches. Also, you can sidestep potential complications arising from formatting discrepancies, whitespace inconsistencies, and other nuances inherent to JSON.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   lbRo:
///     type: aws:iam:UserPolicy
///     name: lb_ro
///     properties:
///       name: test
///       user: ${lb.name}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - ec2:Describe*
///               Effect: Allow
///               Resource: '*'
///   lb:
///     type: aws:iam:User
///     properties:
///       name: loadbalancer
///       path: /system/
///   lbAccessKey:
///     type: aws:iam:AccessKey
///     name: lb
///     properties:
///       user: ${lb.name}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM User Policies using the `user_name:user_policy_name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/userPolicy:UserPolicy mypolicy user_of_mypolicy_name:mypolicy_name
/// ```
pub mod user_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserPolicyArgs {
        /// The name of the policy. If omitted, the provider will assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The policy document. This is a JSON formatted string.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
        /// IAM user to which to attach this policy.
        #[builder(into)]
        pub user: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct UserPolicyResult {
        /// The name of the policy. If omitted, the provider will assign a random, unique name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// The policy document. This is a JSON formatted string.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// IAM user to which to attach this policy.
        pub user: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: UserPolicyArgs) -> UserPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let policy_binding = args.policy.get_inner();
        let user_binding = args.user.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/userPolicy:UserPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "user".into(),
                    value: &user_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "user".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserPolicyResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            user: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("user").unwrap(),
            ),
        }
    }
}