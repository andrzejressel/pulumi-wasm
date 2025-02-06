/// Resource for managing an AWS OpenSearch Serverless Security Policy. See AWS documentation for [encryption policies](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/serverless-encryption.html#serverless-encryption-policies) and [network policies](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/serverless-network.html#serverless-network-policies).
///
/// ## Example Usage
///
/// ### Encryption Security Policy
///
/// ### Applies to a single collection
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:ServerlessSecurityPolicy
///     properties:
///       name: example
///       type: encryption
///       description: encryption security policy for example-collection
///       policy:
///         fn::toJSON:
///           Rules:
///             - Resource:
///                 - collection/example-collection
///               ResourceType: collection
///           AWSOwnedKey: true
/// ```
///
/// ### Applies to multiple collections
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:ServerlessSecurityPolicy
///     properties:
///       name: example
///       type: encryption
///       description: encryption security policy for collections that begin with "example"
///       policy:
///         fn::toJSON:
///           Rules:
///             - Resource:
///                 - collection/example*
///               ResourceType: collection
///           AWSOwnedKey: true
/// ```
///
/// ### Using a customer managed key
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:ServerlessSecurityPolicy
///     properties:
///       name: example
///       type: encryption
///       description: encryption security policy using customer KMS key
///       policy:
///         fn::toJSON:
///           Rules:
///             - Resource:
///                 - collection/customer-managed-key-collection
///               ResourceType: collection
///           AWSOwnedKey: false
///           KmsARN: arn:aws:kms:us-east-1:123456789012:key/93fd6da4-a317-4c17-bfe9-382b5d988b36
/// ```
///
/// ### Network Security Policy
///
/// ### Allow public access to the collection endpoint and the Dashboards endpoint
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:ServerlessSecurityPolicy
///     properties:
///       name: example
///       type: network
///       description: Public access
///       policy:
///         fn::toJSON:
///           - Description: Public access to collection and Dashboards endpoint for example collection
///             Rules:
///               - ResourceType: collection
///                 Resource:
///                   - collection/example-collection
///               - ResourceType: dashboard
///                 Resource:
///                   - collection/example-collection
///             AllowFromPublic: true
/// ```
///
/// ### Allow VPC access to the collection endpoint and the Dashboards endpoint
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:ServerlessSecurityPolicy
///     properties:
///       name: example
///       type: network
///       description: VPC access
///       policy:
///         fn::toJSON:
///           - Description: VPC access to collection and Dashboards endpoint for example collection
///             Rules:
///               - ResourceType: collection
///                 Resource:
///                   - collection/example-collection
///               - ResourceType: dashboard
///                 Resource:
///                   - collection/example-collection
///             AllowFromPublic: false
///             SourceVPCEs:
///               - vpce-050f79086ee71ac05
/// ```
///
/// ### Mixed access for different collections
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:ServerlessSecurityPolicy
///     properties:
///       name: example
///       type: network
///       description: Mixed access for marketing and sales
///       policy:
///         fn::toJSON:
///           - Description: Marketing access
///             Rules:
///               - ResourceType: collection
///                 Resource:
///                   - collection/marketing*
///               - ResourceType: dashboard
///                 Resource:
///                   - collection/marketing*
///             AllowFromPublic: false
///             SourceVPCEs:
///               - vpce-050f79086ee71ac05
///           - Description: Sales access
///             Rules:
///               - ResourceType: collection
///                 Resource:
///                   - collection/finance
///             AllowFromPublic: true
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import OpenSearchServerless Security Policy using the `name` and `type` arguments separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/serverlessSecurityPolicy:ServerlessSecurityPolicy example example/encryption
/// ```
pub mod serverless_security_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerlessSecurityPolicyArgs {
        /// Description of the policy. Typically used to store information about the permissions defined in the policy.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the policy.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// JSON policy document to use as the content for the new policy
        #[builder(into)]
        pub policy: pulumi_wasm_rust::InputOrOutput<String>,
        /// Type of security policy. One of `encryption` or `network`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServerlessSecurityPolicyResult {
        /// Description of the policy. Typically used to store information about the permissions defined in the policy.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the policy.
        pub name: pulumi_wasm_rust::Output<String>,
        /// JSON policy document to use as the content for the new policy
        pub policy: pulumi_wasm_rust::Output<String>,
        /// Version of the policy.
        pub policy_version: pulumi_wasm_rust::Output<String>,
        /// Type of security policy. One of `encryption` or `network`.
        ///
        /// The following arguments are optional:
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServerlessSecurityPolicyArgs,
    ) -> ServerlessSecurityPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let policy_binding = args.policy.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/serverlessSecurityPolicy:ServerlessSecurityPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServerlessSecurityPolicyResult {
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            policy: pulumi_wasm_rust::__private::into_domain(o.extract_field("policy")),
            policy_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyVersion"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
