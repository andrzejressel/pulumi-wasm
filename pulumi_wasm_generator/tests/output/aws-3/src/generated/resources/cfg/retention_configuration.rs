/// Provides a resource to manage the AWS Config retention configuration.
/// The retention configuration defines the number of days that AWS Config stores historical information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = retention_configuration::create(
///         "example",
///         RetentionConfigurationArgs::builder().retention_period_in_days(90).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the AWS Config retention configuration using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/retentionConfiguration:RetentionConfiguration example default
/// ```
pub mod retention_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RetentionConfigurationArgs {
        /// The number of days AWS Config stores historical information.
        #[builder(into)]
        pub retention_period_in_days: pulumi_wasm_rust::Output<i32>,
    }
    #[allow(dead_code)]
    pub struct RetentionConfigurationResult {
        /// The name of the retention configuration object. The object is always named **default**.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The number of days AWS Config stores historical information.
        pub retention_period_in_days: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RetentionConfigurationArgs,
    ) -> RetentionConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let retention_period_in_days_binding = args.retention_period_in_days.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cfg/retentionConfiguration:RetentionConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "retentionPeriodInDays".into(),
                    value: &retention_period_in_days_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "retentionPeriodInDays".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RetentionConfigurationResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            retention_period_in_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionPeriodInDays").unwrap(),
            ),
        }
    }
}
