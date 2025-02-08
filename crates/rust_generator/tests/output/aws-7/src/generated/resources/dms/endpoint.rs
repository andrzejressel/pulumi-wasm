/// Provides a DMS (Data Migration Service) endpoint resource. DMS endpoints can be created, updated, deleted, and imported.
///
/// > **Note:** All arguments including the password will be stored in the raw state as plain-text. > **Note:** The `s3_settings` argument is deprecated, may not be maintained, and will be removed in a future version. Use the `aws.dms.S3Endpoint` resource instead.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   # Create a new endpoint
///   test:
///     type: aws:dms:Endpoint
///     properties:
///       certificateArn: arn:aws:acm:us-east-1:123456789012:certificate/12345678-1234-1234-1234-123456789012
///       databaseName: test
///       endpointId: test-dms-endpoint-tf
///       endpointType: source
///       engineName: aurora
///       extraConnectionAttributes: ""
///       kmsKeyArn: arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
///       password: test
///       port: 3306
///       serverName: test
///       sslMode: none
///       tags:
///         Name: test
///       username: test
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import endpoints using the `endpoint_id`. For example:
///
/// ```sh
/// $ pulumi import aws:dms/endpoint:Endpoint test test-dms-endpoint-tf
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointArgs {
        /// ARN for the certificate.
        #[builder(into, default)]
        pub certificate_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the endpoint database.
        #[builder(into, default)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for OpenSearch settings. See below.
        #[builder(into, default)]
        pub elasticsearch_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dms::EndpointElasticsearchSettings>,
        >,
        /// Database endpoint identifier. Identifiers must contain from 1 to 255 alphanumeric characters or hyphens, begin with a letter, contain only ASCII letters, digits, and hyphens, not end with a hyphen, and not contain two consecutive hyphens.
        #[builder(into)]
        pub endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of endpoint. Valid values are `source`, `target`.
        #[builder(into)]
        pub endpoint_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of engine for the endpoint. Valid values are `aurora`, `aurora-postgresql`, `aurora-serverless`, `aurora-postgresql-serverless`,`azuredb`, `azure-sql-managed-instance`, `babelfish`, `db2`, `db2-zos`, `docdb`, `dynamodb`, `elasticsearch`, `kafka`, `kinesis`, `mariadb`, `mongodb`, `mysql`, `opensearch`, `oracle`, `postgres`, `redshift`,`redshift-serverless`, `s3`, `sqlserver`, `neptune` ,`sybase`. Please note that some of engine names are available only for `target` endpoint type (e.g. `redshift`).
        #[builder(into)]
        pub engine_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Additional attributes associated with the connection. For available attributes for a `source` Endpoint, see [Sources for data migration](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.html). For available attributes for a `target` Endpoint, see [Targets for data migration](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.html).
        #[builder(into, default)]
        pub extra_connection_attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Configuration block for Kafka settings. See below.
        #[builder(into, default)]
        pub kafka_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dms::EndpointKafkaSettings>,
        >,
        /// Configuration block for Kinesis settings. See below.
        #[builder(into, default)]
        pub kinesis_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dms::EndpointKinesisSettings>,
        >,
        /// ARN for the KMS key that will be used to encrypt the connection parameters. If you do not specify a value for `kms_key_arn`, then AWS DMS will use your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS region. To encrypt an S3 target with a KMS Key, use the parameter `s3_settings.server_side_encryption_kms_key_id`. When `engine_name` is `redshift`, `kms_key_arn` is the KMS Key for the Redshift target and the parameter `redshift_settings.server_side_encryption_kms_key_id` encrypts the S3 intermediate storage.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub kms_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for MongoDB settings. See below.
        #[builder(into, default)]
        pub mongodb_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dms::EndpointMongodbSettings>,
        >,
        /// Password to be used to login to the endpoint database.
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub pause_replication_tasks: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Port used by the endpoint database.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Configuration block for Postgres settings. See below.
        #[builder(into, default)]
        pub postgres_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dms::EndpointPostgresSettings>,
        >,
        #[builder(into, default)]
        pub redis_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dms::EndpointRedisSettings>,
        >,
        /// Configuration block for Redshift settings. See below.
        #[builder(into, default)]
        pub redshift_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dms::EndpointRedshiftSettings>,
        >,
        /// (**Deprecated**, use the `aws.dms.S3Endpoint` resource instead) Configuration block for S3 settings. See below.
        #[builder(into, default)]
        pub s3_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dms::EndpointS3Settings>,
        >,
        /// ARN of the IAM role that specifies AWS DMS as the trusted entity and has the required permissions to access the value in the Secrets Manager secret referred to by `secrets_manager_arn`. The role must allow the `iam:PassRole` action.
        ///
        /// > **Note:** You can specify one of two sets of values for these permissions. You can specify the values for this setting and `secrets_manager_arn`. Or you can specify clear-text values for `username`, `password` , `server_name`, and `port`. You can't specify both.
        #[builder(into, default)]
        pub secrets_manager_access_role_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Full ARN, partial ARN, or friendly name of the Secrets Manager secret that contains the endpoint connection details. Supported only when `engine_name` is `aurora`, `aurora-postgresql`, `mariadb`, `mongodb`, `mysql`, `oracle`, `postgres`, `redshift`, or `sqlserver`.
        #[builder(into, default)]
        pub secrets_manager_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Host name of the server.
        #[builder(into, default)]
        pub server_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN used by the service access IAM role for dynamodb endpoints.
        #[builder(into, default)]
        pub service_access_role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// SSL mode to use for the connection. Valid values are `none`, `require`, `verify-ca`, `verify-full`
        #[builder(into, default)]
        pub ssl_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// User name to be used to login to the endpoint database.
        #[builder(into, default)]
        pub username: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EndpointResult {
        /// ARN for the certificate.
        pub certificate_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the endpoint database.
        pub database_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block for OpenSearch settings. See below.
        pub elasticsearch_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::dms::EndpointElasticsearchSettings>,
        >,
        /// ARN for the endpoint.
        pub endpoint_arn: pulumi_gestalt_rust::Output<String>,
        /// Database endpoint identifier. Identifiers must contain from 1 to 255 alphanumeric characters or hyphens, begin with a letter, contain only ASCII letters, digits, and hyphens, not end with a hyphen, and not contain two consecutive hyphens.
        pub endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// Type of endpoint. Valid values are `source`, `target`.
        pub endpoint_type: pulumi_gestalt_rust::Output<String>,
        /// Type of engine for the endpoint. Valid values are `aurora`, `aurora-postgresql`, `aurora-serverless`, `aurora-postgresql-serverless`,`azuredb`, `azure-sql-managed-instance`, `babelfish`, `db2`, `db2-zos`, `docdb`, `dynamodb`, `elasticsearch`, `kafka`, `kinesis`, `mariadb`, `mongodb`, `mysql`, `opensearch`, `oracle`, `postgres`, `redshift`,`redshift-serverless`, `s3`, `sqlserver`, `neptune` ,`sybase`. Please note that some of engine names are available only for `target` endpoint type (e.g. `redshift`).
        pub engine_name: pulumi_gestalt_rust::Output<String>,
        /// Additional attributes associated with the connection. For available attributes for a `source` Endpoint, see [Sources for data migration](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.html). For available attributes for a `target` Endpoint, see [Targets for data migration](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.html).
        pub extra_connection_attributes: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for Kafka settings. See below.
        pub kafka_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::dms::EndpointKafkaSettings>,
        >,
        /// Configuration block for Kinesis settings. See below.
        pub kinesis_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::dms::EndpointKinesisSettings>,
        >,
        /// ARN for the KMS key that will be used to encrypt the connection parameters. If you do not specify a value for `kms_key_arn`, then AWS DMS will use your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS region. To encrypt an S3 target with a KMS Key, use the parameter `s3_settings.server_side_encryption_kms_key_id`. When `engine_name` is `redshift`, `kms_key_arn` is the KMS Key for the Redshift target and the parameter `redshift_settings.server_side_encryption_kms_key_id` encrypts the S3 intermediate storage.
        ///
        /// The following arguments are optional:
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for MongoDB settings. See below.
        pub mongodb_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::dms::EndpointMongodbSettings>,
        >,
        /// Password to be used to login to the endpoint database.
        pub password: pulumi_gestalt_rust::Output<Option<String>>,
        pub pause_replication_tasks: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Port used by the endpoint database.
        pub port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Configuration block for Postgres settings. See below.
        pub postgres_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::dms::EndpointPostgresSettings>,
        >,
        pub redis_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::dms::EndpointRedisSettings>,
        >,
        /// Configuration block for Redshift settings. See below.
        pub redshift_settings: pulumi_gestalt_rust::Output<
            super::super::types::dms::EndpointRedshiftSettings,
        >,
        /// (**Deprecated**, use the `aws.dms.S3Endpoint` resource instead) Configuration block for S3 settings. See below.
        pub s3_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::dms::EndpointS3Settings>,
        >,
        /// ARN of the IAM role that specifies AWS DMS as the trusted entity and has the required permissions to access the value in the Secrets Manager secret referred to by `secrets_manager_arn`. The role must allow the `iam:PassRole` action.
        ///
        /// > **Note:** You can specify one of two sets of values for these permissions. You can specify the values for this setting and `secrets_manager_arn`. Or you can specify clear-text values for `username`, `password` , `server_name`, and `port`. You can't specify both.
        pub secrets_manager_access_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Full ARN, partial ARN, or friendly name of the Secrets Manager secret that contains the endpoint connection details. Supported only when `engine_name` is `aurora`, `aurora-postgresql`, `mariadb`, `mongodb`, `mysql`, `oracle`, `postgres`, `redshift`, or `sqlserver`.
        pub secrets_manager_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Host name of the server.
        pub server_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN used by the service access IAM role for dynamodb endpoints.
        pub service_access_role: pulumi_gestalt_rust::Output<Option<String>>,
        /// SSL mode to use for the connection. Valid values are `none`, `require`, `verify-ca`, `verify-full`
        pub ssl_mode: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// User name to be used to login to the endpoint database.
        pub username: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EndpointArgs,
    ) -> EndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let certificate_arn_binding = args
            .certificate_arn
            .get_output(context)
            .get_inner();
        let database_name_binding = args.database_name.get_output(context).get_inner();
        let elasticsearch_settings_binding = args
            .elasticsearch_settings
            .get_output(context)
            .get_inner();
        let endpoint_id_binding = args.endpoint_id.get_output(context).get_inner();
        let endpoint_type_binding = args.endpoint_type.get_output(context).get_inner();
        let engine_name_binding = args.engine_name.get_output(context).get_inner();
        let extra_connection_attributes_binding = args
            .extra_connection_attributes
            .get_output(context)
            .get_inner();
        let kafka_settings_binding = args.kafka_settings.get_output(context).get_inner();
        let kinesis_settings_binding = args
            .kinesis_settings
            .get_output(context)
            .get_inner();
        let kms_key_arn_binding = args.kms_key_arn.get_output(context).get_inner();
        let mongodb_settings_binding = args
            .mongodb_settings
            .get_output(context)
            .get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let pause_replication_tasks_binding = args
            .pause_replication_tasks
            .get_output(context)
            .get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let postgres_settings_binding = args
            .postgres_settings
            .get_output(context)
            .get_inner();
        let redis_settings_binding = args.redis_settings.get_output(context).get_inner();
        let redshift_settings_binding = args
            .redshift_settings
            .get_output(context)
            .get_inner();
        let s3_settings_binding = args.s3_settings.get_output(context).get_inner();
        let secrets_manager_access_role_arn_binding = args
            .secrets_manager_access_role_arn
            .get_output(context)
            .get_inner();
        let secrets_manager_arn_binding = args
            .secrets_manager_arn
            .get_output(context)
            .get_inner();
        let server_name_binding = args.server_name.get_output(context).get_inner();
        let service_access_role_binding = args
            .service_access_role
            .get_output(context)
            .get_inner();
        let ssl_mode_binding = args.ssl_mode.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let username_binding = args.username.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dms/endpoint:Endpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateArn".into(),
                    value: &certificate_arn_binding,
                },
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "elasticsearchSettings".into(),
                    value: &elasticsearch_settings_binding,
                },
                register_interface::ObjectField {
                    name: "endpointId".into(),
                    value: &endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "endpointType".into(),
                    value: &endpoint_type_binding,
                },
                register_interface::ObjectField {
                    name: "engineName".into(),
                    value: &engine_name_binding,
                },
                register_interface::ObjectField {
                    name: "extraConnectionAttributes".into(),
                    value: &extra_connection_attributes_binding,
                },
                register_interface::ObjectField {
                    name: "kafkaSettings".into(),
                    value: &kafka_settings_binding,
                },
                register_interface::ObjectField {
                    name: "kinesisSettings".into(),
                    value: &kinesis_settings_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyArn".into(),
                    value: &kms_key_arn_binding,
                },
                register_interface::ObjectField {
                    name: "mongodbSettings".into(),
                    value: &mongodb_settings_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "pauseReplicationTasks".into(),
                    value: &pause_replication_tasks_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "postgresSettings".into(),
                    value: &postgres_settings_binding,
                },
                register_interface::ObjectField {
                    name: "redisSettings".into(),
                    value: &redis_settings_binding,
                },
                register_interface::ObjectField {
                    name: "redshiftSettings".into(),
                    value: &redshift_settings_binding,
                },
                register_interface::ObjectField {
                    name: "s3Settings".into(),
                    value: &s3_settings_binding,
                },
                register_interface::ObjectField {
                    name: "secretsManagerAccessRoleArn".into(),
                    value: &secrets_manager_access_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "secretsManagerArn".into(),
                    value: &secrets_manager_arn_binding,
                },
                register_interface::ObjectField {
                    name: "serverName".into(),
                    value: &server_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccessRole".into(),
                    value: &service_access_role_binding,
                },
                register_interface::ObjectField {
                    name: "sslMode".into(),
                    value: &ssl_mode_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "username".into(),
                    value: &username_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EndpointResult {
            certificate_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateArn"),
            ),
            database_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseName"),
            ),
            elasticsearch_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("elasticsearchSettings"),
            ),
            endpoint_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointArn"),
            ),
            endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointId"),
            ),
            endpoint_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointType"),
            ),
            engine_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineName"),
            ),
            extra_connection_attributes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("extraConnectionAttributes"),
            ),
            kafka_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kafkaSettings"),
            ),
            kinesis_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kinesisSettings"),
            ),
            kms_key_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyArn"),
            ),
            mongodb_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mongodbSettings"),
            ),
            password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            pause_replication_tasks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pauseReplicationTasks"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            postgres_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("postgresSettings"),
            ),
            redis_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("redisSettings"),
            ),
            redshift_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("redshiftSettings"),
            ),
            s3_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("s3Settings"),
            ),
            secrets_manager_access_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secretsManagerAccessRoleArn"),
            ),
            secrets_manager_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secretsManagerArn"),
            ),
            server_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverName"),
            ),
            service_access_role: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceAccessRole"),
            ),
            ssl_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sslMode"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            username: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("username"),
            ),
        }
    }
}
