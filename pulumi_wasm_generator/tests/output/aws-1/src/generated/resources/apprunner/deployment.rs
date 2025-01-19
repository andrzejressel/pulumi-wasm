/// Manages an App Runner Deployment Operation.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = deployment::create(
///         "example",
///         DeploymentArgs::builder()
///             .service_arn("${exampleAwsApprunnerService.arn}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentArgs {
        /// The Amazon Resource Name (ARN) of the App Runner service to start the deployment for.
        #[builder(into)]
        pub service_arn: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::apprunner::DeploymentTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentResult {
        /// The unique ID of the operation associated with deployment.
        pub operation_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the App Runner service to start the deployment for.
        pub service_arn: pulumi_wasm_rust::Output<String>,
        /// The current status of the App Runner service deployment.
        pub status: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::apprunner::DeploymentTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DeploymentArgs) -> DeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let service_arn_binding = args.service_arn.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apprunner/deployment:Deployment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "serviceArn".into(),
                    value: &service_arn_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "operationId".into(),
                },
                register_interface::ResultField {
                    name: "serviceArn".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
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
            operation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("operationId").unwrap(),
            ),
            service_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceArn").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
