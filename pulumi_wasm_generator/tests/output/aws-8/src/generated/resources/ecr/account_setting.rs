/// Provides a resource to manage AWS ECR Basic Scan Type
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = account_setting::create(
///         "foo",
///         AccountSettingArgs::builder()
///             .name("BASIC_SCAN_TYPE_VERSION")
///             .value("CLAIR")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EMR Security Configurations using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:ecr/accountSetting:AccountSetting foo BASIC_SCAN_TYPE_VERSION
/// ```
pub mod account_setting {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountSettingArgs {
        /// The name of the ECR Scan Type. This should be `BASIC_SCAN_TYPE_VERSION`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The value of the ECR Scan Type. This can be `AWS_NATIVE` or `CLAIR`.
        #[builder(into)]
        pub value: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AccountSettingResult {
        /// The name of the ECR Scan Type. This should be `BASIC_SCAN_TYPE_VERSION`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The value of the ECR Scan Type. This can be `AWS_NATIVE` or `CLAIR`.
        pub value: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccountSettingArgs) -> AccountSettingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let value_binding = args.value.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecr/accountSetting:AccountSetting".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "value".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountSettingResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("value").unwrap(),
            ),
        }
    }
}
