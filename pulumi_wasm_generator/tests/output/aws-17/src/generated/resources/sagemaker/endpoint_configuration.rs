/// Provides a SageMaker endpoint configuration resource.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   ec:
///     type: aws:sagemaker:EndpointConfiguration
///     properties:
///       name: my-endpoint-config
///       productionVariants:
///         - variantName: variant-1
///           modelName: ${m.name}
///           initialInstanceCount: 1
///           instanceType: ml.t2.medium
///       tags:
///         Name: foo
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import endpoint configurations using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/endpointConfiguration:EndpointConfiguration test_endpoint_config endpoint-config-foo
/// ```
pub mod endpoint_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointConfigurationArgs {
        /// Specifies configuration for how an endpoint performs asynchronous inference.
        #[builder(into, default)]
        pub async_inference_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::sagemaker::EndpointConfigurationAsyncInferenceConfig,
            >,
        >,
        /// Specifies the parameters to capture input/output of SageMaker models endpoints. Fields are documented below.
        #[builder(into, default)]
        pub data_capture_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::sagemaker::EndpointConfigurationDataCaptureConfig,
            >,
        >,
        /// Amazon Resource Name (ARN) of a AWS Key Management Service key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the endpoint configuration. If omitted, this provider will assign a random, unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique endpoint configuration name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// An list of ProductionVariant objects, one for each model that you want to host at this endpoint. Fields are documented below.
        #[builder(into)]
        pub production_variants: pulumi_wasm_rust::Output<
            Vec<super::super::types::sagemaker::EndpointConfigurationProductionVariant>,
        >,
        /// Array of ProductionVariant objects. There is one for each model that you want to host at this endpoint in shadow mode with production traffic replicated from the model specified on ProductionVariants. If you use this field, you can only specify one variant for ProductionVariants and one variant for ShadowProductionVariants. Fields are documented below.
        #[builder(into, default)]
        pub shadow_production_variants: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::sagemaker::EndpointConfigurationShadowProductionVariant,
                >,
            >,
        >,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EndpointConfigurationResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this endpoint configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies configuration for how an endpoint performs asynchronous inference.
        pub async_inference_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::sagemaker::EndpointConfigurationAsyncInferenceConfig,
            >,
        >,
        /// Specifies the parameters to capture input/output of SageMaker models endpoints. Fields are documented below.
        pub data_capture_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::sagemaker::EndpointConfigurationDataCaptureConfig,
            >,
        >,
        /// Amazon Resource Name (ARN) of a AWS Key Management Service key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint.
        pub kms_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the endpoint configuration. If omitted, this provider will assign a random, unique name. Conflicts with `name_prefix`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique endpoint configuration name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// An list of ProductionVariant objects, one for each model that you want to host at this endpoint. Fields are documented below.
        pub production_variants: pulumi_wasm_rust::Output<
            Vec<super::super::types::sagemaker::EndpointConfigurationProductionVariant>,
        >,
        /// Array of ProductionVariant objects. There is one for each model that you want to host at this endpoint in shadow mode with production traffic replicated from the model specified on ProductionVariants. If you use this field, you can only specify one variant for ProductionVariants and one variant for ShadowProductionVariants. Fields are documented below.
        pub shadow_production_variants: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::sagemaker::EndpointConfigurationShadowProductionVariant,
                >,
            >,
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
        args: EndpointConfigurationArgs,
    ) -> EndpointConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let async_inference_config_binding = args.async_inference_config.get_inner();
        let data_capture_config_binding = args.data_capture_config.get_inner();
        let kms_key_arn_binding = args.kms_key_arn.get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let production_variants_binding = args.production_variants.get_inner();
        let shadow_production_variants_binding = args
            .shadow_production_variants
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/endpointConfiguration:EndpointConfiguration".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "asyncInferenceConfig".into(),
                    value: &async_inference_config_binding,
                },
                register_interface::ObjectField {
                    name: "dataCaptureConfig".into(),
                    value: &data_capture_config_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyArn".into(),
                    value: &kms_key_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "productionVariants".into(),
                    value: &production_variants_binding,
                },
                register_interface::ObjectField {
                    name: "shadowProductionVariants".into(),
                    value: &shadow_production_variants_binding,
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
                    name: "asyncInferenceConfig".into(),
                },
                register_interface::ResultField {
                    name: "dataCaptureConfig".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "productionVariants".into(),
                },
                register_interface::ResultField {
                    name: "shadowProductionVariants".into(),
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
        EndpointConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            async_inference_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("asyncInferenceConfig").unwrap(),
            ),
            data_capture_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataCaptureConfig").unwrap(),
            ),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyArn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            production_variants: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productionVariants").unwrap(),
            ),
            shadow_production_variants: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shadowProductionVariants").unwrap(),
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
