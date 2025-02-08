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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ProjectArgs,
    ) -> ProjectResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auto_update_binding = args.auto_update.get_output(context).get_inner();
        let feature_binding = args.feature.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rekognition/project:Project".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoUpdate".into(),
                    value: &auto_update_binding,
                },
                register_interface::ObjectField {
                    name: "feature".into(),
                    value: &feature_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProjectResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auto_update: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoUpdate"),
            ),
            feature: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("feature"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
