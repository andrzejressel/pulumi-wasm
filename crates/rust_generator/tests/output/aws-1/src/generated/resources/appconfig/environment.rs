/// Provides an AppConfig Environment resource for an `aws.appconfig.Application` resource. One or more environments can be defined for an application.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appconfig:Environment
///     properties:
///       name: example-environment-tf
///       description: Example AppConfig Environment
///       applicationId: ${exampleApplication.id}
///       monitors:
///         - alarmArn: ${exampleAwsCloudwatchMetricAlarm.arn}
///           alarmRoleArn: ${exampleAwsIamRole.arn}
///       tags:
///         Type: AppConfig Environment
///   exampleApplication:
///     type: aws:appconfig:Application
///     name: example
///     properties:
///       name: example-application-tf
///       description: Example AppConfig Application
///       tags:
///         Type: AppConfig Application
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppConfig Environments using the environment ID and application ID separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:appconfig/environment:Environment example 71abcde:11xxxxx
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentArgs {
        /// AppConfig application ID. Must be between 4 and 7 characters in length.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the environment. Can be at most 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of Amazon CloudWatch alarms to monitor during the deployment process. Maximum of 5. See Monitor below for more details.
        #[builder(into, default)]
        pub monitors: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appconfig::EnvironmentMonitor>>,
        >,
        /// Name for the environment. Must be between 1 and 64 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnvironmentResult {
        /// AppConfig application ID. Must be between 4 and 7 characters in length.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the AppConfig Environment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the environment. Can be at most 1024 characters.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// AppConfig environment ID.
        pub environment_id: pulumi_gestalt_rust::Output<String>,
        /// Set of Amazon CloudWatch alarms to monitor during the deployment process. Maximum of 5. See Monitor below for more details.
        pub monitors: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::appconfig::EnvironmentMonitor>>,
        >,
        /// Name for the environment. Must be between 1 and 64 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// State of the environment. Possible values are `READY_FOR_DEPLOYMENT`, `DEPLOYING`, `ROLLING_BACK`
        /// or `ROLLED_BACK`.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvironmentArgs,
    ) -> EnvironmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_id_binding = args.application_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let monitors_binding = args.monitors.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appconfig/environment:Environment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitors".into(),
                    value: &monitors_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvironmentResult {
            application_id: o.get_field("applicationId"),
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            environment_id: o.get_field("environmentId"),
            monitors: o.get_field("monitors"),
            name: o.get_field("name"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
