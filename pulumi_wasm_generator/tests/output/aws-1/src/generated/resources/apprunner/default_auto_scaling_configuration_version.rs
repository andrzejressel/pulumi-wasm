/// Manages the default App Runner auto scaling configuration.
/// When creating or updating this resource the existing default auto scaling configuration will be set to non-default automatically.
/// When creating or updating this resource the configuration is automatically assigned as the default to the new services you create in the future. The new default designation doesn't affect the associations that were previously set for existing services.
/// Each account can have only one default auto scaling configuration per Region.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = auto_scaling_configuration_version::create(
///         "example",
///         AutoScalingConfigurationVersionArgs::builder()
///             .auto_scaling_configuration_name("example")
///             .max_concurrency(50)
///             .max_size(10)
///             .min_size(2)
///             .build_struct(),
///     );
///     let exampleDefaultAutoScalingConfigurationVersion = default_auto_scaling_configuration_version::create(
///         "exampleDefaultAutoScalingConfigurationVersion",
///         DefaultAutoScalingConfigurationVersionArgs::builder()
///             .auto_scaling_configuration_arn("${example.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import App Runner default auto scaling configurations using the current Region. For example:
///
/// ```sh
/// $ pulumi import aws:apprunner/defaultAutoScalingConfigurationVersion:DefaultAutoScalingConfigurationVersion example us-west-2
/// ```
pub mod default_auto_scaling_configuration_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultAutoScalingConfigurationVersionArgs {
        /// The ARN of the App Runner auto scaling configuration that you want to set as the default.
        #[builder(into)]
        pub auto_scaling_configuration_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DefaultAutoScalingConfigurationVersionResult {
        /// The ARN of the App Runner auto scaling configuration that you want to set as the default.
        pub auto_scaling_configuration_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DefaultAutoScalingConfigurationVersionArgs,
    ) -> DefaultAutoScalingConfigurationVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_scaling_configuration_arn_binding = args
            .auto_scaling_configuration_arn
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apprunner/defaultAutoScalingConfigurationVersion:DefaultAutoScalingConfigurationVersion"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoScalingConfigurationArn".into(),
                    value: &auto_scaling_configuration_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoScalingConfigurationArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DefaultAutoScalingConfigurationVersionResult {
            auto_scaling_configuration_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoScalingConfigurationArn").unwrap(),
            ),
        }
    }
}
