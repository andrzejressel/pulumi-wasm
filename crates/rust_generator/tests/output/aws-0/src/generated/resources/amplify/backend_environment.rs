/// Provides an Amplify Backend Environment resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = app::create(
///         "example",
///         AppArgs::builder().name("example").build_struct(),
///     );
///     let exampleBackendEnvironment = backend_environment::create(
///         "exampleBackendEnvironment",
///         BackendEnvironmentArgs::builder()
///             .app_id("${example.id}")
///             .deployment_artifacts("app-example-deployment")
///             .environment_name("example")
///             .stack_name("amplify-app-example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amplify backend environment using `app_id` and `environment_name`. For example:
///
/// ```sh
/// $ pulumi import aws:amplify/backendEnvironment:BackendEnvironment example d2ypk4k47z8u6/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backend_environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackendEnvironmentArgs {
        /// Unique ID for an Amplify app.
        #[builder(into)]
        pub app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of deployment artifacts.
        #[builder(into, default)]
        pub deployment_artifacts: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name for the backend environment.
        #[builder(into)]
        pub environment_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS CloudFormation stack name of a backend environment.
        #[builder(into, default)]
        pub stack_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BackendEnvironmentResult {
        /// Unique ID for an Amplify app.
        pub app_id: pulumi_gestalt_rust::Output<String>,
        /// ARN for a backend environment that is part of an Amplify app.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of deployment artifacts.
        pub deployment_artifacts: pulumi_gestalt_rust::Output<String>,
        /// Name for the backend environment.
        pub environment_name: pulumi_gestalt_rust::Output<String>,
        /// AWS CloudFormation stack name of a backend environment.
        pub stack_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BackendEnvironmentArgs,
    ) -> BackendEnvironmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_id_binding_1 = args.app_id.get_output(context);
        let app_id_binding = app_id_binding_1.get_inner();
        let deployment_artifacts_binding_1 = args
            .deployment_artifacts
            .get_output(context);
        let deployment_artifacts_binding = deployment_artifacts_binding_1.get_inner();
        let environment_name_binding_1 = args.environment_name.get_output(context);
        let environment_name_binding = environment_name_binding_1.get_inner();
        let stack_name_binding_1 = args.stack_name.get_output(context);
        let stack_name_binding = stack_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:amplify/backendEnvironment:BackendEnvironment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appId".into(),
                    value: &app_id_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentArtifacts".into(),
                    value: &deployment_artifacts_binding,
                },
                register_interface::ObjectField {
                    name: "environmentName".into(),
                    value: &environment_name_binding,
                },
                register_interface::ObjectField {
                    name: "stackName".into(),
                    value: &stack_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BackendEnvironmentResult {
            app_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appId"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            deployment_artifacts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deploymentArtifacts"),
            ),
            environment_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environmentName"),
            ),
            stack_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stackName"),
            ),
        }
    }
}
