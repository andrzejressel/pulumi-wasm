#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetCloudFormationTypeArgs,
    ) -> GetCloudFormationTypeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let type__binding = args.type_.get_output(context);
        let type_name_binding = args.type_name.get_output(context);
        let version_id_binding = args.version_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cloudformation/getCloudFormationType:getCloudFormationType"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "typeName".into(),
                    value: type_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionId".into(),
                    value: version_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCloudFormationTypeResult {
            arn: o.get_field("arn"),
            default_version_id: o.get_field("defaultVersionId"),
            deprecated_status: o.get_field("deprecatedStatus"),
            description: o.get_field("description"),
            documentation_url: o.get_field("documentationUrl"),
            execution_role_arn: o.get_field("executionRoleArn"),
            id: o.get_field("id"),
            is_default_version: o.get_field("isDefaultVersion"),
            logging_configs: o.get_field("loggingConfigs"),
            provisioning_type: o.get_field("provisioningType"),
            schema: o.get_field("schema"),
            source_url: o.get_field("sourceUrl"),
            type_: o.get_field("type"),
            type_arn: o.get_field("typeArn"),
            type_name: o.get_field("typeName"),
            version_id: o.get_field("versionId"),
            visibility: o.get_field("visibility"),
        }
    }
}
