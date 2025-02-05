#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointConfigurationProductionVariant {
    /// The size of the Elastic Inference (EI) instance to use for the production variant.
    #[builder(into, default)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: Box<Option<String>>,
    /// The timeout value, in seconds, for your inference container to pass health check by SageMaker Hosting. For more information about health check, see [How Your Container Should Respond to Health Check (Ping) Requests](https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms-inference-code.html#your-algorithms-inference-algo-ping-requests). Valid values between `60` and `3600`.
    #[builder(into, default)]
    #[serde(rename = "containerStartupHealthCheckTimeoutInSeconds")]
    pub r#container_startup_health_check_timeout_in_seconds: Box<Option<i32>>,
    /// Specifies configuration for a core dump from the model container when the process crashes. Fields are documented below.
    #[builder(into, default)]
    #[serde(rename = "coreDumpConfig")]
    pub r#core_dump_config: Box<Option<super::super::types::sagemaker::EndpointConfigurationProductionVariantCoreDumpConfig>>,
    /// You can use this parameter to turn on native Amazon Web Services Systems Manager (SSM) access for a production variant behind an endpoint. By default, SSM access is disabled for all production variants behind an endpoints.
    #[builder(into, default)]
    #[serde(rename = "enableSsmAccess")]
    pub r#enable_ssm_access: Box<Option<bool>>,
    /// Specifies an option from a collection of preconfigured Amazon Machine Image (AMI) images. Each image is configured by Amazon Web Services with a set of software and driver versions. Amazon Web Services optimizes these configurations for different machine learning workloads.
    #[builder(into, default)]
    #[serde(rename = "inferenceAmiVersion")]
    pub r#inference_ami_version: Box<Option<String>>,
    /// Initial number of instances used for auto-scaling.
    #[builder(into, default)]
    #[serde(rename = "initialInstanceCount")]
    pub r#initial_instance_count: Box<Option<i32>>,
    /// Determines initial traffic distribution among all of the models that you specify in the endpoint configuration. If unspecified, it defaults to `1.0`.
    #[builder(into, default)]
    #[serde(rename = "initialVariantWeight")]
    pub r#initial_variant_weight: Box<Option<f64>>,
    /// The type of instance to start.
    #[builder(into, default)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<Option<String>>,
    /// Settings that control the range in the number of instances that the endpoint provisions as it scales up or down to accommodate traffic.
    #[builder(into, default)]
    #[serde(rename = "managedInstanceScaling")]
    pub r#managed_instance_scaling: Box<Option<super::super::types::sagemaker::EndpointConfigurationProductionVariantManagedInstanceScaling>>,
    /// The timeout value, in seconds, to download and extract the model that you want to host from Amazon S3 to the individual inference instance associated with this production variant. Valid values between `60` and `3600`.
    #[builder(into, default)]
    #[serde(rename = "modelDataDownloadTimeoutInSeconds")]
    pub r#model_data_download_timeout_in_seconds: Box<Option<i32>>,
    /// The name of the model to use.
    #[builder(into)]
    #[serde(rename = "modelName")]
    pub r#model_name: Box<String>,
    /// Sets how the endpoint routes incoming traffic. See routing_config below.
    #[builder(into, default)]
    #[serde(rename = "routingConfigs")]
    pub r#routing_configs: Box<Option<Vec<super::super::types::sagemaker::EndpointConfigurationProductionVariantRoutingConfig>>>,
    /// Specifies configuration for how an endpoint performs asynchronous inference.
    #[builder(into, default)]
    #[serde(rename = "serverlessConfig")]
    pub r#serverless_config: Box<Option<super::super::types::sagemaker::EndpointConfigurationProductionVariantServerlessConfig>>,
    /// The name of the variant. If omitted, this provider will assign a random, unique name.
    #[builder(into, default)]
    #[serde(rename = "variantName")]
    pub r#variant_name: Box<Option<String>>,
    /// The size, in GB, of the ML storage volume attached to individual inference instance associated with the production variant. Valid values between `1` and `512`.
    #[builder(into, default)]
    #[serde(rename = "volumeSizeInGb")]
    pub r#volume_size_in_gb: Box<Option<i32>>,
}
