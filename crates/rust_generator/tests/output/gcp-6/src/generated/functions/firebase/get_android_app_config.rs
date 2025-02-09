#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_android_app_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAndroidAppConfigArgs {
        #[builder(into)]
        pub app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAndroidAppConfigResult {
        pub app_id: pulumi_gestalt_rust::Output<String>,
        pub config_file_contents: pulumi_gestalt_rust::Output<String>,
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
        args: GetAndroidAppConfigArgs,
    ) -> GetAndroidAppConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_id_binding_1 = args.app_id.get_output(context);
        let app_id_binding = app_id_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:firebase/getAndroidAppConfig:getAndroidAppConfig".into(),
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
        GetAndroidAppConfigResult {
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
