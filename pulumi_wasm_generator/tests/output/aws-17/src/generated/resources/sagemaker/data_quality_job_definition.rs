/// Provides a SageMaker data quality job definition resource.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = data_quality_job_definition::create(
///         "test",
///         DataQualityJobDefinitionArgs::builder()
///             .data_quality_app_specification(
///                 DataQualityJobDefinitionDataQualityAppSpecification::builder()
///                     .imageUri("${monitor.registryPath}")
///                     .build_struct(),
///             )
///             .data_quality_job_input(
///                 DataQualityJobDefinitionDataQualityJobInput::builder()
///                     .endpointInput(
///                         DataQualityJobDefinitionDataQualityJobInputEndpointInput::builder()
///                             .endpointName("${myEndpoint.name}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .data_quality_job_output_config(
///                 DataQualityJobDefinitionDataQualityJobOutputConfig::builder()
///                     .monitoringOutputs(
///                         DataQualityJobDefinitionDataQualityJobOutputConfigMonitoringOutputs::builder()
///                             .s3Output(
///                                 DataQualityJobDefinitionDataQualityJobOutputConfigMonitoringOutputsS3Output::builder()
///                                     .s3Uri(
///                                         "https://${myBucket.bucketRegionalDomainName}/output",
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .job_resources(
///                 DataQualityJobDefinitionJobResources::builder()
///                     .clusterConfig(
///                         DataQualityJobDefinitionJobResourcesClusterConfig::builder()
///                             .instanceCount(1)
///                             .instanceType("ml.t3.medium")
///                             .volumeSizeInGb(20)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("my-data-quality-job-definition")
///             .role_arn("${myRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import data quality job definitions using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/dataQualityJobDefinition:DataQualityJobDefinition test_data_quality_job_definition data-quality-job-definition-foo
/// ```
pub mod data_quality_job_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataQualityJobDefinitionArgs {
        /// Specifies the container that runs the monitoring job. Fields are documented below.
        #[builder(into)]
        pub data_quality_app_specification: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::DataQualityJobDefinitionDataQualityAppSpecification,
        >,
        /// Configures the constraints and baselines for the monitoring job. Fields are documented below.
        #[builder(into, default)]
        pub data_quality_baseline_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::sagemaker::DataQualityJobDefinitionDataQualityBaselineConfig,
            >,
        >,
        /// A list of inputs for the monitoring job. Fields are documented below.
        #[builder(into)]
        pub data_quality_job_input: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::DataQualityJobDefinitionDataQualityJobInput,
        >,
        /// The output configuration for monitoring jobs. Fields are documented below.
        #[builder(into)]
        pub data_quality_job_output_config: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::DataQualityJobDefinitionDataQualityJobOutputConfig,
        >,
        /// Identifies the resources to deploy for a monitoring job. Fields are documented below.
        #[builder(into)]
        pub job_resources: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::DataQualityJobDefinitionJobResources,
        >,
        /// The name of the data quality job definition. If omitted, the provider will assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies networking configuration for the monitoring job. Fields are documented below.
        #[builder(into, default)]
        pub network_config: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::DataQualityJobDefinitionNetworkConfig>,
        >,
        /// The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on your behalf.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// A time limit for how long the monitoring job is allowed to run before stopping. Fields are documented below.
        #[builder(into, default)]
        pub stopping_condition: pulumi_wasm_rust::Output<
            Option<
                super::super::types::sagemaker::DataQualityJobDefinitionStoppingCondition,
            >,
        >,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataQualityJobDefinitionResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this data quality job definition.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies the container that runs the monitoring job. Fields are documented below.
        pub data_quality_app_specification: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::DataQualityJobDefinitionDataQualityAppSpecification,
        >,
        /// Configures the constraints and baselines for the monitoring job. Fields are documented below.
        pub data_quality_baseline_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::sagemaker::DataQualityJobDefinitionDataQualityBaselineConfig,
            >,
        >,
        /// A list of inputs for the monitoring job. Fields are documented below.
        pub data_quality_job_input: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::DataQualityJobDefinitionDataQualityJobInput,
        >,
        /// The output configuration for monitoring jobs. Fields are documented below.
        pub data_quality_job_output_config: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::DataQualityJobDefinitionDataQualityJobOutputConfig,
        >,
        /// Identifies the resources to deploy for a monitoring job. Fields are documented below.
        pub job_resources: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::DataQualityJobDefinitionJobResources,
        >,
        /// The name of the data quality job definition. If omitted, the provider will assign a random, unique name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies networking configuration for the monitoring job. Fields are documented below.
        pub network_config: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::DataQualityJobDefinitionNetworkConfig>,
        >,
        /// The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on your behalf.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// A time limit for how long the monitoring job is allowed to run before stopping. Fields are documented below.
        pub stopping_condition: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::DataQualityJobDefinitionStoppingCondition,
        >,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DataQualityJobDefinitionArgs,
    ) -> DataQualityJobDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_quality_app_specification_binding = args
            .data_quality_app_specification
            .get_inner();
        let data_quality_baseline_config_binding = args
            .data_quality_baseline_config
            .get_inner();
        let data_quality_job_input_binding = args.data_quality_job_input.get_inner();
        let data_quality_job_output_config_binding = args
            .data_quality_job_output_config
            .get_inner();
        let job_resources_binding = args.job_resources.get_inner();
        let name_binding = args.name.get_inner();
        let network_config_binding = args.network_config.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let stopping_condition_binding = args.stopping_condition.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/dataQualityJobDefinition:DataQualityJobDefinition"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataQualityAppSpecification".into(),
                    value: &data_quality_app_specification_binding,
                },
                register_interface::ObjectField {
                    name: "dataQualityBaselineConfig".into(),
                    value: &data_quality_baseline_config_binding,
                },
                register_interface::ObjectField {
                    name: "dataQualityJobInput".into(),
                    value: &data_quality_job_input_binding,
                },
                register_interface::ObjectField {
                    name: "dataQualityJobOutputConfig".into(),
                    value: &data_quality_job_output_config_binding,
                },
                register_interface::ObjectField {
                    name: "jobResources".into(),
                    value: &job_resources_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfig".into(),
                    value: &network_config_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "stoppingCondition".into(),
                    value: &stopping_condition_binding,
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
                    name: "dataQualityAppSpecification".into(),
                },
                register_interface::ResultField {
                    name: "dataQualityBaselineConfig".into(),
                },
                register_interface::ResultField {
                    name: "dataQualityJobInput".into(),
                },
                register_interface::ResultField {
                    name: "dataQualityJobOutputConfig".into(),
                },
                register_interface::ResultField {
                    name: "jobResources".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkConfig".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "stoppingCondition".into(),
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
        DataQualityJobDefinitionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            data_quality_app_specification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataQualityAppSpecification").unwrap(),
            ),
            data_quality_baseline_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataQualityBaselineConfig").unwrap(),
            ),
            data_quality_job_input: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataQualityJobInput").unwrap(),
            ),
            data_quality_job_output_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataQualityJobOutputConfig").unwrap(),
            ),
            job_resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jobResources").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkConfig").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            stopping_condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stoppingCondition").unwrap(),
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
