pub mod get_cloud_formation_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCloudFormationTypeArgs {
        /// ARN of the CloudFormation Type. For example, `arn:aws:cloudformation:us-west-2::type/resource/AWS-EC2-VPC`.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// CloudFormation Registry Type. For example, `RESOURCE`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// CloudFormation Type name. For example, `AWS::EC2::VPC`.
        #[builder(into, default)]
        pub type_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the CloudFormation Type version.
        #[builder(into, default)]
        pub version_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCloudFormationTypeResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the CloudFormation Type default version.
        pub default_version_id: pulumi_gestalt_rust::Output<String>,
        /// Deprecation status of the CloudFormation Type.
        pub deprecated_status: pulumi_gestalt_rust::Output<String>,
        /// Description of the CloudFormation Type.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// URL of the documentation for the CloudFormation Type.
        pub documentation_url: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM Role used to register the CloudFormation Type.
        pub execution_role_arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Whether the CloudFormation Type version is the default version.
        pub is_default_version: pulumi_gestalt_rust::Output<bool>,
        /// List of objects containing logging configuration.
        pub logging_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::cloudformation::GetCloudFormationTypeLoggingConfig,
            >,
        >,
        /// Provisioning behavior of the CloudFormation Type.
        pub provisioning_type: pulumi_gestalt_rust::Output<String>,
        /// JSON document of the CloudFormation Type schema.
        pub schema: pulumi_gestalt_rust::Output<String>,
        /// URL of the source code for the CloudFormation Type.
        pub source_url: pulumi_gestalt_rust::Output<String>,
        pub type_: pulumi_gestalt_rust::Output<String>,
        pub type_arn: pulumi_gestalt_rust::Output<String>,
        pub type_name: pulumi_gestalt_rust::Output<String>,
        pub version_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Scope of the CloudFormation Type.
        pub visibility: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetCloudFormationTypeArgs,
    ) -> GetCloudFormationTypeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCloudFormationTypeResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            default_version_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultVersionId"),
            ),
            deprecated_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deprecatedStatus"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            documentation_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("documentationUrl"),
            ),
            execution_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("executionRoleArn"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            is_default_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isDefaultVersion"),
            ),
            logging_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loggingConfigs"),
            ),
            provisioning_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("provisioningType"),
            ),
            schema: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schema"),
            ),
            source_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceUrl"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            type_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("typeArn"),
            ),
            type_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("typeName"),
            ),
            version_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionId"),
            ),
            visibility: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("visibility"),
            ),
        }
    }
}
