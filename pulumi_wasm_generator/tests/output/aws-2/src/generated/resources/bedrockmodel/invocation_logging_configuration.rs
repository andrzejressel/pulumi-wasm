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
pub mod invocation_logging_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InvocationLoggingConfigurationArgs {
        /// The logging configuration values to set.
        #[builder(into, default)]
        pub logging_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::bedrockmodel::InvocationLoggingConfigurationLoggingConfig,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct InvocationLoggingConfigurationResult {
        /// The logging configuration values to set.
        pub logging_config: pulumi_wasm_rust::Output<
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
        name: &str,
        args: InvocationLoggingConfigurationArgs,
    ) -> InvocationLoggingConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let logging_config_binding = args.logging_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:bedrockmodel/invocationLoggingConfiguration:InvocationLoggingConfiguration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "loggingConfig".into(),
                    value: &logging_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "loggingConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InvocationLoggingConfigurationResult {
            logging_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingConfig").unwrap(),
            ),
        }
    }
}
