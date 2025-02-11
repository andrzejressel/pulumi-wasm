#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApplicationArgs {
        /// ARN of the application.
        #[builder(into)]
        pub application_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Options for the portal associated with an application. See the `aws.ssoadmin.Application` resource documentation. The attributes are the same.
        #[builder(into, default)]
        pub portal_options: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ssoadmin::GetApplicationPortalOption>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetApplicationResult {
        /// AWS account ID.
        pub application_account: pulumi_gestalt_rust::Output<String>,
        pub application_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the application provider.
        pub application_provider_arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the application.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// ARN of the application.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the instance of IAM Identity Center.
        pub instance_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the application.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Options for the portal associated with an application. See the `aws.ssoadmin.Application` resource documentation. The attributes are the same.
        pub portal_options: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ssoadmin::GetApplicationPortalOption>>,
        >,
        /// Status of the application.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetApplicationArgs,
    ) -> GetApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_arn_binding = args.application_arn.get_output(context);
        let portal_options_binding = args.portal_options.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ssoadmin/getApplication:getApplication".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationArn".into(),
                    value: &application_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "portalOptions".into(),
                    value: &portal_options_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetApplicationResult {
            application_account: o.get_field("applicationAccount"),
            application_arn: o.get_field("applicationArn"),
            application_provider_arn: o.get_field("applicationProviderArn"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            instance_arn: o.get_field("instanceArn"),
            name: o.get_field("name"),
            portal_options: o.get_field("portalOptions"),
            status: o.get_field("status"),
        }
    }
}
