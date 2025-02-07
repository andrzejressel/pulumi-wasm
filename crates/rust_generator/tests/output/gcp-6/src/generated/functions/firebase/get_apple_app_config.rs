pub mod get_apple_app_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAppleAppConfigArgs {
        /// The id of the Firebase iOS App.
        ///
        /// - - -
        #[builder(into)]
        pub app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAppleAppConfigResult {
        pub app_id: pulumi_gestalt_rust::Output<String>,
        /// The content of the XML configuration file as a base64-encoded string.
        pub config_file_contents: pulumi_gestalt_rust::Output<String>,
        /// The filename that the configuration artifact for the IosApp is typically saved as.
        pub config_filename: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAppleAppConfigArgs,
    ) -> GetAppleAppConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_id_binding = args.app_id.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:firebase/getAppleAppConfig:getAppleAppConfig".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appId".into(),
                    value: &app_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAppleAppConfigResult {
            app_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appId"),
            ),
            config_file_contents: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configFileContents"),
            ),
            config_filename: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configFilename"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
        }
    }
}
