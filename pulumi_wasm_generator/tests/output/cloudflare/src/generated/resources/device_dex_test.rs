/// Provides a Cloudflare Device Dex Test resource. Device Dex Tests allow for building location-aware device settings policies.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = device_dex_test::create(
///         "example",
///         DeviceDexTestArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .data(
///                 DeviceDexTestData::builder()
///                     .host("https://example.com/home")
///                     .kind("http")
///                     .method("GET")
///                     .build_struct(),
///             )
///             .description("Send a HTTP GET request to the home endpoint every half hour.")
///             .enabled(true)
///             .interval("0h30m0s")
///             .name("GET homepage")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/deviceDexTest:DeviceDexTest example <account_id>/<device_dex_test_id>
/// ```
///
pub mod device_dex_test {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeviceDexTestArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The configuration object which contains the details for the WARP client to conduct the test.
        #[builder(into)]
        pub data: pulumi_wasm_rust::Output<super::types::DeviceDexTestData>,
        /// Additional details about the test.
        #[builder(into)]
        pub description: pulumi_wasm_rust::Output<String>,
        /// Determines whether or not the test is active.
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// How often the test will run.
        #[builder(into)]
        pub interval: pulumi_wasm_rust::Output<String>,
        /// The name of the Device Dex Test. Must be unique.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DeviceDexTestResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Timestamp of when the Dex Test was created.
        pub created: pulumi_wasm_rust::Output<String>,
        /// The configuration object which contains the details for the WARP client to conduct the test.
        pub data: pulumi_wasm_rust::Output<super::types::DeviceDexTestData>,
        /// Additional details about the test.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Determines whether or not the test is active.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// How often the test will run.
        pub interval: pulumi_wasm_rust::Output<String>,
        /// The name of the Device Dex Test. Must be unique.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Timestamp of when the Dex Test was last updated.
        pub updated: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DeviceDexTestArgs) -> DeviceDexTestResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let data_binding = args.data.get_inner();
        let description_binding = args.description.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let interval_binding = args.interval.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/deviceDexTest:DeviceDexTest".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "data".into(),
                    value: &data_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "interval".into(),
                    value: &interval_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "created".into(),
                },
                register_interface::ResultField {
                    name: "data".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "interval".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "updated".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DeviceDexTestResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("created").unwrap(),
            ),
            data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("data").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            interval: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interval").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            updated: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updated").unwrap(),
            ),
        }
    }
}
