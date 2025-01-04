/// Resource for managing an [AWS Mainframe Modernization Deployment.](https://docs.aws.amazon.com/m2/latest/userguide/applications-m2-deploy.html)
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = deployment::create(
///         "test",
///         DeploymentArgs::builder()
///             .application_id("34567890abcdef012345678012")
///             .application_version(1)
///             .environment_id("01234567890abcdef012345678")
///             .start(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Mainframe Modernization Deployment using the `APPLICATION-ID,DEPLOYMENT-ID`. For example:
///
/// ```sh
/// $ pulumi import aws:m2/deployment:Deployment example APPLICATION-ID,DEPLOYMENT-ID
/// ```
pub mod deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentArgs {
        /// Application to deploy.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// Version to application to deploy
        #[builder(into)]
        pub application_version: pulumi_wasm_rust::Output<i32>,
        /// Environment to deploy application to.
        #[builder(into)]
        pub environment_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub force_stop: pulumi_wasm_rust::Output<Option<bool>>,
        /// Start the application once deployed.
        #[builder(into)]
        pub start: pulumi_wasm_rust::Output<bool>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::m2::DeploymentTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentResult {
        /// Application to deploy.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// Version to application to deploy
        pub application_version: pulumi_wasm_rust::Output<i32>,
        pub deployment_id: pulumi_wasm_rust::Output<String>,
        /// Environment to deploy application to.
        pub environment_id: pulumi_wasm_rust::Output<String>,
        pub force_stop: pulumi_wasm_rust::Output<Option<bool>>,
        /// Start the application once deployed.
        pub start: pulumi_wasm_rust::Output<bool>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::m2::DeploymentTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DeploymentArgs) -> DeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_inner();
        let application_version_binding = args.application_version.get_inner();
        let environment_id_binding = args.environment_id.get_inner();
        let force_stop_binding = args.force_stop.get_inner();
        let start_binding = args.start.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:m2/deployment:Deployment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "applicationVersion".into(),
                    value: &application_version_binding,
                },
                register_interface::ObjectField {
                    name: "environmentId".into(),
                    value: &environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "forceStop".into(),
                    value: &force_stop_binding,
                },
                register_interface::ObjectField {
                    name: "start".into(),
                    value: &start_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "applicationVersion".into(),
                },
                register_interface::ResultField {
                    name: "deploymentId".into(),
                },
                register_interface::ResultField {
                    name: "environmentId".into(),
                },
                register_interface::ResultField {
                    name: "forceStop".into(),
                },
                register_interface::ResultField {
                    name: "start".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DeploymentResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            application_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationVersion").unwrap(),
            ),
            deployment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentId").unwrap(),
            ),
            environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentId").unwrap(),
            ),
            force_stop: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceStop").unwrap(),
            ),
            start: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("start").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
