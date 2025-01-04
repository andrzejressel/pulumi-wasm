/// Provides a resource to manage whether serial console access is enabled for your AWS account in the current AWS region.
///
/// > **NOTE:** Removing this resource disables serial console access.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = serial_console_access::create(
///         "example",
///         SerialConsoleAccessArgs::builder().enabled(true).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import serial console access state. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/serialConsoleAccess:SerialConsoleAccess example default
/// ```
pub mod serial_console_access {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SerialConsoleAccessArgs {
        /// Whether or not serial console access is enabled. Valid values are `true` or `false`. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SerialConsoleAccessResult {
        /// Whether or not serial console access is enabled. Valid values are `true` or `false`. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SerialConsoleAccessArgs,
    ) -> SerialConsoleAccessResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/serialConsoleAccess:SerialConsoleAccess".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SerialConsoleAccessResult {
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
        }
    }
}
