/// Provides a resource to manage a CloudWatch log resource policy.
///
/// ## Example Usage
///
/// ### Elasticsearch Log Publishing
///
/// ```yaml
/// resources:
///   elasticsearch-log-publishing-policyLogResourcePolicy:
///     type: aws:cloudwatch:LogResourcePolicy
///     name: elasticsearch-log-publishing-policy
///     properties:
///       policyDocument: ${["elasticsearch-log-publishing-policy"].json}
///       policyName: elasticsearch-log-publishing-policy
/// variables:
///   elasticsearch-log-publishing-policy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - logs:CreateLogStream
///               - logs:PutLogEvents
///               - logs:PutLogEventsBatch
///             resources:
///               - arn:aws:logs:*
///             principals:
///               - identifiers:
///                   - es.amazonaws.com
///                 type: Service
/// ```
///
/// ### Route53 Query Logging
///
/// ```yaml
/// resources:
///   route53-query-logging-policyLogResourcePolicy:
///     type: aws:cloudwatch:LogResourcePolicy
///     name: route53-query-logging-policy
///     properties:
///       policyDocument: ${["route53-query-logging-policy"].json}
///       policyName: route53-query-logging-policy
/// variables:
///   route53-query-logging-policy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - logs:CreateLogStream
///               - logs:PutLogEvents
///             resources:
///               - arn:aws:logs:*:*:log-group:/aws/route53/*
///             principals:
///               - identifiers:
///                   - route53.amazonaws.com
///                 type: Service
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch log resource policies using the policy name. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logResourcePolicy:LogResourcePolicy MyPolicy MyPolicy
/// ```
pub mod log_resource_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogResourcePolicyArgs {
        /// Details of the resource policy, including the identity of the principal that is enabled to put logs to this account. This is formatted as a JSON string. Maximum length of 5120 characters.
        #[builder(into)]
        pub policy_document: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the resource policy.
        #[builder(into)]
        pub policy_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LogResourcePolicyResult {
        /// Details of the resource policy, including the identity of the principal that is enabled to put logs to this account. This is formatted as a JSON string. Maximum length of 5120 characters.
        pub policy_document: pulumi_wasm_rust::Output<String>,
        /// Name of the resource policy.
        pub policy_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LogResourcePolicyArgs,
    ) -> LogResourcePolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_document_binding = args
            .policy_document
            .get_output(context)
            .get_inner();
        let policy_name_binding = args.policy_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/logResourcePolicy:LogResourcePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policyDocument".into(),
                    value: &policy_document_binding,
                },
                register_interface::ObjectField {
                    name: "policyName".into(),
                    value: &policy_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "policyDocument".into(),
                },
                register_interface::ResultField {
                    name: "policyName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LogResourcePolicyResult {
            policy_document: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyDocument").unwrap(),
            ),
            policy_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyName").unwrap(),
            ),
        }
    }
}
