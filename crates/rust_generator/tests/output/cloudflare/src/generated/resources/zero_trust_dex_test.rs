/// Provides a Cloudflare Device Dex Test resource. Device Dex Tests allow for building location-aware device settings policies.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zero_trust_dex_test::create(
///         "example",
///         ZeroTrustDexTestArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .data(
///                 ZeroTrustDexTestData::builder()
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
/// $ pulumi import cloudflare:index/zeroTrustDexTest:ZeroTrustDexTest example <account_id>/<device_dex_test_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zero_trust_dex_test {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustDexTestArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The configuration object which contains the details for the WARP client to conduct the test.
        #[builder(into)]
        pub data: pulumi_gestalt_rust::InputOrOutput<super::types::ZeroTrustDexTestData>,
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
    pub struct ZeroTrustDexTestResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Timestamp of when the Dex Test was created.
        pub created: pulumi_gestalt_rust::Output<String>,
        /// The configuration object which contains the details for the WARP client to conduct the test.
        pub data: pulumi_gestalt_rust::Output<super::types::ZeroTrustDexTestData>,
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZeroTrustDexTestArgs,
    ) -> ZeroTrustDexTestResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let data_binding = args.data.get_output(context);
        let description_binding = args.description.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let interval_binding = args.interval.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustDexTest:ZeroTrustDexTest".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "data".into(),
                    value: data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interval".into(),
                    value: interval_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZeroTrustDexTestResult {
            account_id: o.get_field("accountId"),
            created: o.get_field("created"),
            data: o.get_field("data"),
            description: o.get_field("description"),
            enabled: o.get_field("enabled"),
            interval: o.get_field("interval"),
            name: o.get_field("name"),
            updated: o.get_field("updated"),
        }
    }
}
