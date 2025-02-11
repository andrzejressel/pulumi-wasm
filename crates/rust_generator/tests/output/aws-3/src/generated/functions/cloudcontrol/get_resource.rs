#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourceArgs {
        /// Identifier of the CloudFormation resource type. For example, `vpc-12345678`.
        #[builder(into)]
        pub identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the IAM Role to assume for operations.
        #[builder(into, default)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// CloudFormation resource type name. For example, `AWS::EC2::VPC`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of the CloudFormation resource type version.
        #[builder(into, default)]
        pub type_version_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetResourceResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub identifier: pulumi_gestalt_rust::Output<String>,
        /// JSON string matching the CloudFormation resource type schema with current configuration.
        pub properties: pulumi_gestalt_rust::Output<String>,
        pub role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        pub type_name: pulumi_gestalt_rust::Output<String>,
        pub type_version_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetResourceArgs,
    ) -> GetResourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identifier_binding = args.identifier.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let type_name_binding = args.type_name.get_output(context);
        let type_version_id_binding = args.type_version_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cloudcontrol/getResource:getResource".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "typeName".into(),
                    value: &type_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "typeVersionId".into(),
                    value: &type_version_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResourceResult {
            id: o.get_field("id"),
            identifier: o.get_field("identifier"),
            properties: o.get_field("properties"),
            role_arn: o.get_field("roleArn"),
            type_name: o.get_field("typeName"),
            type_version_id: o.get_field("typeVersionId"),
        }
    }
}
