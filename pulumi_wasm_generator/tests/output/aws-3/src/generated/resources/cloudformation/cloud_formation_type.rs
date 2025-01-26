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
pub mod cloud_formation_type {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CloudFormationTypeArgs {
        /// Amazon Resource Name (ARN) of the IAM Role for CloudFormation to assume when invoking the extension. If your extension calls AWS APIs in any of its handlers, you must create an IAM execution role that includes the necessary permissions to call those AWS APIs, and provision that execution role in your account. When CloudFormation needs to invoke the extension handler, CloudFormation assumes this execution role to create a temporary session token, which it then passes to the extension handler, thereby supplying your extension with the appropriate credentials.
        #[builder(into, default)]
        pub execution_role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block containing logging configuration.
        #[builder(into, default)]
        pub logging_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cloudformation::CloudFormationTypeLoggingConfig>,
        >,
        /// URL to the S3 bucket containing the extension project package that contains the necessary files for the extension you want to register. Must begin with `s3://` or `https://`. For example, `s3://example-bucket/example-object`.
        #[builder(into)]
        pub schema_handler_package: pulumi_wasm_rust::InputOrOutput<String>,
        /// CloudFormation Registry Type. For example, `RESOURCE` or `MODULE`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// CloudFormation Type name. For example, `ExampleCompany::ExampleService::ExampleResource`.
        #[builder(into)]
        pub type_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CloudFormationTypeResult {
        /// (Optional) Amazon Resource Name (ARN) of the CloudFormation Type version. See also `type_arn`.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Identifier of the CloudFormation Type default version.
        pub default_version_id: pulumi_wasm_rust::Output<String>,
        /// Deprecation status of the version.
        pub deprecated_status: pulumi_wasm_rust::Output<String>,
        /// Description of the version.
        pub description: pulumi_wasm_rust::Output<String>,
        /// URL of the documentation for the CloudFormation Type.
        pub documentation_url: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the IAM Role for CloudFormation to assume when invoking the extension. If your extension calls AWS APIs in any of its handlers, you must create an IAM execution role that includes the necessary permissions to call those AWS APIs, and provision that execution role in your account. When CloudFormation needs to invoke the extension handler, CloudFormation assumes this execution role to create a temporary session token, which it then passes to the extension handler, thereby supplying your extension with the appropriate credentials.
        pub execution_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the CloudFormation Type version is the default version.
        pub is_default_version: pulumi_wasm_rust::Output<bool>,
        /// Configuration block containing logging configuration.
        pub logging_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudformation::CloudFormationTypeLoggingConfig>,
        >,
        /// Provisioning behavior of the CloudFormation Type.
        pub provisioning_type: pulumi_wasm_rust::Output<String>,
        /// JSON document of the CloudFormation Type schema.
        pub schema: pulumi_wasm_rust::Output<String>,
        /// URL to the S3 bucket containing the extension project package that contains the necessary files for the extension you want to register. Must begin with `s3://` or `https://`. For example, `s3://example-bucket/example-object`.
        pub schema_handler_package: pulumi_wasm_rust::Output<String>,
        /// URL of the source code for the CloudFormation Type.
        pub source_url: pulumi_wasm_rust::Output<String>,
        /// CloudFormation Registry Type. For example, `RESOURCE` or `MODULE`.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// (Optional) Amazon Resource Name (ARN) of the CloudFormation Type. See also `arn`.
        pub type_arn: pulumi_wasm_rust::Output<String>,
        /// CloudFormation Type name. For example, `ExampleCompany::ExampleService::ExampleResource`.
        pub type_name: pulumi_wasm_rust::Output<String>,
        /// (Optional) Identifier of the CloudFormation Type version.
        pub version_id: pulumi_wasm_rust::Output<String>,
        /// Scope of the CloudFormation Type.
        pub visibility: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CloudFormationTypeArgs,
    ) -> CloudFormationTypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let execution_role_arn_binding = args
            .execution_role_arn
            .get_output(context)
            .get_inner();
        let logging_config_binding = args.logging_config.get_output(context).get_inner();
        let schema_handler_package_binding = args
            .schema_handler_package
            .get_output(context)
            .get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let type_name_binding = args.type_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudformation/cloudFormationType:CloudFormationType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "executionRoleArn".into(),
                    value: &execution_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "loggingConfig".into(),
                    value: &logging_config_binding,
                },
                register_interface::ObjectField {
                    name: "schemaHandlerPackage".into(),
                    value: &schema_handler_package_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "typeName".into(),
                    value: &type_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "defaultVersionId".into(),
                },
                register_interface::ResultField {
                    name: "deprecatedStatus".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "documentationUrl".into(),
                },
                register_interface::ResultField {
                    name: "executionRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "isDefaultVersion".into(),
                },
                register_interface::ResultField {
                    name: "loggingConfig".into(),
                },
                register_interface::ResultField {
                    name: "provisioningType".into(),
                },
                register_interface::ResultField {
                    name: "schema".into(),
                },
                register_interface::ResultField {
                    name: "schemaHandlerPackage".into(),
                },
                register_interface::ResultField {
                    name: "sourceUrl".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "typeArn".into(),
                },
                register_interface::ResultField {
                    name: "typeName".into(),
                },
                register_interface::ResultField {
                    name: "versionId".into(),
                },
                register_interface::ResultField {
                    name: "visibility".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CloudFormationTypeResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            default_version_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultVersionId").unwrap(),
            ),
            deprecated_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deprecatedStatus").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            documentation_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("documentationUrl").unwrap(),
            ),
            execution_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionRoleArn").unwrap(),
            ),
            is_default_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isDefaultVersion").unwrap(),
            ),
            logging_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingConfig").unwrap(),
            ),
            provisioning_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisioningType").unwrap(),
            ),
            schema: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schema").unwrap(),
            ),
            schema_handler_package: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schemaHandlerPackage").unwrap(),
            ),
            source_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceUrl").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            type_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("typeArn").unwrap(),
            ),
            type_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("typeName").unwrap(),
            ),
            version_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionId").unwrap(),
            ),
            visibility: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("visibility").unwrap(),
            ),
        }
    }
}
