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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgentKnowledgeBaseArgs {
        /// Description of the knowledge base.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Details about the embeddings configuration of the knowledge base. See `knowledge_base_configuration` block for details.
        #[builder(into, default)]
        pub knowledge_base_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::bedrock::AgentKnowledgeBaseKnowledgeBaseConfiguration,
            >,
        >,
        /// Name of the knowledge base.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the IAM role with permissions to invoke API operations on the knowledge base.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Details about the storage configuration of the knowledge base. See `storage_configuration` block for details.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub storage_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::AgentKnowledgeBaseStorageConfiguration>,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::AgentKnowledgeBaseTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct AgentKnowledgeBaseResult {
        /// ARN of the knowledge base.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Time at which the knowledge base was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Description of the knowledge base.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        pub failure_reasons: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Details about the embeddings configuration of the knowledge base. See `knowledge_base_configuration` block for details.
        pub knowledge_base_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::bedrock::AgentKnowledgeBaseKnowledgeBaseConfiguration,
            >,
        >,
        /// Name of the knowledge base.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM role with permissions to invoke API operations on the knowledge base.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Details about the storage configuration of the knowledge base. See `storage_configuration` block for details.
        ///
        /// The following arguments are optional:
        pub storage_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::AgentKnowledgeBaseStorageConfiguration>,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::AgentKnowledgeBaseTimeouts>,
        >,
        /// Time at which the knowledge base was last updated.
        pub updated_at: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AgentKnowledgeBaseArgs,
    ) -> AgentKnowledgeBaseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let knowledge_base_configuration_binding = args
            .knowledge_base_configuration
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let storage_configuration_binding = args
            .storage_configuration
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:bedrock/agentKnowledgeBase:AgentKnowledgeBase".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        AgentKnowledgeBaseResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            created_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            failure_reasons: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("failureReasons"),
            ),
            knowledge_base_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("knowledgeBaseConfiguration"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            storage_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageConfiguration"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
            updated_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updatedAt"),
            ),
        }
    }
}
