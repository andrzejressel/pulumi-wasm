/// Manages a version of a CloudFormation Type.
///
///
///
/// ## Import
///
/// Using `pulumi import`, import `aws_cloudformation_type` using the type version Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:cloudformation/cloudFormationType:CloudFormationType example arn:aws:cloudformation:us-east-1:123456789012:type/resource/ExampleCompany-ExampleService-ExampleType/1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cloud_formation_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CloudFormationTypeArgs {
        /// Amazon Resource Name (ARN) of the IAM Role for CloudFormation to assume when invoking the extension. If your extension calls AWS APIs in any of its handlers, you must create an IAM execution role that includes the necessary permissions to call those AWS APIs, and provision that execution role in your account. When CloudFormation needs to invoke the extension handler, CloudFormation assumes this execution role to create a temporary session token, which it then passes to the extension handler, thereby supplying your extension with the appropriate credentials.
        #[builder(into, default)]
        pub execution_role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block containing logging configuration.
        #[builder(into, default)]
        pub logging_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudformation::CloudFormationTypeLoggingConfig>,
        >,
        /// URL to the S3 bucket containing the extension project package that contains the necessary files for the extension you want to register. Must begin with `s3://` or `https://`. For example, `s3://example-bucket/example-object`.
        #[builder(into)]
        pub schema_handler_package: pulumi_gestalt_rust::InputOrOutput<String>,
        /// CloudFormation Registry Type. For example, `RESOURCE` or `MODULE`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// CloudFormation Type name. For example, `ExampleCompany::ExampleService::ExampleResource`.
        #[builder(into)]
        pub type_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CloudFormationTypeResult {
        /// (Optional) Amazon Resource Name (ARN) of the CloudFormation Type version. See also `type_arn`.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the CloudFormation Type default version.
        pub default_version_id: pulumi_gestalt_rust::Output<String>,
        /// Deprecation status of the version.
        pub deprecated_status: pulumi_gestalt_rust::Output<String>,
        /// Description of the version.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// URL of the documentation for the CloudFormation Type.
        pub documentation_url: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the IAM Role for CloudFormation to assume when invoking the extension. If your extension calls AWS APIs in any of its handlers, you must create an IAM execution role that includes the necessary permissions to call those AWS APIs, and provision that execution role in your account. When CloudFormation needs to invoke the extension handler, CloudFormation assumes this execution role to create a temporary session token, which it then passes to the extension handler, thereby supplying your extension with the appropriate credentials.
        pub execution_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the CloudFormation Type version is the default version.
        pub is_default_version: pulumi_gestalt_rust::Output<bool>,
        /// Configuration block containing logging configuration.
        pub logging_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudformation::CloudFormationTypeLoggingConfig>,
        >,
        /// Provisioning behavior of the CloudFormation Type.
        pub provisioning_type: pulumi_gestalt_rust::Output<String>,
        /// JSON document of the CloudFormation Type schema.
        pub schema: pulumi_gestalt_rust::Output<String>,
        /// URL to the S3 bucket containing the extension project package that contains the necessary files for the extension you want to register. Must begin with `s3://` or `https://`. For example, `s3://example-bucket/example-object`.
        pub schema_handler_package: pulumi_gestalt_rust::Output<String>,
        /// URL of the source code for the CloudFormation Type.
        pub source_url: pulumi_gestalt_rust::Output<String>,
        /// CloudFormation Registry Type. For example, `RESOURCE` or `MODULE`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// (Optional) Amazon Resource Name (ARN) of the CloudFormation Type. See also `arn`.
        pub type_arn: pulumi_gestalt_rust::Output<String>,
        /// CloudFormation Type name. For example, `ExampleCompany::ExampleService::ExampleResource`.
        pub type_name: pulumi_gestalt_rust::Output<String>,
        /// (Optional) Identifier of the CloudFormation Type version.
        pub version_id: pulumi_gestalt_rust::Output<String>,
        /// Scope of the CloudFormation Type.
        pub visibility: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CloudFormationTypeArgs,
    ) -> CloudFormationTypeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let execution_role_arn_binding = args.execution_role_arn.get_output(context);
        let logging_config_binding = args.logging_config.get_output(context);
        let schema_handler_package_binding = args
            .schema_handler_package
            .get_output(context);
        let type__binding = args.type_.get_output(context);
        let type_name_binding = args.type_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudformation/cloudFormationType:CloudFormationType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "executionRoleArn".into(),
                    value: &execution_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loggingConfig".into(),
                    value: &logging_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schemaHandlerPackage".into(),
                    value: &schema_handler_package_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "typeName".into(),
                    value: &type_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CloudFormationTypeResult {
            arn: o.get_field("arn"),
            default_version_id: o.get_field("defaultVersionId"),
            deprecated_status: o.get_field("deprecatedStatus"),
            description: o.get_field("description"),
            documentation_url: o.get_field("documentationUrl"),
            execution_role_arn: o.get_field("executionRoleArn"),
            is_default_version: o.get_field("isDefaultVersion"),
            logging_config: o.get_field("loggingConfig"),
            provisioning_type: o.get_field("provisioningType"),
            schema: o.get_field("schema"),
            schema_handler_package: o.get_field("schemaHandlerPackage"),
            source_url: o.get_field("sourceUrl"),
            type_: o.get_field("type"),
            type_arn: o.get_field("typeArn"),
            type_name: o.get_field("typeName"),
            version_id: o.get_field("versionId"),
            visibility: o.get_field("visibility"),
        }
    }
}
