/// Resource for managing an AWS Agents for Amazon Bedrock Knowledge Base.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:bedrock:AgentKnowledgeBase
///     properties:
///       name: example
///       roleArn: ${exampleAwsIamRole.arn}
///       knowledgeBaseConfiguration:
///         vectorKnowledgeBaseConfiguration:
///           embeddingModelArn: arn:aws:bedrock:us-west-2::foundation-model/amazon.titan-embed-text-v1
///         type: VECTOR
///       storageConfiguration:
///         type: OPENSEARCH_SERVERLESS
///         opensearchServerlessConfiguration:
///           collectionArn: arn:aws:aoss:us-west-2:123456789012:collection/142bezjddq707i5stcrf
///           vectorIndexName: bedrock-knowledge-base-default-index
///           fieldMapping:
///             vectorField: bedrock-knowledge-base-default-vector
///             textField: AMAZON_BEDROCK_TEXT_CHUNK
///             metadataField: AMAZON_BEDROCK_METADATA
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Agents for Amazon Bedrock Knowledge Base using the knowledge base ID. For example:
///
/// ```sh
/// $ pulumi import aws:bedrock/agentKnowledgeBase:AgentKnowledgeBase example EMDPPAYPZI
/// ```
pub mod agent_knowledge_base {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgentKnowledgeBaseArgs {
        /// Description of the knowledge base.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Details about the embeddings configuration of the knowledge base. See `knowledge_base_configuration` block for details.
        #[builder(into, default)]
        pub knowledge_base_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::bedrock::AgentKnowledgeBaseKnowledgeBaseConfiguration,
            >,
        >,
        /// Name of the knowledge base.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the IAM role with permissions to invoke API operations on the knowledge base.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Details about the storage configuration of the knowledge base. See `storage_configuration` block for details.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub storage_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::AgentKnowledgeBaseStorageConfiguration>,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::AgentKnowledgeBaseTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct AgentKnowledgeBaseResult {
        /// ARN of the knowledge base.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Time at which the knowledge base was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Description of the knowledge base.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub failure_reasons: pulumi_wasm_rust::Output<Vec<String>>,
        /// Details about the embeddings configuration of the knowledge base. See `knowledge_base_configuration` block for details.
        pub knowledge_base_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::bedrock::AgentKnowledgeBaseKnowledgeBaseConfiguration,
            >,
        >,
        /// Name of the knowledge base.
        pub name: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role with permissions to invoke API operations on the knowledge base.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Details about the storage configuration of the knowledge base. See `storage_configuration` block for details.
        ///
        /// The following arguments are optional:
        pub storage_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::AgentKnowledgeBaseStorageConfiguration>,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::AgentKnowledgeBaseTimeouts>,
        >,
        /// Time at which the knowledge base was last updated.
        pub updated_at: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AgentKnowledgeBaseArgs) -> AgentKnowledgeBaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let knowledge_base_configuration_binding = args
            .knowledge_base_configuration
            .get_inner();
        let name_binding = args.name.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let storage_configuration_binding = args.storage_configuration.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:bedrock/agentKnowledgeBase:AgentKnowledgeBase".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "knowledgeBaseConfiguration".into(),
                    value: &knowledge_base_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "storageConfiguration".into(),
                    value: &storage_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "failureReasons".into(),
                },
                register_interface::ResultField {
                    name: "knowledgeBaseConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "storageConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "updatedAt".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AgentKnowledgeBaseResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            failure_reasons: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("failureReasons").unwrap(),
            ),
            knowledge_base_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("knowledgeBaseConfiguration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            storage_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageConfiguration").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatedAt").unwrap(),
            ),
        }
    }
}