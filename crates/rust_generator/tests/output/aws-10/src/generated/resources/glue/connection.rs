/// Provides a Glue Connection resource.
///
/// ## Example Usage
///
/// ### Non-VPC Connection
///
/// ```yaml
/// resources:
///   example:
///     type: aws:glue:Connection
///     properties:
///       name: example
///       connectionProperties:
///         JDBC_CONNECTION_URL: jdbc:mysql://example.com/exampledatabase
///         PASSWORD: examplepassword
///         USERNAME: exampleusername
/// ```
///
/// ### Non-VPC Connection with secret manager reference
///
/// ```yaml
/// resources:
///   exampleConnection:
///     type: aws:glue:Connection
///     name: example
///     properties:
///       name: example
///       connectionProperties:
///         JDBC_CONNECTION_URL: jdbc:mysql://example.com/exampledatabase
///         SECRET_ID: ${example.name}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:secretsmanager:getSecret
///       arguments:
///         name: example-secret
/// ```
///
/// ### VPC Connection
///
/// For more information, see the [AWS Documentation](https://docs.aws.amazon.com/glue/latest/dg/populate-add-connection.html#connection-JDBC-VPC).
///
/// ```yaml
/// resources:
///   example:
///     type: aws:glue:Connection
///     properties:
///       name: example
///       connectionProperties:
///         JDBC_CONNECTION_URL: jdbc:mysql://${exampleAwsRdsCluster.endpoint}/exampledatabase
///         PASSWORD: examplepassword
///         USERNAME: exampleusername
///       physicalConnectionRequirements:
///         availabilityZone: ${exampleAwsSubnet.availabilityZone}
///         securityGroupIdLists:
///           - ${exampleAwsSecurityGroup.id}
///         subnetId: ${exampleAwsSubnet.id}
/// ```
///
/// ### Connection using a custom connector
///
/// ```yaml
/// resources:
///   example1:
///     type: aws:glue:Connection
///     properties:
///       name: example1
///       connectionType: CUSTOM
///       connectionProperties:
///         CONNECTOR_CLASS_NAME: net.snowflake.client.jdbc.SnowflakeDriver
///         CONNECTION_TYPE: Jdbc
///         CONNECTOR_URL: s3://example/snowflake-jdbc.jar
///         JDBC_CONNECTION_URL: '[["default=jdbc:snowflake://example.com/?user=$${user}&password=$${password}"],","]'
///       matchCriterias:
///         - template-connection
///   # Reference the connector using match_criteria with the connector created above.
///   example2:
///     type: aws:glue:Connection
///     properties:
///       name: example2
///       connectionType: CUSTOM
///       connectionProperties:
///         CONNECTOR_CLASS_NAME: net.snowflake.client.jdbc.SnowflakeDriver
///         CONNECTION_TYPE: Jdbc
///         CONNECTOR_URL: s3://example/snowflake-jdbc.jar
///         JDBC_CONNECTION_URL: jdbc:snowflake://example.com/?user=$${user}&password=$${password}
///         SECRET_ID: ${example.name}
///       matchCriterias:
///         - Connection
///         - ${example1.name}
/// variables:
///   # Define the custom connector using the connection_type of `CUSTOM` with the match_criteria of `template_connection`
///   # Example here being a snowflake jdbc connector with a secret having user and password as keys
///   example:
///     fn::invoke:
///       function: aws:secretsmanager:getSecret
///       arguments:
///         name: example-secret
/// ```
///
/// ### Azure Cosmos Connection
///
/// For more information, see the [AWS Documentation](https://docs.aws.amazon.com/glue/latest/dg/connection-properties.html#connection-properties-azurecosmos).
///
/// ```yaml
/// resources:
///   example:
///     type: aws:secretsmanager:Secret
///     properties:
///       name: example-secret
///   exampleSecretVersion:
///     type: aws:secretsmanager:SecretVersion
///     name: example
///     properties:
///       secretId: ${example.id}
///       secretString:
///         fn::toJSON:
///           username: exampleusername
///           password: examplepassword
///   exampleConnection:
///     type: aws:glue:Connection
///     name: example
///     properties:
///       name: example
///       connectionType: AZURECOSMOS
///       connectionProperties:
///         SparkProperties:
///           fn::toJSON:
///             secretId: ${example.name}
///             spark.cosmos.accountEndpoint: https://exampledbaccount.documents.azure.com:443/
/// ```
///
/// ### Azure SQL Connection
///
/// For more information, see the [AWS Documentation](https://docs.aws.amazon.com/glue/latest/dg/connection-properties.html#connection-properties-azuresql).
///
/// ```yaml
/// resources:
///   example:
///     type: aws:secretsmanager:Secret
///     properties:
///       name: example-secret
///   exampleSecretVersion:
///     type: aws:secretsmanager:SecretVersion
///     name: example
///     properties:
///       secretId: ${example.id}
///       secretString:
///         fn::toJSON:
///           username: exampleusername
///           password: examplepassword
///   exampleConnection:
///     type: aws:glue:Connection
///     name: example
///     properties:
///       name: example
///       connectionType: AZURECOSMOS
///       connectionProperties:
///         SparkProperties:
///           fn::toJSON:
///             secretId: ${example.name}
///             url: jdbc:sqlserver:exampledbserver.database.windows.net:1433;database=exampledatabase
/// ```
///
/// ### Google BigQuery Connection
///
/// For more information, see the [AWS Documentation](https://docs.aws.amazon.com/glue/latest/dg/connection-properties.html#connection-properties-bigquery).
///
/// ```yaml
/// resources:
///   example:
///     type: aws:secretsmanager:Secret
///     properties:
///       name: example-secret
///   exampleSecretVersion:
///     type: aws:secretsmanager:SecretVersion
///     name: example
///     properties:
///       secretId: ${example.id}
///       secretString:
///         fn::toJSON:
///           credentials:
///             fn::invoke:
///               function: std:base64encode
///               arguments:
///                 input: |
///                   {
///                     "type": "service_account",
///                     "project_id": "example-project",
///                     "private_key_id": "example-key",
///                     "private_key": "-----BEGIN RSA PRIVATE KEY-----\nREDACTED\n-----END RSA PRIVATE KEY-----",
///                     "client_email": "example-project@appspot.gserviceaccount.com",
///                     "client_id": example-client",
///                     "auth_uri": "https://accounts.google.com/o/oauth2/auth",
///                     "token_uri": "https://oauth2.googleapis.com/token",
///                     "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
///                     "client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/example-project%%40appspot.gserviceaccount.com",
///                     "universe_domain": "googleapis.com"
///                   }
///               return: result
///   exampleConnection:
///     type: aws:glue:Connection
///     name: example
///     properties:
///       name: example
///       connectionType: BIGQUERY
///       connectionProperties:
///         SparkProperties:
///           fn::toJSON:
///             secretId: ${example.name}
/// ```
///
/// ### OpenSearch Service Connection
///
/// For more information, see the [AWS Documentation](https://docs.aws.amazon.com/glue/latest/dg/connection-properties.html#connection-properties-opensearch).
///
/// ```yaml
/// resources:
///   example:
///     type: aws:secretsmanager:Secret
///     properties:
///       name: example-secret
///   exampleSecretVersion:
///     type: aws:secretsmanager:SecretVersion
///     name: example
///     properties:
///       secretId: ${example.id}
///       secretString:
///         fn::toJSON:
///           opensearch.net.http.auth.user: exampleusername
///           opensearch.net.http.auth.pass: examplepassword
///   exampleConnection:
///     type: aws:glue:Connection
///     name: example
///     properties:
///       name: example
///       connectionType: OPENSEARCH
///       connectionProperties:
///         SparkProperties:
///           fn::toJSON:
///             secretId: ${example.name}
///             opensearch.nodes: https://search-exampledomain-ixlmh4jieahrau3bfebcgp8cnm.us-east-1.es.amazonaws.com
///             opensearch.port: '443'
///             opensearch.aws.sigv4.region: us-east-1
///             opensearch.nodes.wan.only: 'true'
///             opensearch.aws.sigv4.enabled: 'true'
/// ```
///
/// ### Snowflake Connection
///
/// For more information, see the [AWS Documentation](https://docs.aws.amazon.com/glue/latest/dg/connection-properties.html#connection-properties-snowflake).
///
/// ```yaml
/// resources:
///   example:
///     type: aws:secretsmanager:Secret
///     properties:
///       name: example-secret
///   exampleSecretVersion:
///     type: aws:secretsmanager:SecretVersion
///     name: example
///     properties:
///       secretId: ${example.id}
///       secretString:
///         fn::toJSON:
///           sfUser: exampleusername
///           sfPassword: examplepassword
///   exampleConnection:
///     type: aws:glue:Connection
///     name: example
///     properties:
///       name: example
///       connectionType: SNOWFLAKE
///       connectionProperties:
///         SparkProperties:
///           fn::toJSON:
///             secretId: ${example.name}
///             sfRole: EXAMPLEETLROLE
///             sfUrl: exampleorg-exampleconnection.snowflakecomputing.com
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Connections using the `CATALOG-ID` (AWS account ID if not custom) and `NAME`. For example:
///
/// ```sh
/// $ pulumi import aws:glue/connection:Connection MyConnection 123456789012:MyConnection
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        /// ID of the Data Catalog in which to create the connection. If none is supplied, the AWS account ID is used by default.
        #[builder(into, default)]
        pub catalog_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of key-value pairs used as parameters for this connection. For more information, see the [AWS Documentation](https://docs.aws.amazon.com/glue/latest/dg/connection-properties.html).
        ///
        /// **Note:** Some connection types require the `SparkProperties` property with a JSON document that contains the actual connection properties. For specific examples, refer to Example Usage.
        #[builder(into, default)]
        pub connection_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of the connection. Valid values: `AZURECOSMOS`, `AZURESQL`, `BIGQUERY`, `CUSTOM`, `JDBC`, `KAFKA`, `MARKETPLACE`, `MONGODB`, `NETWORK`, `OPENSEARCH`, `SNOWFLAKE`. Defaults to `JDBC`.
        #[builder(into, default)]
        pub connection_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description of the connection.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of criteria that can be used in selecting this connection.
        #[builder(into, default)]
        pub match_criterias: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the connection.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of physical connection requirements, such as VPC and SecurityGroup. See `physical_connection_requirements` Block for details.
        #[builder(into, default)]
        pub physical_connection_requirements: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::ConnectionPhysicalConnectionRequirements>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        /// ARN of the Glue Connection.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the Data Catalog in which to create the connection. If none is supplied, the AWS account ID is used by default.
        pub catalog_id: pulumi_gestalt_rust::Output<String>,
        /// Map of key-value pairs used as parameters for this connection. For more information, see the [AWS Documentation](https://docs.aws.amazon.com/glue/latest/dg/connection-properties.html).
        ///
        /// **Note:** Some connection types require the `SparkProperties` property with a JSON document that contains the actual connection properties. For specific examples, refer to Example Usage.
        pub connection_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of the connection. Valid values: `AZURECOSMOS`, `AZURESQL`, `BIGQUERY`, `CUSTOM`, `JDBC`, `KAFKA`, `MARKETPLACE`, `MONGODB`, `NETWORK`, `OPENSEARCH`, `SNOWFLAKE`. Defaults to `JDBC`.
        pub connection_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Description of the connection.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of criteria that can be used in selecting this connection.
        pub match_criterias: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Name of the connection.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Map of physical connection requirements, such as VPC and SecurityGroup. See `physical_connection_requirements` Block for details.
        pub physical_connection_requirements: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::ConnectionPhysicalConnectionRequirements>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectionArgs,
    ) -> ConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let catalog_id_binding = args.catalog_id.get_output(context);
        let connection_properties_binding = args
            .connection_properties
            .get_output(context);
        let connection_type_binding = args.connection_type.get_output(context);
        let description_binding = args.description.get_output(context);
        let match_criterias_binding = args.match_criterias.get_output(context);
        let name_binding = args.name.get_output(context);
        let physical_connection_requirements_binding = args
            .physical_connection_requirements
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/connection:Connection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionProperties".into(),
                    value: &connection_properties_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionType".into(),
                    value: &connection_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "matchCriterias".into(),
                    value: &match_criterias_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "physicalConnectionRequirements".into(),
                    value: &physical_connection_requirements_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectionResult {
            arn: o.get_field("arn"),
            catalog_id: o.get_field("catalogId"),
            connection_properties: o.get_field("connectionProperties"),
            connection_type: o.get_field("connectionType"),
            description: o.get_field("description"),
            match_criterias: o.get_field("matchCriterias"),
            name: o.get_field("name"),
            physical_connection_requirements: o
                .get_field("physicalConnectionRequirements"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
