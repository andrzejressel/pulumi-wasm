/// This setting defines how a user interacts with or uses a service or a feature of a service.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testSetting = service_setting::create(
///         "testSetting",
///         ServiceSettingArgs::builder()
///             .setting_id(
///                 "arn:aws:ssm:us-east-1:123456789012:servicesetting/ssm/parameter-store/high-throughput-enabled",
///             )
///             .setting_value("true")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS SSM Service Setting using the `setting_id`. For example:
///
/// ```sh
/// $ pulumi import aws:ssm/serviceSetting:ServiceSetting example arn:aws:ssm:us-east-1:123456789012:servicesetting/ssm/parameter-store/high-throughput-enabled
/// ```
pub mod service_setting {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceSettingArgs {
        /// ID of the service setting.
        #[builder(into)]
        pub setting_id: pulumi_wasm_rust::Output<String>,
        /// Value of the service setting.
        #[builder(into)]
        pub setting_value: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceSettingResult {
        /// ARN of the service setting.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ID of the service setting.
        pub setting_id: pulumi_wasm_rust::Output<String>,
        /// Value of the service setting.
        pub setting_value: pulumi_wasm_rust::Output<String>,
        /// Status of the service setting. Value can be `Default`, `Customized` or `PendingUpdate`.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceSettingArgs) -> ServiceSettingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let setting_id_binding = args.setting_id.get_inner();
        let setting_value_binding = args.setting_value.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssm/serviceSetting:ServiceSetting".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "settingId".into(),
                    value: &setting_id_binding,
                },
                register_interface::ObjectField {
                    name: "settingValue".into(),
                    value: &setting_value_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "settingId".into(),
                },
                register_interface::ResultField {
                    name: "settingValue".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceSettingResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            setting_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("settingId").unwrap(),
            ),
            setting_value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("settingValue").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
