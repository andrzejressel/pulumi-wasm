/// Provides a SageMaker model resource.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:sagemaker:Model
///     properties:
///       name: my-model
///       executionRoleArn: ${exampleRole.arn}
///       primaryContainer:
///         image: ${test.registryPath}
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       assumeRolePolicy: ${assumeRole.json}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             principals:
///               - type: Service
///                 identifiers:
///                   - sagemaker.amazonaws.com
///   test:
///     fn::invoke:
///       function: aws:sagemaker:getPrebuiltEcrImage
///       arguments:
///         repositoryName: kmeans
/// ```
///
/// ## Inference Execution Config
///
/// * `mode` - (Required) How containers in a multi-container are run. The following values are valid `Serial` and `Direct`.
///
/// ## Import
///
/// Using `pulumi import`, import models using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/model:Model test_model model-foo
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod model {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ModelArgs {
        /// Specifies containers in the inference pipeline. If not specified, the `primary_container` argument is required. Fields are documented below.
        #[builder(into, default)]
        pub containers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::sagemaker::ModelContainer>>,
        >,
        /// Isolates the model container. No inbound or outbound network calls can be made to or from the model container.
        #[builder(into, default)]
        pub enable_network_isolation: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A role that SageMaker can assume to access model artifacts and docker images for deployment.
        #[builder(into)]
        pub execution_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies details of how containers in a multi-container endpoint are called. see Inference Execution Config.
        #[builder(into, default)]
        pub inference_execution_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::ModelInferenceExecutionConfig>,
        >,
        /// The name of the model (must be unique). If omitted, this provider will assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The primary docker image containing inference code that is used when the model is deployed for predictions.  If not specified, the `container` argument is required. Fields are documented below.
        #[builder(into, default)]
        pub primary_container: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::ModelPrimaryContainer>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the VPC that you want your model to connect to. VpcConfig is used in hosting services and in batch transform.
        #[builder(into, default)]
        pub vpc_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::ModelVpcConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct ModelResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this model.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies containers in the inference pipeline. If not specified, the `primary_container` argument is required. Fields are documented below.
        pub containers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::sagemaker::ModelContainer>>,
        >,
        /// Isolates the model container. No inbound or outbound network calls can be made to or from the model container.
        pub enable_network_isolation: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A role that SageMaker can assume to access model artifacts and docker images for deployment.
        pub execution_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies details of how containers in a multi-container endpoint are called. see Inference Execution Config.
        pub inference_execution_config: pulumi_gestalt_rust::Output<
            super::super::types::sagemaker::ModelInferenceExecutionConfig,
        >,
        /// The name of the model (must be unique). If omitted, this provider will assign a random, unique name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The primary docker image containing inference code that is used when the model is deployed for predictions.  If not specified, the `container` argument is required. Fields are documented below.
        pub primary_container: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::ModelPrimaryContainer>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the VPC that you want your model to connect to. VpcConfig is used in hosting services and in batch transform.
        pub vpc_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::ModelVpcConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ModelArgs,
    ) -> ModelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let containers_binding = args.containers.get_output(context);
        let enable_network_isolation_binding = args
            .enable_network_isolation
            .get_output(context);
        let execution_role_arn_binding = args.execution_role_arn.get_output(context);
        let inference_execution_config_binding = args
            .inference_execution_config
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let primary_container_binding = args.primary_container.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_config_binding = args.vpc_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/model:Model".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containers".into(),
                    value: &containers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableNetworkIsolation".into(),
                    value: &enable_network_isolation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "executionRoleArn".into(),
                    value: &execution_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inferenceExecutionConfig".into(),
                    value: &inference_execution_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "primaryContainer".into(),
                    value: &primary_container_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConfig".into(),
                    value: &vpc_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ModelResult {
            arn: o.get_field("arn"),
            containers: o.get_field("containers"),
            enable_network_isolation: o.get_field("enableNetworkIsolation"),
            execution_role_arn: o.get_field("executionRoleArn"),
            inference_execution_config: o.get_field("inferenceExecutionConfig"),
            name: o.get_field("name"),
            primary_container: o.get_field("primaryContainer"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_config: o.get_field("vpcConfig"),
        }
    }
}
