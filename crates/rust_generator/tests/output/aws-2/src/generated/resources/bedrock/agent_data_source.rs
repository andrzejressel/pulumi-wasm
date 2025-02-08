/// Resource for managing an AWS Agents for Amazon Bedrock Data Source.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:bedrock:AgentDataSource
///     properties:
///       knowledgeBaseId: EMDPPAYPZI
///       name: example
///       dataSourceConfiguration:
///         type: S3
///         s3Configuration:
///           bucketArn: arn:aws:s3:::example-bucket
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Agents for Amazon Bedrock Data Source using the data source ID and the knowledge base ID. For example:
///
/// ```sh
/// $ pulumi import aws:bedrock/agentDataSource:AgentDataSource example GWCMFMQF6T,EMDPPAYPZI
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod agent_data_source {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgentDataSourceArgs {
        /// Data deletion policy for a data source. Valid values: `RETAIN`, `DELETE`.
        #[builder(into, default)]
        pub data_deletion_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Details about how the data source is stored. See `data_source_configuration` block for details.
        #[builder(into, default)]
        pub data_source_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::AgentDataSourceDataSourceConfiguration>,
        >,
        /// Description of the data source.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique identifier of the knowledge base to which the data source belongs.
        #[builder(into)]
        pub knowledge_base_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the data source.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Details about the configuration of the server-side encryption. See `server_side_encryption_configuration` block for details.
        #[builder(into, default)]
        pub server_side_encryption_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::bedrock::AgentDataSourceServerSideEncryptionConfiguration,
            >,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::AgentDataSourceTimeouts>,
        >,
        /// Details about the configuration of the server-side encryption. See `vector_ingestion_configuration` block for details.
        #[builder(into, default)]
        pub vector_ingestion_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::bedrock::AgentDataSourceVectorIngestionConfiguration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct AgentDataSourceResult {
        /// Data deletion policy for a data source. Valid values: `RETAIN`, `DELETE`.
        pub data_deletion_policy: pulumi_gestalt_rust::Output<String>,
        /// Details about how the data source is stored. See `data_source_configuration` block for details.
        pub data_source_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::AgentDataSourceDataSourceConfiguration>,
        >,
        /// Unique identifier of the data source.
        pub data_source_id: pulumi_gestalt_rust::Output<String>,
        /// Description of the data source.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Unique identifier of the knowledge base to which the data source belongs.
        pub knowledge_base_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the data source.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Details about the configuration of the server-side encryption. See `server_side_encryption_configuration` block for details.
        pub server_side_encryption_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::bedrock::AgentDataSourceServerSideEncryptionConfiguration,
            >,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::AgentDataSourceTimeouts>,
        >,
        /// Details about the configuration of the server-side encryption. See `vector_ingestion_configuration` block for details.
        pub vector_ingestion_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::bedrock::AgentDataSourceVectorIngestionConfiguration,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AgentDataSourceArgs,
    ) -> AgentDataSourceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let data_deletion_policy_binding = args
            .data_deletion_policy
            .get_output(context)
            .get_inner();
        let data_source_configuration_binding = args
            .data_source_configuration
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let knowledge_base_id_binding = args
            .knowledge_base_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let server_side_encryption_configuration_binding = args
            .server_side_encryption_configuration
            .get_output(context)
            .get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let vector_ingestion_configuration_binding = args
            .vector_ingestion_configuration
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:bedrock/agentDataSource:AgentDataSource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataDeletionPolicy".into(),
                    value: &data_deletion_policy_binding,
                },
                register_interface::ObjectField {
                    name: "dataSourceConfiguration".into(),
                    value: &data_source_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "knowledgeBaseId".into(),
                    value: &knowledge_base_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "serverSideEncryptionConfiguration".into(),
                    value: &server_side_encryption_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "vectorIngestionConfiguration".into(),
                    value: &vector_ingestion_configuration_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AgentDataSourceResult {
            data_deletion_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataDeletionPolicy"),
            ),
            data_source_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataSourceConfiguration"),
            ),
            data_source_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataSourceId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            knowledge_base_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("knowledgeBaseId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            server_side_encryption_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverSideEncryptionConfiguration"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
            vector_ingestion_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vectorIngestionConfiguration"),
            ),
        }
    }
}
