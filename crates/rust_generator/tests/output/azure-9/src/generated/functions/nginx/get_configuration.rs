#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationArgs {
        /// The ID of the Nginx Deployment.
        #[builder(into)]
        pub nginx_deployment_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetConfigurationResult {
        /// A `config_file` block as defined below.
        pub config_files: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::nginx::GetConfigurationConfigFile>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub nginx_deployment_id: pulumi_gestalt_rust::Output<String>,
        /// The package data for this configuration.
        pub package_data: pulumi_gestalt_rust::Output<String>,
        /// A `protected_file` block as defined below.
        pub protected_files: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::nginx::GetConfigurationProtectedFile>,
        >,
        /// The root file path of this Nginx Configuration.
        pub root_file: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetConfigurationArgs,
    ) -> GetConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let nginx_deployment_id_binding = args.nginx_deployment_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:nginx/getConfiguration:getConfiguration".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nginxDeploymentId".into(),
                    value: &nginx_deployment_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetConfigurationResult {
            config_files: o.get_field("configFiles"),
            id: o.get_field("id"),
            nginx_deployment_id: o.get_field("nginxDeploymentId"),
            package_data: o.get_field("packageData"),
            protected_files: o.get_field("protectedFiles"),
            root_file: o.get_field("rootFile"),
        }
    }
}
