/// Provides a resource to manage AWS Secrets Manager secret rotation. To manage a secret, see the `aws.secretsmanager.Secret` resource. To manage a secret value, see the `aws.secretsmanager.SecretVersion` resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = secret_rotation::create(
///         "example",
///         SecretRotationArgs::builder()
///             .rotation_lambda_arn("${exampleAwsLambdaFunction.arn}")
///             .rotation_rules(
///                 SecretRotationRotationRules::builder()
///                     .automaticallyAfterDays(30)
///                     .build_struct(),
///             )
///             .secret_id("${exampleAwsSecretsmanagerSecret.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Rotation Configuration
///
/// To enable automatic secret rotation, the Secrets Manager service requires usage of a Lambda function. The [Rotate Secrets section in the Secrets Manager User Guide](https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotating-secrets.html) provides additional information about deploying a prebuilt Lambda functions for supported credential rotation (e.g., RDS) or deploying a custom Lambda function.
///
/// > **NOTE:** Configuring rotation causes the secret to rotate once as soon as you enable rotation. Before you do this, you must ensure that all of your applications that use the credentials stored in the secret are updated to retrieve the secret from AWS Secrets Manager. The old credentials might no longer be usable after the initial rotation and any applications that you fail to update will break as soon as the old credentials are no longer valid.
///
/// > **NOTE:** If you cancel a rotation that is in progress (by removing the `rotation` configuration), it can leave the VersionStage labels in an unexpected state. Depending on what step of the rotation was in progress, you might need to remove the staging label AWSPENDING from the partially created version, specified by the SecretVersionId response value. You should also evaluate the partially rotated new version to see if it should be deleted, which you can do by removing all staging labels from the new version's VersionStage field.
///
/// ## Import
///
/// Using `pulumi import`, import `aws_secretsmanager_secret_rotation` using the secret Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:secretsmanager/secretRotation:SecretRotation example arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod secret_rotation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecretRotationArgs {
        /// Specifies whether to rotate the secret immediately or wait until the next scheduled rotation window. The rotation schedule is defined in `rotation_rules`. For secrets that use a Lambda rotation function to rotate, if you don't immediately rotate the secret, Secrets Manager tests the rotation configuration by running the testSecret step (https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_how.html) of the Lambda rotation function. The test creates an AWSPENDING version of the secret and then removes it. Defaults to `true`.
        #[builder(into, default)]
        pub rotate_immediately: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the ARN of the Lambda function that can rotate the secret. Must be supplied if the secret is not managed by AWS.
        #[builder(into, default)]
        pub rotation_lambda_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A structure that defines the rotation configuration for this secret. Defined below.
        #[builder(into)]
        pub rotation_rules: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::secretsmanager::SecretRotationRotationRules,
        >,
        /// Specifies the secret to which you want to add a new version. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret. The secret must already exist.
        #[builder(into)]
        pub secret_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SecretRotationResult {
        /// Specifies whether to rotate the secret immediately or wait until the next scheduled rotation window. The rotation schedule is defined in `rotation_rules`. For secrets that use a Lambda rotation function to rotate, if you don't immediately rotate the secret, Secrets Manager tests the rotation configuration by running the testSecret step (https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_how.html) of the Lambda rotation function. The test creates an AWSPENDING version of the secret and then removes it. Defaults to `true`.
        pub rotate_immediately: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether automatic rotation is enabled for this secret.
        pub rotation_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the ARN of the Lambda function that can rotate the secret. Must be supplied if the secret is not managed by AWS.
        pub rotation_lambda_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// A structure that defines the rotation configuration for this secret. Defined below.
        pub rotation_rules: pulumi_gestalt_rust::Output<
            super::super::types::secretsmanager::SecretRotationRotationRules,
        >,
        /// Specifies the secret to which you want to add a new version. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret. The secret must already exist.
        pub secret_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecretRotationArgs,
    ) -> SecretRotationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let rotate_immediately_binding = args.rotate_immediately.get_output(context);
        let rotation_lambda_arn_binding = args.rotation_lambda_arn.get_output(context);
        let rotation_rules_binding = args.rotation_rules.get_output(context);
        let secret_id_binding = args.secret_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:secretsmanager/secretRotation:SecretRotation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rotateImmediately".into(),
                    value: rotate_immediately_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rotationLambdaArn".into(),
                    value: rotation_lambda_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rotationRules".into(),
                    value: rotation_rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretId".into(),
                    value: secret_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecretRotationResult {
            rotate_immediately: o.get_field("rotateImmediately"),
            rotation_enabled: o.get_field("rotationEnabled"),
            rotation_lambda_arn: o.get_field("rotationLambdaArn"),
            rotation_rules: o.get_field("rotationRules"),
            secret_id: o.get_field("secretId"),
        }
    }
}
