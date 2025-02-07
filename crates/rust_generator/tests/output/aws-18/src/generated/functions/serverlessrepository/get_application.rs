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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetApplicationArgs,
    ) -> GetApplicationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_output(context).get_inner();
        let semantic_version_binding = args
            .semantic_version
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:serverlessrepository/getApplication:getApplication".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "semanticVersion".into(),
                    value: &semantic_version_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetApplicationResult {
            application_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            required_capabilities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requiredCapabilities"),
            ),
            semantic_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("semanticVersion"),
            ),
            source_code_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceCodeUrl"),
            ),
            template_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateUrl"),
            ),
        }
    }
}
