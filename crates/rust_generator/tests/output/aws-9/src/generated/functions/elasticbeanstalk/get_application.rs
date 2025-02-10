#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApplicationArgs {
        /// Name of the application
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetApplicationResult {
        pub appversion_lifecycle: pulumi_gestalt_rust::Output<
            super::super::super::types::elasticbeanstalk::GetApplicationAppversionLifecycle,
        >,
        /// ARN of the application.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Short description of the application
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
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
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:elasticbeanstalk/getApplication:getApplication".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetApplicationResult {
            appversion_lifecycle: o.get_field("appversionLifecycle"),
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
        }
    }
}
