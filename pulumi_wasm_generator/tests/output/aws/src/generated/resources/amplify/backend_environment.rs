/// Provides an Amplify Backend Environment resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod backend_environment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackendEnvironmentArgs {
        /// Unique ID for an Amplify app.
        #[builder(into)]
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// Name of deployment artifacts.
        #[builder(into, default)]
        pub deployment_artifacts: pulumi_wasm_rust::Output<Option<String>>,
        /// Name for the backend environment.
        #[builder(into)]
        pub environment_name: pulumi_wasm_rust::Output<String>,
        /// AWS CloudFormation stack name of a backend environment.
        #[builder(into, default)]
        pub stack_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BackendEnvironmentResult {
        /// Unique ID for an Amplify app.
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// ARN for a backend environment that is part of an Amplify app.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of deployment artifacts.
        pub deployment_artifacts: pulumi_wasm_rust::Output<String>,
        /// Name for the backend environment.
        pub environment_name: pulumi_wasm_rust::Output<String>,
        /// AWS CloudFormation stack name of a backend environment.
        pub stack_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BackendEnvironmentArgs) -> BackendEnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_id_binding = args.app_id.get_inner();
        let deployment_artifacts_binding = args.deployment_artifacts.get_inner();
        let environment_name_binding = args.environment_name.get_inner();
        let stack_name_binding = args.stack_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:amplify/backendEnvironment:BackendEnvironment".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "appId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "deploymentArtifacts".into(),
                },
                register_interface::ResultField {
                    name: "environmentName".into(),
                },
                register_interface::ResultField {
                    name: "stackName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BackendEnvironmentResult {
            app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            deployment_artifacts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentArtifacts").unwrap(),
            ),
            environment_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentName").unwrap(),
            ),
            stack_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackName").unwrap(),
            ),
        }
    }
}