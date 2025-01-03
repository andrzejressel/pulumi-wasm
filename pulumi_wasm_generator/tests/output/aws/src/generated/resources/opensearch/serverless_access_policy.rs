/// Resource for managing an AWS OpenSearch Serverless Access Policy. See AWS documentation for [data access policies](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/serverless-data-access.html) and [supported data access policy permissions](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/serverless-data-access.html#serverless-data-supported-permissions).
///
/// ## Example Usage
///
/// ### Grant all collection and index permissions
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:ServerlessAccessPolicy
///     properties:
///       name: example
///       type: data
///       description: read and write permissions
///       policy:
///         fn::toJSON:
///           - Rules:
///               - ResourceType: index
///                 Resource:
///                   - index/example-collection/*
///                 Permission:
///                   - aoss:*
///               - ResourceType: collection
///                 Resource:
///                   - collection/example-collection
///                 Permission:
///                   - aoss:*
///             Principal:
///               - ${current.arn}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ### Grant read-only collection and index permissions
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:ServerlessAccessPolicy
///     properties:
///       name: example
///       type: data
///       description: read-only permissions
///       policy:
///         fn::toJSON:
///           - Rules:
///               - ResourceType: index
///                 Resource:
///                   - index/example-collection/*
///                 Permission:
///                   - aoss:DescribeIndex
///                   - aoss:ReadDocument
///               - ResourceType: collection
///                 Resource:
///                   - collection/example-collection
///                 Permission:
///                   - aoss:DescribeCollectionItems
///             Principal:
///               - ${current.arn}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ### Grant SAML identity permissions
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:ServerlessAccessPolicy
///     properties:
///       name: example
///       type: data
///       description: saml permissions
///       policy:
///         fn::toJSON:
///           - Rules:
///               - ResourceType: index
///                 Resource:
///                   - index/example-collection/*
///                 Permission:
///                   - aoss:*
///               - ResourceType: collection
///                 Resource:
///                   - collection/example-collection
///                 Permission:
///                   - aoss:*
///             Principal:
///               - saml/123456789012/myprovider/user/Annie
///               - saml/123456789012/anotherprovider/group/Accounting
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import OpenSearchServerless Access Policy using the `name` and `type` arguments separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/serverlessAccessPolicy:ServerlessAccessPolicy example example/data
/// ```
pub mod serverless_access_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerlessAccessPolicyArgs {
        /// Description of the policy. Typically used to store information about the permissions defined in the policy.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the policy.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// JSON policy document to use as the content for the new policy
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
        /// Type of access policy. Must be `data`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ServerlessAccessPolicyResult {
        /// Description of the policy. Typically used to store information about the permissions defined in the policy.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the policy.
        pub name: pulumi_wasm_rust::Output<String>,
        /// JSON policy document to use as the content for the new policy
        pub policy: pulumi_wasm_rust::Output<String>,
        /// Version of the policy.
        pub policy_version: pulumi_wasm_rust::Output<String>,
        /// Type of access policy. Must be `data`.
        ///
        /// The following arguments are optional:
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ServerlessAccessPolicyArgs,
    ) -> ServerlessAccessPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let policy_binding = args.policy.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/serverlessAccessPolicy:ServerlessAccessPolicy".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "policyVersion".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServerlessAccessPolicyResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            policy_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyVersion").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
