/// Provides a Glue Development Endpoint resource.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   exampleDevEndpoint:
///     type: aws:glue:DevEndpoint
///     name: example
///     properties:
///       name: foo
///       roleArn: ${exampleRole.arn}
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: AWSGlueServiceRole-foo
///       assumeRolePolicy: ${example.json}
///   example-AWSGlueServiceRole:
///     type: aws:iam:RolePolicyAttachment
///     properties:
///       policyArn: arn:aws:iam::aws:policy/service-role/AWSGlueServiceRole
///       role: ${exampleRole.name}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             principals:
///               - type: Service
///                 identifiers:
///                   - glue.amazonaws.com
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a Glue Development Endpoint using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:glue/devEndpoint:DevEndpoint example foo
/// ```
pub mod dev_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DevEndpointArgs {
        /// A map of arguments used to configure the endpoint.
        #[builder(into, default)]
        pub arguments: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Path to one or more Java Jars in an S3 bucket that should be loaded in this endpoint.
        #[builder(into, default)]
        pub extra_jars_s3_path: pulumi_wasm_rust::Output<Option<String>>,
        /// Path(s) to one or more Python libraries in an S3 bucket that should be loaded in this endpoint. Multiple values must be complete paths separated by a comma.
        #[builder(into, default)]
        pub extra_python_libs_s3_path: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the versions of Python and Apache Spark to use. Defaults to AWS Glue version 0.9.
        #[builder(into, default)]
        pub glue_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of this endpoint. It must be unique in your account.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of AWS Glue Data Processing Units (DPUs) to allocate to this endpoint. Conflicts with `worker_type`.
        #[builder(into, default)]
        pub number_of_nodes: pulumi_wasm_rust::Output<Option<i32>>,
        /// The number of workers of a defined worker type that are allocated to this endpoint. This field is available only when you choose worker type G.1X or G.2X.
        #[builder(into, default)]
        pub number_of_workers: pulumi_wasm_rust::Output<Option<i32>>,
        /// The public key to be used by this endpoint for authentication.
        #[builder(into, default)]
        pub public_key: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of public keys to be used by this endpoint for authentication.
        #[builder(into, default)]
        pub public_keys: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The IAM role for this endpoint.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the Security Configuration structure to be used with this endpoint.
        #[builder(into, default)]
        pub security_configuration: pulumi_wasm_rust::Output<Option<String>>,
        /// Security group IDs for the security groups to be used by this endpoint.
        #[builder(into, default)]
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The subnet ID for the new endpoint to use.
        #[builder(into, default)]
        pub subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of predefined worker that is allocated to this endpoint. Accepts a value of Standard, G.1X, or G.2X.
        #[builder(into, default)]
        pub worker_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DevEndpointResult {
        /// A map of arguments used to configure the endpoint.
        pub arguments: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ARN of the endpoint.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The AWS availability zone where this endpoint is located.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// Path to one or more Java Jars in an S3 bucket that should be loaded in this endpoint.
        pub extra_jars_s3_path: pulumi_wasm_rust::Output<Option<String>>,
        /// Path(s) to one or more Python libraries in an S3 bucket that should be loaded in this endpoint. Multiple values must be complete paths separated by a comma.
        pub extra_python_libs_s3_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The reason for a current failure in this endpoint.
        pub failure_reason: pulumi_wasm_rust::Output<String>,
        /// Specifies the versions of Python and Apache Spark to use. Defaults to AWS Glue version 0.9.
        pub glue_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of this endpoint. It must be unique in your account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The number of AWS Glue Data Processing Units (DPUs) to allocate to this endpoint. Conflicts with `worker_type`.
        pub number_of_nodes: pulumi_wasm_rust::Output<Option<i32>>,
        /// The number of workers of a defined worker type that are allocated to this endpoint. This field is available only when you choose worker type G.1X or G.2X.
        pub number_of_workers: pulumi_wasm_rust::Output<Option<i32>>,
        /// A private IP address to access the endpoint within a VPC, if this endpoint is created within one.
        pub private_address: pulumi_wasm_rust::Output<String>,
        /// The public IP address used by this endpoint. The PublicAddress field is present only when you create a non-VPC endpoint.
        pub public_address: pulumi_wasm_rust::Output<String>,
        /// The public key to be used by this endpoint for authentication.
        pub public_key: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of public keys to be used by this endpoint for authentication.
        pub public_keys: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The IAM role for this endpoint.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the Security Configuration structure to be used with this endpoint.
        pub security_configuration: pulumi_wasm_rust::Output<Option<String>>,
        /// Security group IDs for the security groups to be used by this endpoint.
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The current status of this endpoint.
        pub status: pulumi_wasm_rust::Output<String>,
        /// The subnet ID for the new endpoint to use.
        pub subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// he ID of the VPC used by this endpoint.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
        /// The type of predefined worker that is allocated to this endpoint. Accepts a value of Standard, G.1X, or G.2X.
        pub worker_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The YARN endpoint address used by this endpoint.
        pub yarn_endpoint_address: pulumi_wasm_rust::Output<String>,
        /// The Apache Zeppelin port for the remote Apache Spark interpreter.
        pub zeppelin_remote_spark_interpreter_port: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DevEndpointArgs) -> DevEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arguments_binding = args.arguments.get_inner();
        let extra_jars_s3_path_binding = args.extra_jars_s3_path.get_inner();
        let extra_python_libs_s3_path_binding = args
            .extra_python_libs_s3_path
            .get_inner();
        let glue_version_binding = args.glue_version.get_inner();
        let name_binding = args.name.get_inner();
        let number_of_nodes_binding = args.number_of_nodes.get_inner();
        let number_of_workers_binding = args.number_of_workers.get_inner();
        let public_key_binding = args.public_key.get_inner();
        let public_keys_binding = args.public_keys.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let security_configuration_binding = args.security_configuration.get_inner();
        let security_group_ids_binding = args.security_group_ids.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let worker_type_binding = args.worker_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/devEndpoint:DevEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arguments".into(),
                    value: &arguments_binding,
                },
                register_interface::ObjectField {
                    name: "extraJarsS3Path".into(),
                    value: &extra_jars_s3_path_binding,
                },
                register_interface::ObjectField {
                    name: "extraPythonLibsS3Path".into(),
                    value: &extra_python_libs_s3_path_binding,
                },
                register_interface::ObjectField {
                    name: "glueVersion".into(),
                    value: &glue_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "numberOfNodes".into(),
                    value: &number_of_nodes_binding,
                },
                register_interface::ObjectField {
                    name: "numberOfWorkers".into(),
                    value: &number_of_workers_binding,
                },
                register_interface::ObjectField {
                    name: "publicKey".into(),
                    value: &public_key_binding,
                },
                register_interface::ObjectField {
                    name: "publicKeys".into(),
                    value: &public_keys_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "securityConfiguration".into(),
                    value: &security_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workerType".into(),
                    value: &worker_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arguments".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "extraJarsS3Path".into(),
                },
                register_interface::ResultField {
                    name: "extraPythonLibsS3Path".into(),
                },
                register_interface::ResultField {
                    name: "failureReason".into(),
                },
                register_interface::ResultField {
                    name: "glueVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "numberOfNodes".into(),
                },
                register_interface::ResultField {
                    name: "numberOfWorkers".into(),
                },
                register_interface::ResultField {
                    name: "privateAddress".into(),
                },
                register_interface::ResultField {
                    name: "publicAddress".into(),
                },
                register_interface::ResultField {
                    name: "publicKey".into(),
                },
                register_interface::ResultField {
                    name: "publicKeys".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "securityConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
                register_interface::ResultField {
                    name: "workerType".into(),
                },
                register_interface::ResultField {
                    name: "yarnEndpointAddress".into(),
                },
                register_interface::ResultField {
                    name: "zeppelinRemoteSparkInterpreterPort".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DevEndpointResult {
            arguments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arguments").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            extra_jars_s3_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extraJarsS3Path").unwrap(),
            ),
            extra_python_libs_s3_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extraPythonLibsS3Path").unwrap(),
            ),
            failure_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("failureReason").unwrap(),
            ),
            glue_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("glueVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            number_of_nodes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numberOfNodes").unwrap(),
            ),
            number_of_workers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numberOfWorkers").unwrap(),
            ),
            private_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateAddress").unwrap(),
            ),
            public_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicAddress").unwrap(),
            ),
            public_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicKey").unwrap(),
            ),
            public_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicKeys").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            security_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityConfiguration").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
            worker_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workerType").unwrap(),
            ),
            yarn_endpoint_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("yarnEndpointAddress").unwrap(),
            ),
            zeppelin_remote_spark_interpreter_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zeppelinRemoteSparkInterpreterPort").unwrap(),
            ),
        }
    }
}
