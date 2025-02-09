#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEndpointArgs {
        /// Database endpoint identifier. Identifiers must contain from 1 to 255 alphanumeric characters or hyphens, begin with a letter, contain only ASCII letters, digits, and hyphens, not end with a hyphen, and not contain two consecutive hyphens.
        #[builder(into)]
        pub endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetEndpointResult {
        pub certificate_arn: pulumi_gestalt_rust::Output<String>,
        pub database_name: pulumi_gestalt_rust::Output<String>,
        pub elasticsearch_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointElasticsearchSetting>,
        >,
        pub endpoint_arn: pulumi_gestalt_rust::Output<String>,
        pub endpoint_id: pulumi_gestalt_rust::Output<String>,
        pub endpoint_type: pulumi_gestalt_rust::Output<String>,
        pub engine_name: pulumi_gestalt_rust::Output<String>,
        pub extra_connection_attributes: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub kafka_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointKafkaSetting>,
        >,
        pub kinesis_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointKinesisSetting>,
        >,
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
        pub mongodb_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointMongodbSetting>,
        >,
        pub password: pulumi_gestalt_rust::Output<String>,
        pub port: pulumi_gestalt_rust::Output<i32>,
        pub postgres_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointPostgresSetting>,
        >,
        pub redis_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointRedisSetting>,
        >,
        pub redshift_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointRedshiftSetting>,
        >,
        pub s3_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointS3Setting>,
        >,
        pub secrets_manager_access_role_arn: pulumi_gestalt_rust::Output<String>,
        pub secrets_manager_arn: pulumi_gestalt_rust::Output<String>,
        pub server_name: pulumi_gestalt_rust::Output<String>,
        pub service_access_role: pulumi_gestalt_rust::Output<String>,
        pub ssl_mode: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub username: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetEndpointArgs,
    ) -> GetEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let endpoint_id_binding = args.endpoint_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:dms/getEndpoint:getEndpoint".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointId".into(),
                    value: endpoint_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetEndpointResult {
            certificate_arn: o.get_field("certificateArn"),
            database_name: o.get_field("databaseName"),
            elasticsearch_settings: o.get_field("elasticsearchSettings"),
            endpoint_arn: o.get_field("endpointArn"),
            endpoint_id: o.get_field("endpointId"),
            endpoint_type: o.get_field("endpointType"),
            engine_name: o.get_field("engineName"),
            extra_connection_attributes: o.get_field("extraConnectionAttributes"),
            id: o.get_field("id"),
            kafka_settings: o.get_field("kafkaSettings"),
            kinesis_settings: o.get_field("kinesisSettings"),
            kms_key_arn: o.get_field("kmsKeyArn"),
            mongodb_settings: o.get_field("mongodbSettings"),
            password: o.get_field("password"),
            port: o.get_field("port"),
            postgres_settings: o.get_field("postgresSettings"),
            redis_settings: o.get_field("redisSettings"),
            redshift_settings: o.get_field("redshiftSettings"),
            s3_settings: o.get_field("s3Settings"),
            secrets_manager_access_role_arn: o.get_field("secretsManagerAccessRoleArn"),
            secrets_manager_arn: o.get_field("secretsManagerArn"),
            server_name: o.get_field("serverName"),
            service_access_role: o.get_field("serviceAccessRole"),
            ssl_mode: o.get_field("sslMode"),
            tags: o.get_field("tags"),
            username: o.get_field("username"),
        }
    }
}
