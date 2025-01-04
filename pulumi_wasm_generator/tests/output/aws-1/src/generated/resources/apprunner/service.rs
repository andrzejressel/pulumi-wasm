/// Manages an App Runner Service.
///
/// ## Example Usage
///
/// ### Service with a Code Repository Source
///
/// ```yaml
/// resources:
///   example:
///     type: aws:apprunner:Service
///     properties:
///       serviceName: example
///       sourceConfiguration:
///         authenticationConfiguration:
///           connectionArn: ${exampleAwsApprunnerConnection.arn}
///         codeRepository:
///           codeConfiguration:
///             codeConfigurationValues:
///               buildCommand: python setup.py develop
///               port: '8000'
///               runtime: PYTHON_3
///               startCommand: python runapp.py
///             configurationSource: API
///           repositoryUrl: https://github.com/example/my-example-python-app
///           sourceCodeVersion:
///             type: BRANCH
///             value: main
///       networkConfiguration:
///         egressConfiguration:
///           egressType: VPC
///           vpcConnectorArn: ${connector.arn}
///       tags:
///         Name: example-apprunner-service
/// ```
///
/// ### Service with an Image Repository Source
///
/// ```yaml
/// resources:
///   example:
///     type: aws:apprunner:Service
///     properties:
///       serviceName: example
///       sourceConfiguration:
///         imageRepository:
///           imageConfiguration:
///             port: '8000'
///           imageIdentifier: public.ecr.aws/aws-containers/hello-app-runner:latest
///           imageRepositoryType: ECR_PUBLIC
///         autoDeploymentsEnabled: false
///       tags:
///         Name: example-apprunner-service
/// ```
///
/// ### Service with Observability Configuration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:apprunner:Service
///     properties:
///       serviceName: example
///       observabilityConfiguration:
///         observabilityConfigurationArn: ${exampleObservabilityConfiguration.arn}
///         observabilityEnabled: true
///       sourceConfiguration:
///         imageRepository:
///           imageConfiguration:
///             port: '8000'
///           imageIdentifier: public.ecr.aws/aws-containers/hello-app-runner:latest
///           imageRepositoryType: ECR_PUBLIC
///         autoDeploymentsEnabled: false
///       tags:
///         Name: example-apprunner-service
///   exampleObservabilityConfiguration:
///     type: aws:apprunner:ObservabilityConfiguration
///     name: example
///     properties:
///       observabilityConfigurationName: example
///       traceConfiguration:
///         vendor: AWSXRAY
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import App Runner Services using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:apprunner/service:Service example arn:aws:apprunner:us-east-1:1234567890:service/example/0a03292a89764e5882c41d8f991c82fe
/// ```
pub mod service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// ARN of an App Runner automatic scaling configuration resource that you want to associate with your service. If not provided, App Runner associates the latest revision of a default auto scaling configuration.
        #[builder(into, default)]
        pub auto_scaling_configuration_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// An optional custom encryption key that App Runner uses to encrypt the copy of your source repository that it maintains and your service logs. By default, App Runner uses an AWS managed CMK. See Encryption Configuration below for more details.
        #[builder(into, default)]
        pub encryption_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::apprunner::ServiceEncryptionConfiguration>,
        >,
        /// Settings of the health check that AWS App Runner performs to monitor the health of your service. See Health Check Configuration below for more details.
        #[builder(into, default)]
        pub health_check_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::apprunner::ServiceHealthCheckConfiguration>,
        >,
        /// The runtime configuration of instances (scaling units) of the App Runner service. See Instance Configuration below for more details.
        #[builder(into, default)]
        pub instance_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::apprunner::ServiceInstanceConfiguration>,
        >,
        /// Configuration settings related to network traffic of the web application that the App Runner service runs. See Network Configuration below for more details.
        #[builder(into, default)]
        pub network_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::apprunner::ServiceNetworkConfiguration>,
        >,
        /// The observability configuration of your service. See Observability Configuration below for more details.
        #[builder(into, default)]
        pub observability_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::apprunner::ServiceObservabilityConfiguration>,
        >,
        /// Name of the service.
        #[builder(into)]
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// The source to deploy to the App Runner service. Can be a code or an image repository. See Source Configuration below for more details.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub source_configuration: pulumi_wasm_rust::Output<
            super::super::types::apprunner::ServiceSourceConfiguration,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// ARN of the App Runner service.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN of an App Runner automatic scaling configuration resource that you want to associate with your service. If not provided, App Runner associates the latest revision of a default auto scaling configuration.
        pub auto_scaling_configuration_arn: pulumi_wasm_rust::Output<String>,
        /// An optional custom encryption key that App Runner uses to encrypt the copy of your source repository that it maintains and your service logs. By default, App Runner uses an AWS managed CMK. See Encryption Configuration below for more details.
        pub encryption_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::apprunner::ServiceEncryptionConfiguration>,
        >,
        /// Settings of the health check that AWS App Runner performs to monitor the health of your service. See Health Check Configuration below for more details.
        pub health_check_configuration: pulumi_wasm_rust::Output<
            super::super::types::apprunner::ServiceHealthCheckConfiguration,
        >,
        /// The runtime configuration of instances (scaling units) of the App Runner service. See Instance Configuration below for more details.
        pub instance_configuration: pulumi_wasm_rust::Output<
            super::super::types::apprunner::ServiceInstanceConfiguration,
        >,
        /// Configuration settings related to network traffic of the web application that the App Runner service runs. See Network Configuration below for more details.
        pub network_configuration: pulumi_wasm_rust::Output<
            super::super::types::apprunner::ServiceNetworkConfiguration,
        >,
        /// The observability configuration of your service. See Observability Configuration below for more details.
        pub observability_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::apprunner::ServiceObservabilityConfiguration>,
        >,
        /// An alphanumeric ID that App Runner generated for this service. Unique within the AWS Region.
        pub service_id: pulumi_wasm_rust::Output<String>,
        /// Name of the service.
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// Subdomain URL that App Runner generated for this service. You can use this URL to access your service web application.
        pub service_url: pulumi_wasm_rust::Output<String>,
        /// The source to deploy to the App Runner service. Can be a code or an image repository. See Source Configuration below for more details.
        ///
        /// The following arguments are optional:
        pub source_configuration: pulumi_wasm_rust::Output<
            super::super::types::apprunner::ServiceSourceConfiguration,
        >,
        /// Current state of the App Runner service.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceArgs) -> ServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_scaling_configuration_arn_binding = args
            .auto_scaling_configuration_arn
            .get_inner();
        let encryption_configuration_binding = args.encryption_configuration.get_inner();
        let health_check_configuration_binding = args
            .health_check_configuration
            .get_inner();
        let instance_configuration_binding = args.instance_configuration.get_inner();
        let network_configuration_binding = args.network_configuration.get_inner();
        let observability_configuration_binding = args
            .observability_configuration
            .get_inner();
        let service_name_binding = args.service_name.get_inner();
        let source_configuration_binding = args.source_configuration.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apprunner/service:Service".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoScalingConfigurationArn".into(),
                    value: &auto_scaling_configuration_arn_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionConfiguration".into(),
                    value: &encryption_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "healthCheckConfiguration".into(),
                    value: &health_check_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "instanceConfiguration".into(),
                    value: &instance_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfiguration".into(),
                    value: &network_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "observabilityConfiguration".into(),
                    value: &observability_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "serviceName".into(),
                    value: &service_name_binding,
                },
                register_interface::ObjectField {
                    name: "sourceConfiguration".into(),
                    value: &source_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoScalingConfigurationArn".into(),
                },
                register_interface::ResultField {
                    name: "encryptionConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "healthCheckConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "instanceConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "networkConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "observabilityConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "serviceId".into(),
                },
                register_interface::ResultField {
                    name: "serviceName".into(),
                },
                register_interface::ResultField {
                    name: "serviceUrl".into(),
                },
                register_interface::ResultField {
                    name: "sourceConfiguration".into(),
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
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_scaling_configuration_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoScalingConfigurationArn").unwrap(),
            ),
            encryption_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfiguration").unwrap(),
            ),
            health_check_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthCheckConfiguration").unwrap(),
            ),
            instance_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceConfiguration").unwrap(),
            ),
            network_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkConfiguration").unwrap(),
            ),
            observability_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("observabilityConfiguration").unwrap(),
            ),
            service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceId").unwrap(),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceName").unwrap(),
            ),
            service_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceUrl").unwrap(),
            ),
            source_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceConfiguration").unwrap(),
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
        }
    }
}
