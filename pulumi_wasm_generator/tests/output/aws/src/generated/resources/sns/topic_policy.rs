/// Provides an SNS topic policy resource
///
/// > **NOTE:** If a Principal is specified as just an AWS account ID rather than an ARN, AWS silently converts it to the ARN for the root user, causing future deployments to differ. To avoid this problem, just specify the full ARN, e.g. `arn:aws:iam::123456789012:root`
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let snsTopicPolicy = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .policy_id("__default_policy_ID")
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder().actions(vec!["SNS:Subscribe",
///                     "SNS:SetTopicAttributes", "SNS:RemovePermission", "SNS:Receive",
///                     "SNS:Publish", "SNS:ListSubscriptionsByTopic",
///                     "SNS:GetTopicAttributes", "SNS:DeleteTopic", "SNS:AddPermission",])
///                     .conditions(vec![GetPolicyDocumentStatementCondition::builder()
///                     .test("StringEquals").values(vec!["${[\"account-id\"]}",])
///                     .variable("AWS:SourceOwner").build_struct(),]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["*",]). type ("AWS").build_struct(),])
///                     .resources(vec!["${test.arn}",]).sid("__default_statement_ID")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let default = topic_policy::create(
///         "default",
///         TopicPolicyArgs::builder()
///             .arn("${test.arn}")
///             .policy("${snsTopicPolicy.json}")
///             .build_struct(),
///     );
///     let test = topic::create(
///         "test",
///         TopicArgs::builder().name("my-topic-with-policy").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SNS Topic Policy using the topic ARN. For example:
///
/// ```sh
/// $ pulumi import aws:sns/topicPolicy:TopicPolicy user_updates arn:aws:sns:us-west-2:123456789012:my-topic
/// ```
pub mod topic_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicPolicyArgs {
        /// The ARN of the SNS topic
        #[builder(into)]
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The fully-formed AWS policy as JSON.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TopicPolicyResult {
        /// The ARN of the SNS topic
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The AWS Account ID of the SNS topic owner
        pub owner: pulumi_wasm_rust::Output<String>,
        /// The fully-formed AWS policy as JSON.
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TopicPolicyArgs) -> TopicPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let policy_binding = args.policy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sns/topicPolicy:TopicPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TopicPolicyResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
        }
    }
}
