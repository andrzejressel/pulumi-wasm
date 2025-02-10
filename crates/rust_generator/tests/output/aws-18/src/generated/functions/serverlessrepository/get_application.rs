#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApplicationArgs {
        /// ARN of the application.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Requested version of the application. By default, retrieves the latest version.
        #[builder(into, default)]
        pub semantic_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetApplicationResult {
        /// ARN of the application.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the application.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of capabilities describing the permissions needed to deploy the application.
        pub required_capabilities: pulumi_gestalt_rust::Output<Vec<String>>,
        pub semantic_version: pulumi_gestalt_rust::Output<String>,
        /// URL pointing to the source code of the application version.
        pub source_code_url: pulumi_gestalt_rust::Output<String>,
        /// URL pointing to the Cloud Formation template for the application version.
        pub template_url: pulumi_gestalt_rust::Output<String>,
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
        let application_id_binding = args.application_id.get_output(context);
        let semantic_version_binding = args.semantic_version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:serverlessrepository/getApplication:getApplication".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: application_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "semanticVersion".into(),
                    value: semantic_version_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetApplicationResult {
            application_id: o.get_field("applicationId"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            required_capabilities: o.get_field("requiredCapabilities"),
            semantic_version: o.get_field("semanticVersion"),
            source_code_url: o.get_field("sourceCodeUrl"),
            template_url: o.get_field("templateUrl"),
        }
    }
}
