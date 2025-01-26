pub mod get_cloud_formation_type {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCloudFormationTypeArgs {
        /// ARN of the CloudFormation Type. For example, `arn:aws:cloudformation:us-west-2::type/resource/AWS-EC2-VPC`.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// CloudFormation Registry Type. For example, `RESOURCE`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// CloudFormation Type name. For example, `AWS::EC2::VPC`.
        #[builder(into, default)]
        pub type_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Identifier of the CloudFormation Type version.
        #[builder(into, default)]
        pub version_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCloudFormationTypeResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Identifier of the CloudFormation Type default version.
        pub default_version_id: pulumi_wasm_rust::Output<String>,
        /// Deprecation status of the CloudFormation Type.
        pub deprecated_status: pulumi_wasm_rust::Output<String>,
        /// Description of the CloudFormation Type.
        pub description: pulumi_wasm_rust::Output<String>,
        /// URL of the documentation for the CloudFormation Type.
        pub documentation_url: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM Role used to register the CloudFormation Type.
        pub execution_role_arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Whether the CloudFormation Type version is the default version.
        pub is_default_version: pulumi_wasm_rust::Output<bool>,
        /// List of objects containing logging configuration.
        pub logging_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudformation::GetCloudFormationTypeLoggingConfig,
            >,
        >,
        /// Provisioning behavior of the CloudFormation Type.
        pub provisioning_type: pulumi_wasm_rust::Output<String>,
        /// JSON document of the CloudFormation Type schema.
        pub schema: pulumi_wasm_rust::Output<String>,
        /// URL of the source code for the CloudFormation Type.
        pub source_url: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub type_arn: pulumi_wasm_rust::Output<String>,
        pub type_name: pulumi_wasm_rust::Output<String>,
        pub version_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Scope of the CloudFormation Type.
        pub visibility: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetCloudFormationTypeArgs,
    ) -> GetCloudFormationTypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let type_name_binding = args.type_name.get_output(context).get_inner();
        let version_id_binding = args.version_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudformation/getCloudFormationType:getCloudFormationType"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "typeName".into(),
                    value: &type_name_binding,
                },
                register_interface::ObjectField {
                    name: "versionId".into(),
                    value: &version_id_binding,
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "isDefaultVersion".into(),
                },
                register_interface::ResultField {
                    name: "loggingConfigs".into(),
                },
                register_interface::ResultField {
                    name: "provisioningType".into(),
                },
                register_interface::ResultField {
                    name: "schema".into(),
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
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCloudFormationTypeResult {
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            is_default_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isDefaultVersion").unwrap(),
            ),
            logging_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingConfigs").unwrap(),
            ),
            provisioning_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisioningType").unwrap(),
            ),
            schema: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schema").unwrap(),
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
