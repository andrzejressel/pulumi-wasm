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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointConfigurationArgs {
        /// Specifies configuration for how an endpoint performs asynchronous inference.
        #[builder(into, default)]
        pub async_inference_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::sagemaker::EndpointConfigurationAsyncInferenceConfig,
            >,
        >,
        /// Specifies the parameters to capture input/output of SageMaker models endpoints. Fields are documented below.
        #[builder(into, default)]
        pub data_capture_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::sagemaker::EndpointConfigurationDataCaptureConfig,
            >,
        >,
        /// Amazon Resource Name (ARN) of a AWS Key Management Service key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the endpoint configuration. If omitted, this provider will assign a random, unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique endpoint configuration name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An list of ProductionVariant objects, one for each model that you want to host at this endpoint. Fields are documented below.
        #[builder(into)]
        pub production_variants: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::sagemaker::EndpointConfigurationProductionVariant>,
        >,
        /// Array of ProductionVariant objects. There is one for each model that you want to host at this endpoint in shadow mode with production traffic replicated from the model specified on ProductionVariants. If you use this field, you can only specify one variant for ProductionVariants and one variant for ShadowProductionVariants. Fields are documented below.
        #[builder(into, default)]
        pub shadow_production_variants: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::sagemaker::EndpointConfigurationShadowProductionVariant,
                >,
            >,
        >,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EndpointConfigurationResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this endpoint configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies configuration for how an endpoint performs asynchronous inference.
        pub async_inference_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::sagemaker::EndpointConfigurationAsyncInferenceConfig,
            >,
        >,
        /// Specifies the parameters to capture input/output of SageMaker models endpoints. Fields are documented below.
        pub data_capture_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::sagemaker::EndpointConfigurationDataCaptureConfig,
            >,
        >,
        /// Amazon Resource Name (ARN) of a AWS Key Management Service key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint.
        pub kms_key_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the endpoint configuration. If omitted, this provider will assign a random, unique name. Conflicts with `name_prefix`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique endpoint configuration name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// An list of ProductionVariant objects, one for each model that you want to host at this endpoint. Fields are documented below.
        pub production_variants: pulumi_gestalt_rust::Output<
            Vec<super::super::types::sagemaker::EndpointConfigurationProductionVariant>,
        >,
        /// Array of ProductionVariant objects. There is one for each model that you want to host at this endpoint in shadow mode with production traffic replicated from the model specified on ProductionVariants. If you use this field, you can only specify one variant for ProductionVariants and one variant for ShadowProductionVariants. Fields are documented below.
        pub shadow_production_variants: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::sagemaker::EndpointConfigurationShadowProductionVariant,
                >,
            >,
        >,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: EndpointConfigurationArgs,
    ) -> EndpointConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let async_inference_config_binding = args
            .async_inference_config
            .get_output(context);
        let data_capture_config_binding = args.data_capture_config.get_output(context);
        let kms_key_arn_binding = args.kms_key_arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let production_variants_binding = args.production_variants.get_output(context);
        let shadow_production_variants_binding = args
            .shadow_production_variants
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/endpointConfiguration:EndpointConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "asyncInferenceConfig".into(),
                    value: async_inference_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataCaptureConfig".into(),
                    value: data_capture_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyArn".into(),
                    value: kms_key_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "productionVariants".into(),
                    value: production_variants_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shadowProductionVariants".into(),
                    value: shadow_production_variants_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EndpointConfigurationResult {
            arn: o.get_field("arn"),
            async_inference_config: o.get_field("asyncInferenceConfig"),
            data_capture_config: o.get_field("dataCaptureConfig"),
            kms_key_arn: o.get_field("kmsKeyArn"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            production_variants: o.get_field("productionVariants"),
            shadow_production_variants: o.get_field("shadowProductionVariants"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
