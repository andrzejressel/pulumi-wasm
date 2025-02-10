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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// ARN of an App Runner automatic scaling configuration resource that you want to associate with your service. If not provided, App Runner associates the latest revision of a default auto scaling configuration.
        #[builder(into, default)]
        pub auto_scaling_configuration_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// An optional custom encryption key that App Runner uses to encrypt the copy of your source repository that it maintains and your service logs. By default, App Runner uses an AWS managed CMK. See Encryption Configuration below for more details.
        #[builder(into, default)]
        pub encryption_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apprunner::ServiceEncryptionConfiguration>,
        >,
        /// Settings of the health check that AWS App Runner performs to monitor the health of your service. See Health Check Configuration below for more details.
        #[builder(into, default)]
        pub health_check_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apprunner::ServiceHealthCheckConfiguration>,
        >,
        /// The runtime configuration of instances (scaling units) of the App Runner service. See Instance Configuration below for more details.
        #[builder(into, default)]
        pub instance_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apprunner::ServiceInstanceConfiguration>,
        >,
        /// Configuration settings related to network traffic of the web application that the App Runner service runs. See Network Configuration below for more details.
        #[builder(into, default)]
        pub network_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apprunner::ServiceNetworkConfiguration>,
        >,
        /// The observability configuration of your service. See Observability Configuration below for more details.
        #[builder(into, default)]
        pub observability_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apprunner::ServiceObservabilityConfiguration>,
        >,
        /// Name of the service.
        #[builder(into)]
        pub service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The source to deploy to the App Runner service. Can be a code or an image repository. See Source Configuration below for more details.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub source_configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::apprunner::ServiceSourceConfiguration,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// ARN of the App Runner service.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of an App Runner automatic scaling configuration resource that you want to associate with your service. If not provided, App Runner associates the latest revision of a default auto scaling configuration.
        pub auto_scaling_configuration_arn: pulumi_gestalt_rust::Output<String>,
        /// An optional custom encryption key that App Runner uses to encrypt the copy of your source repository that it maintains and your service logs. By default, App Runner uses an AWS managed CMK. See Encryption Configuration below for more details.
        pub encryption_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::apprunner::ServiceEncryptionConfiguration>,
        >,
        /// Settings of the health check that AWS App Runner performs to monitor the health of your service. See Health Check Configuration below for more details.
        pub health_check_configuration: pulumi_gestalt_rust::Output<
            super::super::types::apprunner::ServiceHealthCheckConfiguration,
        >,
        /// The runtime configuration of instances (scaling units) of the App Runner service. See Instance Configuration below for more details.
        pub instance_configuration: pulumi_gestalt_rust::Output<
            super::super::types::apprunner::ServiceInstanceConfiguration,
        >,
        /// Configuration settings related to network traffic of the web application that the App Runner service runs. See Network Configuration below for more details.
        pub network_configuration: pulumi_gestalt_rust::Output<
            super::super::types::apprunner::ServiceNetworkConfiguration,
        >,
        /// The observability configuration of your service. See Observability Configuration below for more details.
        pub observability_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::apprunner::ServiceObservabilityConfiguration>,
        >,
        /// An alphanumeric ID that App Runner generated for this service. Unique within the AWS Region.
        pub service_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the service.
        pub service_name: pulumi_gestalt_rust::Output<String>,
        /// Subdomain URL that App Runner generated for this service. You can use this URL to access your service web application.
        pub service_url: pulumi_gestalt_rust::Output<String>,
        /// The source to deploy to the App Runner service. Can be a code or an image repository. See Source Configuration below for more details.
        ///
        /// The following arguments are optional:
        pub source_configuration: pulumi_gestalt_rust::Output<
            super::super::types::apprunner::ServiceSourceConfiguration,
        >,
        /// Current state of the App Runner service.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: ServiceArgs,
    ) -> ServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_scaling_configuration_arn_binding = args
            .auto_scaling_configuration_arn
            .get_output(context);
        let encryption_configuration_binding = args
            .encryption_configuration
            .get_output(context);
        let health_check_configuration_binding = args
            .health_check_configuration
            .get_output(context);
        let instance_configuration_binding = args
            .instance_configuration
            .get_output(context);
        let network_configuration_binding = args
            .network_configuration
            .get_output(context);
        let observability_configuration_binding = args
            .observability_configuration
            .get_output(context);
        let service_name_binding = args.service_name.get_output(context);
        let source_configuration_binding = args.source_configuration.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apprunner/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoScalingConfigurationArn".into(),
                    value: auto_scaling_configuration_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionConfiguration".into(),
                    value: encryption_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthCheckConfiguration".into(),
                    value: health_check_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceConfiguration".into(),
                    value: instance_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkConfiguration".into(),
                    value: network_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "observabilityConfiguration".into(),
                    value: observability_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceName".into(),
                    value: service_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceConfiguration".into(),
                    value: source_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceResult {
            arn: o.get_field("arn"),
            auto_scaling_configuration_arn: o.get_field("autoScalingConfigurationArn"),
            encryption_configuration: o.get_field("encryptionConfiguration"),
            health_check_configuration: o.get_field("healthCheckConfiguration"),
            instance_configuration: o.get_field("instanceConfiguration"),
            network_configuration: o.get_field("networkConfiguration"),
            observability_configuration: o.get_field("observabilityConfiguration"),
            service_id: o.get_field("serviceId"),
            service_name: o.get_field("serviceName"),
            service_url: o.get_field("serviceUrl"),
            source_configuration: o.get_field("sourceConfiguration"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
