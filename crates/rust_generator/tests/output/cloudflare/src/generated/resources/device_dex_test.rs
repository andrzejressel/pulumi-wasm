/// Provides a Cloudflare Device Dex Test resource. Device Dex Tests allow for building location-aware device settings policies.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation)]
pub mod device_dex_test {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeviceDexTestArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The configuration object which contains the details for the WARP client to conduct the test.
        #[builder(into)]
        pub data: pulumi_gestalt_rust::InputOrOutput<super::types::DeviceDexTestData>,
        /// Additional details about the test.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Determines whether or not the test is active.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// How often the test will run.
        #[builder(into)]
        pub interval: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Device Dex Test. Must be unique.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DeviceDexTestResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Timestamp of when the Dex Test was created.
        pub created: pulumi_gestalt_rust::Output<String>,
        /// The configuration object which contains the details for the WARP client to conduct the test.
        pub data: pulumi_gestalt_rust::Output<super::types::DeviceDexTestData>,
        /// Additional details about the test.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Determines whether or not the test is active.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// How often the test will run.
        pub interval: pulumi_gestalt_rust::Output<String>,
        /// The name of the Device Dex Test. Must be unique.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Timestamp of when the Dex Test was last updated.
        pub updated: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DeviceDexTestArgs,
    ) -> DeviceDexTestResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let data_binding = args.data.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let interval_binding = args.interval.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        DeviceDexTestResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            created: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("created"),
            ),
            data: pulumi_gestalt_rust::__private::into_domain(o.extract_field("data")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            interval: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("interval"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            updated: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updated"),
            ),
        }
    }
}
