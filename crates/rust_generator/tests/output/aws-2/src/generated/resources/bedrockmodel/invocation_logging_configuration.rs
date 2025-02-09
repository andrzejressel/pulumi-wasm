/// Manages Bedrock model invocation logging configuration.
///
/// > Model invocation logging is configured per AWS region. To avoid overwriting settings, this resource should not be defined in multiple configurations.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example
///       forceDestroy: true
///   exampleBucketPolicy:
///     type: aws:s3:BucketPolicy
///     name: example
///     properties:
///       bucket: ${example.bucket}
///       policy: |
///         {
///           "Version": "2012-10-17",
///           "Statement": [
///             {
///               "Effect": "Allow",
///               "Principal": {
///                 "Service": "bedrock.amazonaws.com"
///               },
///               "Action": [
///                 "s3:*"
///               ],
///               "Resource": [
///                 "${example.arn}/*"
///               ],
///               "Condition": {
///                 "StringEquals": {
///                   "aws:SourceAccount": "${current.accountId}"
///                 },
///                 "ArnLike": {
///                   "aws:SourceArn": "arn:aws:bedrock:us-east-1:${current.accountId}:*"
///                 }
///               }
///             }
///           ]
///         }
///   exampleInvocationLoggingConfiguration:
///     type: aws:bedrockmodel:InvocationLoggingConfiguration
///     name: example
///     properties:
///       loggingConfig:
///         - embeddingDataDeliveryEnabled: true
///           imageDataDeliveryEnabled: true
///           textDataDeliveryEnabled: true
///           s3Config:
///             - bucketName: ${example.id}
///               keyPrefix: bedrock
///     options:
///       dependsOn:
///         - ${exampleBucketPolicy}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Bedrock custom model using the `id` set to the AWS Region. For example:
///
/// ```sh
/// $ pulumi import aws:bedrockmodel/invocationLoggingConfiguration:InvocationLoggingConfiguration my_config us-east-1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod invocation_logging_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InvocationLoggingConfigurationArgs {
        /// The logging configuration values to set.
        #[builder(into, default)]
        pub logging_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::bedrockmodel::InvocationLoggingConfigurationLoggingConfig,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct InvocationLoggingConfigurationResult {
        /// The logging configuration values to set.
        pub logging_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::bedrockmodel::InvocationLoggingConfigurationLoggingConfig,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InvocationLoggingConfigurationArgs,
    ) -> InvocationLoggingConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let logging_config_binding = args.logging_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:bedrockmodel/invocationLoggingConfiguration:InvocationLoggingConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loggingConfig".into(),
                    value: logging_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InvocationLoggingConfigurationResult {
            logging_config: o.get_field("loggingConfig"),
        }
    }
}
