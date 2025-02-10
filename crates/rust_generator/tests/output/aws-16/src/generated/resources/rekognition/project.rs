/// Resource for managing an AWS Rekognition Project.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = project::create(
///         "example",
///         ProjectArgs::builder()
///             .auto_update("ENABLED")
///             .feature("CONTENT_MODERATION")
///             .name("example-project")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Rekognition Project using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:rekognition/project:Project example project-id-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// Specify if automatic retraining should occur. Valid values are `ENABLED` or `DISABLED`. Defaults to `DISABLED`
        #[builder(into, default)]
        pub auto_update: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specify the feature being customized. Valid values are `CONTENT_MODERATION` or `CUSTOM_LABELS`. Defaults to `CUSTOM_LABELS`
        #[builder(into, default)]
        pub feature: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Desired name of the project
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rekognition::ProjectTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProjectResult {
        /// ARN of the Project.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specify if automatic retraining should occur. Valid values are `ENABLED` or `DISABLED`. Defaults to `DISABLED`
        pub auto_update: pulumi_gestalt_rust::Output<String>,
        /// Specify the feature being customized. Valid values are `CONTENT_MODERATION` or `CUSTOM_LABELS`. Defaults to `CUSTOM_LABELS`
        pub feature: pulumi_gestalt_rust::Output<Option<String>>,
        /// Desired name of the project
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::rekognition::ProjectTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProjectArgs,
    ) -> ProjectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_update_binding = args.auto_update.get_output(context);
        let feature_binding = args.feature.get_output(context);
        let name_binding = args.name.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rekognition/project:Project".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoUpdate".into(),
                    value: auto_update_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "feature".into(),
                    value: feature_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProjectResult {
            arn: o.get_field("arn"),
            auto_update: o.get_field("autoUpdate"),
            feature: o.get_field("feature"),
            name: o.get_field("name"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
