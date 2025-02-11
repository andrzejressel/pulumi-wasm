#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_code_signing_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCodeSigningConfigArgs {
        /// ARN of the code signing configuration.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCodeSigningConfigResult {
        /// List of allowed publishers as signing profiles for this code signing configuration.
        pub allowed_publishers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::lambda::GetCodeSigningConfigAllowedPublisher>,
        >,
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier for the code signing configuration.
        pub config_id: pulumi_gestalt_rust::Output<String>,
        /// Code signing configuration description.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Date and time that the code signing configuration was last modified.
        pub last_modified: pulumi_gestalt_rust::Output<String>,
        /// List of code signing policies that control the validation failure action for signature mismatch or expiry.
        pub policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::lambda::GetCodeSigningConfigPolicy>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCodeSigningConfigArgs,
    ) -> GetCodeSigningConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:lambda/getCodeSigningConfig:getCodeSigningConfig".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCodeSigningConfigResult {
            allowed_publishers: o.get_field("allowedPublishers"),
            arn: o.get_field("arn"),
            config_id: o.get_field("configId"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            last_modified: o.get_field("lastModified"),
            policies: o.get_field("policies"),
        }
    }
}
