#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEnvironmentArgs {
        /// ID of the AppConfig Application to which this Environment belongs.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the AppConfig Environment.
        #[builder(into)]
        pub environment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetEnvironmentResult {
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the environment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the environment.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub environment_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set of Amazon CloudWatch alarms to monitor during the deployment process.
        pub monitors: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appconfig::GetEnvironmentMonitor>,
        >,
        /// Name of the environment.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// State of the environment. Possible values are `READY_FOR_DEPLOYMENT`, `DEPLOYING`, `ROLLING_BACK`
        /// or `ROLLED_BACK`.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Map of tags for the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetEnvironmentArgs,
    ) -> GetEnvironmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_id_binding = args.application_id.get_output(context);
        let environment_id_binding = args.environment_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:appconfig/getEnvironment:getEnvironment".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: application_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environmentId".into(),
                    value: environment_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetEnvironmentResult {
            application_id: o.get_field("applicationId"),
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            environment_id: o.get_field("environmentId"),
            id: o.get_field("id"),
            monitors: o.get_field("monitors"),
            name: o.get_field("name"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
        }
    }
}
