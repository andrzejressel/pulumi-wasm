/// Creates a MWAA Environment resource.
///
/// ## Example Usage
///
/// A MWAA Environment requires an IAM role (`aws.iam.Role`), two subnets in the private zone (`aws.ec2.Subnet`) and a versioned S3 bucket (`aws.s3.BucketV2`).
///
/// ### Basic Usage
///
///
/// ### Example with Airflow configuration options
///
///
/// ### Example with logging configurations
///
/// Note that Airflow task logs are enabled by default with the `INFO` log level.
///
///
/// ### Example with tags
///
///
/// ## Import
///
/// Using `pulumi import`, import MWAA Environment using `Name`. For example:
///
/// ```sh
/// $ pulumi import aws:mwaa/environment:Environment example MyAirflowEnvironment
/// ```
pub mod environment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentArgs {
        /// The `airflow_configuration_options` parameter specifies airflow override options. Check the [Official documentation](https://docs.aws.amazon.com/mwaa/latest/userguide/configuring-env-variables.html#configuring-env-variables-reference) for all possible configuration options.
        #[builder(into, default)]
        pub airflow_configuration_options: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Airflow version of your environment, will be set by default to the latest version that MWAA supports.
        #[builder(into, default)]
        pub airflow_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The relative path to the DAG folder on your Amazon S3 storage bucket. For example, dags. For more information, see [Importing DAGs on Amazon MWAA](https://docs.aws.amazon.com/mwaa/latest/userguide/configuring-dag-import.html).
        #[builder(into)]
        pub dag_s3_path: pulumi_wasm_rust::Output<String>,
        /// Defines whether the VPC endpoints configured for the environment are created and managed by the customer or by AWS. If set to `SERVICE`, Amazon MWAA will create and manage the required VPC endpoints in your VPC. If set to `CUSTOMER`, you must create, and manage, the VPC endpoints for your VPC. Defaults to `SERVICE` if not set.
        #[builder(into, default)]
        pub endpoint_management: pulumi_wasm_rust::Output<Option<String>>,
        /// Environment class for the cluster. Possible options are `mw1.small`, `mw1.medium`, `mw1.large`. Will be set by default to `mw1.small`. Please check the [AWS Pricing](https://aws.amazon.com/de/managed-workflows-for-apache-airflow/pricing/) for more information about the environment classes.
        #[builder(into, default)]
        pub environment_class: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the task execution role that the Amazon MWAA and its environment can assume. Check the [official AWS documentation](https://docs.aws.amazon.com/mwaa/latest/userguide/mwaa-create-role.html) for the detailed role specification.
        #[builder(into)]
        pub execution_role_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of your KMS key that you want to use for encryption. Will be set to the ARN of the managed KMS key `aws/airflow` by default. Please check the [Official Documentation](https://docs.aws.amazon.com/mwaa/latest/userguide/custom-keys-certs.html) for more information.
        #[builder(into, default)]
        pub kms_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The Apache Airflow logs you want to send to Amazon CloudWatch Logs. See `logging_configuration` Block for details.
        #[builder(into, default)]
        pub logging_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::mwaa::EnvironmentLoggingConfiguration>,
        >,
        /// The maximum number of web servers that you want to run in your environment. Value need to be between `2` and `5`. Will be `2` by default.
        #[builder(into, default)]
        pub max_webservers: pulumi_wasm_rust::Output<Option<i32>>,
        /// The maximum number of workers that can be automatically scaled up. Value need to be between `1` and `25`. Will be `10` by default.
        #[builder(into, default)]
        pub max_workers: pulumi_wasm_rust::Output<Option<i32>>,
        /// The minimum number of web servers that you want to run in your environment. Value need to be between `2` and `5`. Will be `2` by default.
        #[builder(into, default)]
        pub min_webservers: pulumi_wasm_rust::Output<Option<i32>>,
        /// The minimum number of workers that you want to run in your environment. Will be `1` by default.
        #[builder(into, default)]
        pub min_workers: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the Apache Airflow Environment
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the network configuration for your Apache Airflow Environment. This includes two private subnets as well as security groups for the Airflow environment. Each subnet requires internet connection, otherwise the deployment will fail. See `network_configuration` Block for details.
        #[builder(into)]
        pub network_configuration: pulumi_wasm_rust::Output<
            super::super::types::mwaa::EnvironmentNetworkConfiguration,
        >,
        /// The plugins.zip file version you want to use.
        #[builder(into, default)]
        pub plugins_s3_object_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The relative path to the plugins.zip file on your Amazon S3 storage bucket. For example, plugins.zip. If a relative path is provided in the request, then plugins_s3_object_version is required. For more information, see [Importing DAGs on Amazon MWAA](https://docs.aws.amazon.com/mwaa/latest/userguide/configuring-dag-import.html).
        #[builder(into, default)]
        pub plugins_s3_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The requirements.txt file version you want to use.
        #[builder(into, default)]
        pub requirements_s3_object_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The relative path to the requirements.txt file on your Amazon S3 storage bucket. For example, requirements.txt. If a relative path is provided in the request, then requirements_s3_object_version is required. For more information, see [Importing DAGs on Amazon MWAA](https://docs.aws.amazon.com/mwaa/latest/userguide/configuring-dag-import.html).
        #[builder(into, default)]
        pub requirements_s3_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of schedulers that you want to run in your environment. v2.0.2 and above accepts `2` - `5`, default `2`. v1.10.12 accepts `1`.
        #[builder(into, default)]
        pub schedulers: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Amazon Resource Name (ARN) of your Amazon S3 storage bucket. For example, arn:aws:s3:::airflow-mybucketname.
        #[builder(into)]
        pub source_bucket_arn: pulumi_wasm_rust::Output<String>,
        /// The version of the startup shell script you want to use. You must specify the version ID that Amazon S3 assigns to the file every time you update the script.
        #[builder(into, default)]
        pub startup_script_s3_object_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The relative path to the script hosted in your bucket. The script runs as your environment starts before starting the Apache Airflow process. Use this script to install dependencies, modify configuration options, and set environment variables. See [Using a startup script](https://docs.aws.amazon.com/mwaa/latest/userguide/using-startup-script.html). Supported for environment versions 2.x and later.
        #[builder(into, default)]
        pub startup_script_s3_path: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies whether the webserver should be accessible over the internet or via your specified VPC. Possible options: `PRIVATE_ONLY` (default) and `PUBLIC_ONLY`.
        #[builder(into, default)]
        pub webserver_access_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the start date for the weekly maintenance window.
        #[builder(into, default)]
        pub weekly_maintenance_window_start: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentResult {
        /// The `airflow_configuration_options` parameter specifies airflow override options. Check the [Official documentation](https://docs.aws.amazon.com/mwaa/latest/userguide/configuring-env-variables.html#configuring-env-variables-reference) for all possible configuration options.
        pub airflow_configuration_options: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Airflow version of your environment, will be set by default to the latest version that MWAA supports.
        pub airflow_version: pulumi_wasm_rust::Output<String>,
        /// The ARN of the MWAA Environment
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Created At date of the MWAA Environment
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The relative path to the DAG folder on your Amazon S3 storage bucket. For example, dags. For more information, see [Importing DAGs on Amazon MWAA](https://docs.aws.amazon.com/mwaa/latest/userguide/configuring-dag-import.html).
        pub dag_s3_path: pulumi_wasm_rust::Output<String>,
        /// The VPC endpoint for the environment's Amazon RDS database
        /// * `logging_configuration[0].<LOG_CONFIGURATION_TYPE>[0].cloud_watch_log_group_arn` - Provides the ARN for the CloudWatch group where the logs will be published
        pub database_vpc_endpoint_service: pulumi_wasm_rust::Output<String>,
        /// Defines whether the VPC endpoints configured for the environment are created and managed by the customer or by AWS. If set to `SERVICE`, Amazon MWAA will create and manage the required VPC endpoints in your VPC. If set to `CUSTOMER`, you must create, and manage, the VPC endpoints for your VPC. Defaults to `SERVICE` if not set.
        pub endpoint_management: pulumi_wasm_rust::Output<String>,
        /// Environment class for the cluster. Possible options are `mw1.small`, `mw1.medium`, `mw1.large`. Will be set by default to `mw1.small`. Please check the [AWS Pricing](https://aws.amazon.com/de/managed-workflows-for-apache-airflow/pricing/) for more information about the environment classes.
        pub environment_class: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the task execution role that the Amazon MWAA and its environment can assume. Check the [official AWS documentation](https://docs.aws.amazon.com/mwaa/latest/userguide/mwaa-create-role.html) for the detailed role specification.
        pub execution_role_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of your KMS key that you want to use for encryption. Will be set to the ARN of the managed KMS key `aws/airflow` by default. Please check the [Official Documentation](https://docs.aws.amazon.com/mwaa/latest/userguide/custom-keys-certs.html) for more information.
        pub kms_key: pulumi_wasm_rust::Output<Option<String>>,
        pub last_updateds: pulumi_wasm_rust::Output<
            Vec<super::super::types::mwaa::EnvironmentLastUpdated>,
        >,
        /// The Apache Airflow logs you want to send to Amazon CloudWatch Logs. See `logging_configuration` Block for details.
        pub logging_configuration: pulumi_wasm_rust::Output<
            super::super::types::mwaa::EnvironmentLoggingConfiguration,
        >,
        /// The maximum number of web servers that you want to run in your environment. Value need to be between `2` and `5`. Will be `2` by default.
        pub max_webservers: pulumi_wasm_rust::Output<i32>,
        /// The maximum number of workers that can be automatically scaled up. Value need to be between `1` and `25`. Will be `10` by default.
        pub max_workers: pulumi_wasm_rust::Output<i32>,
        /// The minimum number of web servers that you want to run in your environment. Value need to be between `2` and `5`. Will be `2` by default.
        pub min_webservers: pulumi_wasm_rust::Output<i32>,
        /// The minimum number of workers that you want to run in your environment. Will be `1` by default.
        pub min_workers: pulumi_wasm_rust::Output<i32>,
        /// The name of the Apache Airflow Environment
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the network configuration for your Apache Airflow Environment. This includes two private subnets as well as security groups for the Airflow environment. Each subnet requires internet connection, otherwise the deployment will fail. See `network_configuration` Block for details.
        pub network_configuration: pulumi_wasm_rust::Output<
            super::super::types::mwaa::EnvironmentNetworkConfiguration,
        >,
        /// The plugins.zip file version you want to use.
        pub plugins_s3_object_version: pulumi_wasm_rust::Output<String>,
        /// The relative path to the plugins.zip file on your Amazon S3 storage bucket. For example, plugins.zip. If a relative path is provided in the request, then plugins_s3_object_version is required. For more information, see [Importing DAGs on Amazon MWAA](https://docs.aws.amazon.com/mwaa/latest/userguide/configuring-dag-import.html).
        pub plugins_s3_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The requirements.txt file version you want to use.
        pub requirements_s3_object_version: pulumi_wasm_rust::Output<String>,
        /// The relative path to the requirements.txt file on your Amazon S3 storage bucket. For example, requirements.txt. If a relative path is provided in the request, then requirements_s3_object_version is required. For more information, see [Importing DAGs on Amazon MWAA](https://docs.aws.amazon.com/mwaa/latest/userguide/configuring-dag-import.html).
        pub requirements_s3_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of schedulers that you want to run in your environment. v2.0.2 and above accepts `2` - `5`, default `2`. v1.10.12 accepts `1`.
        pub schedulers: pulumi_wasm_rust::Output<i32>,
        /// The Service Role ARN of the Amazon MWAA Environment
        pub service_role_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of your Amazon S3 storage bucket. For example, arn:aws:s3:::airflow-mybucketname.
        pub source_bucket_arn: pulumi_wasm_rust::Output<String>,
        /// The version of the startup shell script you want to use. You must specify the version ID that Amazon S3 assigns to the file every time you update the script.
        pub startup_script_s3_object_version: pulumi_wasm_rust::Output<String>,
        /// The relative path to the script hosted in your bucket. The script runs as your environment starts before starting the Apache Airflow process. Use this script to install dependencies, modify configuration options, and set environment variables. See [Using a startup script](https://docs.aws.amazon.com/mwaa/latest/userguide/using-startup-script.html). Supported for environment versions 2.x and later.
        pub startup_script_s3_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The status of the Amazon MWAA Environment
        pub status: pulumi_wasm_rust::Output<String>,
        /// A map of resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies whether the webserver should be accessible over the internet or via your specified VPC. Possible options: `PRIVATE_ONLY` (default) and `PUBLIC_ONLY`.
        pub webserver_access_mode: pulumi_wasm_rust::Output<String>,
        /// The webserver URL of the MWAA Environment
        pub webserver_url: pulumi_wasm_rust::Output<String>,
        /// The VPC endpoint for the environment's web server
        pub webserver_vpc_endpoint_service: pulumi_wasm_rust::Output<String>,
        /// Specifies the start date for the weekly maintenance window.
        pub weekly_maintenance_window_start: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EnvironmentArgs) -> EnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let airflow_configuration_options_binding = args
            .airflow_configuration_options
            .get_inner();
        let airflow_version_binding = args.airflow_version.get_inner();
        let dag_s3_path_binding = args.dag_s3_path.get_inner();
        let endpoint_management_binding = args.endpoint_management.get_inner();
        let environment_class_binding = args.environment_class.get_inner();
        let execution_role_arn_binding = args.execution_role_arn.get_inner();
        let kms_key_binding = args.kms_key.get_inner();
        let logging_configuration_binding = args.logging_configuration.get_inner();
        let max_webservers_binding = args.max_webservers.get_inner();
        let max_workers_binding = args.max_workers.get_inner();
        let min_webservers_binding = args.min_webservers.get_inner();
        let min_workers_binding = args.min_workers.get_inner();
        let name_binding = args.name.get_inner();
        let network_configuration_binding = args.network_configuration.get_inner();
        let plugins_s3_object_version_binding = args
            .plugins_s3_object_version
            .get_inner();
        let plugins_s3_path_binding = args.plugins_s3_path.get_inner();
        let requirements_s3_object_version_binding = args
            .requirements_s3_object_version
            .get_inner();
        let requirements_s3_path_binding = args.requirements_s3_path.get_inner();
        let schedulers_binding = args.schedulers.get_inner();
        let source_bucket_arn_binding = args.source_bucket_arn.get_inner();
        let startup_script_s3_object_version_binding = args
            .startup_script_s3_object_version
            .get_inner();
        let startup_script_s3_path_binding = args.startup_script_s3_path.get_inner();
        let tags_binding = args.tags.get_inner();
        let webserver_access_mode_binding = args.webserver_access_mode.get_inner();
        let weekly_maintenance_window_start_binding = args
            .weekly_maintenance_window_start
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:mwaa/environment:Environment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "airflowConfigurationOptions".into(),
                    value: &airflow_configuration_options_binding,
                },
                register_interface::ObjectField {
                    name: "airflowVersion".into(),
                    value: &airflow_version_binding,
                },
                register_interface::ObjectField {
                    name: "dagS3Path".into(),
                    value: &dag_s3_path_binding,
                },
                register_interface::ObjectField {
                    name: "endpointManagement".into(),
                    value: &endpoint_management_binding,
                },
                register_interface::ObjectField {
                    name: "environmentClass".into(),
                    value: &environment_class_binding,
                },
                register_interface::ObjectField {
                    name: "executionRoleArn".into(),
                    value: &execution_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKey".into(),
                    value: &kms_key_binding,
                },
                register_interface::ObjectField {
                    name: "loggingConfiguration".into(),
                    value: &logging_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "maxWebservers".into(),
                    value: &max_webservers_binding,
                },
                register_interface::ObjectField {
                    name: "maxWorkers".into(),
                    value: &max_workers_binding,
                },
                register_interface::ObjectField {
                    name: "minWebservers".into(),
                    value: &min_webservers_binding,
                },
                register_interface::ObjectField {
                    name: "minWorkers".into(),
                    value: &min_workers_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfiguration".into(),
                    value: &network_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "pluginsS3ObjectVersion".into(),
                    value: &plugins_s3_object_version_binding,
                },
                register_interface::ObjectField {
                    name: "pluginsS3Path".into(),
                    value: &plugins_s3_path_binding,
                },
                register_interface::ObjectField {
                    name: "requirementsS3ObjectVersion".into(),
                    value: &requirements_s3_object_version_binding,
                },
                register_interface::ObjectField {
                    name: "requirementsS3Path".into(),
                    value: &requirements_s3_path_binding,
                },
                register_interface::ObjectField {
                    name: "schedulers".into(),
                    value: &schedulers_binding,
                },
                register_interface::ObjectField {
                    name: "sourceBucketArn".into(),
                    value: &source_bucket_arn_binding,
                },
                register_interface::ObjectField {
                    name: "startupScriptS3ObjectVersion".into(),
                    value: &startup_script_s3_object_version_binding,
                },
                register_interface::ObjectField {
                    name: "startupScriptS3Path".into(),
                    value: &startup_script_s3_path_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "webserverAccessMode".into(),
                    value: &webserver_access_mode_binding,
                },
                register_interface::ObjectField {
                    name: "weeklyMaintenanceWindowStart".into(),
                    value: &weekly_maintenance_window_start_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "airflowConfigurationOptions".into(),
                },
                register_interface::ResultField {
                    name: "airflowVersion".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "dagS3Path".into(),
                },
                register_interface::ResultField {
                    name: "databaseVpcEndpointService".into(),
                },
                register_interface::ResultField {
                    name: "endpointManagement".into(),
                },
                register_interface::ResultField {
                    name: "environmentClass".into(),
                },
                register_interface::ResultField {
                    name: "executionRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "kmsKey".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdateds".into(),
                },
                register_interface::ResultField {
                    name: "loggingConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "maxWebservers".into(),
                },
                register_interface::ResultField {
                    name: "maxWorkers".into(),
                },
                register_interface::ResultField {
                    name: "minWebservers".into(),
                },
                register_interface::ResultField {
                    name: "minWorkers".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "pluginsS3ObjectVersion".into(),
                },
                register_interface::ResultField {
                    name: "pluginsS3Path".into(),
                },
                register_interface::ResultField {
                    name: "requirementsS3ObjectVersion".into(),
                },
                register_interface::ResultField {
                    name: "requirementsS3Path".into(),
                },
                register_interface::ResultField {
                    name: "schedulers".into(),
                },
                register_interface::ResultField {
                    name: "serviceRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "sourceBucketArn".into(),
                },
                register_interface::ResultField {
                    name: "startupScriptS3ObjectVersion".into(),
                },
                register_interface::ResultField {
                    name: "startupScriptS3Path".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "webserverAccessMode".into(),
                },
                register_interface::ResultField {
                    name: "webserverUrl".into(),
                },
                register_interface::ResultField {
                    name: "webserverVpcEndpointService".into(),
                },
                register_interface::ResultField {
                    name: "weeklyMaintenanceWindowStart".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EnvironmentResult {
            airflow_configuration_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("airflowConfigurationOptions").unwrap(),
            ),
            airflow_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("airflowVersion").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            dag_s3_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dagS3Path").unwrap(),
            ),
            database_vpc_endpoint_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseVpcEndpointService").unwrap(),
            ),
            endpoint_management: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointManagement").unwrap(),
            ),
            environment_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentClass").unwrap(),
            ),
            execution_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionRoleArn").unwrap(),
            ),
            kms_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKey").unwrap(),
            ),
            last_updateds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdateds").unwrap(),
            ),
            logging_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingConfiguration").unwrap(),
            ),
            max_webservers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxWebservers").unwrap(),
            ),
            max_workers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxWorkers").unwrap(),
            ),
            min_webservers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minWebservers").unwrap(),
            ),
            min_workers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minWorkers").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkConfiguration").unwrap(),
            ),
            plugins_s3_object_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pluginsS3ObjectVersion").unwrap(),
            ),
            plugins_s3_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pluginsS3Path").unwrap(),
            ),
            requirements_s3_object_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requirementsS3ObjectVersion").unwrap(),
            ),
            requirements_s3_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requirementsS3Path").unwrap(),
            ),
            schedulers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedulers").unwrap(),
            ),
            service_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceRoleArn").unwrap(),
            ),
            source_bucket_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceBucketArn").unwrap(),
            ),
            startup_script_s3_object_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startupScriptS3ObjectVersion").unwrap(),
            ),
            startup_script_s3_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startupScriptS3Path").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            webserver_access_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webserverAccessMode").unwrap(),
            ),
            webserver_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webserverUrl").unwrap(),
            ),
            webserver_vpc_endpoint_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webserverVpcEndpointService").unwrap(),
            ),
            weekly_maintenance_window_start: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("weeklyMaintenanceWindowStart").unwrap(),
            ),
        }
    }
}
