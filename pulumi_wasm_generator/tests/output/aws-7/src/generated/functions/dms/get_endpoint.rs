pub mod get_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEndpointArgs {
        /// Database endpoint identifier. Identifiers must contain from 1 to 255 alphanumeric characters or hyphens, begin with a letter, contain only ASCII letters, digits, and hyphens, not end with a hyphen, and not contain two consecutive hyphens.
        #[builder(into)]
        pub endpoint_id: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetEndpointResult {
        pub certificate_arn: pulumi_wasm_rust::Output<String>,
        pub database_name: pulumi_wasm_rust::Output<String>,
        pub elasticsearch_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointElasticsearchSetting>,
        >,
        pub endpoint_arn: pulumi_wasm_rust::Output<String>,
        pub endpoint_id: pulumi_wasm_rust::Output<String>,
        pub endpoint_type: pulumi_wasm_rust::Output<String>,
        pub engine_name: pulumi_wasm_rust::Output<String>,
        pub extra_connection_attributes: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub kafka_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointKafkaSetting>,
        >,
        pub kinesis_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointKinesisSetting>,
        >,
        pub kms_key_arn: pulumi_wasm_rust::Output<String>,
        pub mongodb_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointMongodbSetting>,
        >,
        pub password: pulumi_wasm_rust::Output<String>,
        pub port: pulumi_wasm_rust::Output<i32>,
        pub postgres_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointPostgresSetting>,
        >,
        pub redis_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointRedisSetting>,
        >,
        pub redshift_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointRedshiftSetting>,
        >,
        pub s3_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dms::GetEndpointS3Setting>,
        >,
        pub secrets_manager_access_role_arn: pulumi_wasm_rust::Output<String>,
        pub secrets_manager_arn: pulumi_wasm_rust::Output<String>,
        pub server_name: pulumi_wasm_rust::Output<String>,
        pub service_access_role: pulumi_wasm_rust::Output<String>,
        pub ssl_mode: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub username: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetEndpointArgs,
    ) -> GetEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let endpoint_id_binding = args.endpoint_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:dms/getEndpoint:getEndpoint".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "endpointId".into(),
                    value: &endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificateArn".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "elasticsearchSettings".into(),
                },
                register_interface::ResultField {
                    name: "endpointArn".into(),
                },
                register_interface::ResultField {
                    name: "endpointId".into(),
                },
                register_interface::ResultField {
                    name: "endpointType".into(),
                },
                register_interface::ResultField {
                    name: "engineName".into(),
                },
                register_interface::ResultField {
                    name: "extraConnectionAttributes".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kafkaSettings".into(),
                },
                register_interface::ResultField {
                    name: "kinesisSettings".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "mongodbSettings".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "postgresSettings".into(),
                },
                register_interface::ResultField {
                    name: "redisSettings".into(),
                },
                register_interface::ResultField {
                    name: "redshiftSettings".into(),
                },
                register_interface::ResultField {
                    name: "s3Settings".into(),
                },
                register_interface::ResultField {
                    name: "secretsManagerAccessRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "secretsManagerArn".into(),
                },
                register_interface::ResultField {
                    name: "serverName".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccessRole".into(),
                },
                register_interface::ResultField {
                    name: "sslMode".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "username".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetEndpointResult {
            certificate_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateArn").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            elasticsearch_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticsearchSettings").unwrap(),
            ),
            endpoint_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointArn").unwrap(),
            ),
            endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointId").unwrap(),
            ),
            endpoint_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointType").unwrap(),
            ),
            engine_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineName").unwrap(),
            ),
            extra_connection_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extraConnectionAttributes").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kafka_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kafkaSettings").unwrap(),
            ),
            kinesis_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kinesisSettings").unwrap(),
            ),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyArn").unwrap(),
            ),
            mongodb_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mongodbSettings").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            postgres_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("postgresSettings").unwrap(),
            ),
            redis_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redisSettings").unwrap(),
            ),
            redshift_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redshiftSettings").unwrap(),
            ),
            s3_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3Settings").unwrap(),
            ),
            secrets_manager_access_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretsManagerAccessRoleArn").unwrap(),
            ),
            secrets_manager_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretsManagerArn").unwrap(),
            ),
            server_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverName").unwrap(),
            ),
            service_access_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccessRole").unwrap(),
            ),
            ssl_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslMode").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("username").unwrap(),
            ),
        }
    }
}
