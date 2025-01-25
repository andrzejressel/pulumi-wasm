/// Resource for managing an AWS SESv2 (Simple Email V2) Configuration Set.
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
///     let example = configuration_set::create(
///         "example",
///         ConfigurationSetArgs::builder()
///             .configuration_set_name("example")
///             .delivery_options(
///                 ConfigurationSetDeliveryOptions::builder()
///                     .maxDeliverySeconds(300)
///                     .tlsPolicy("REQUIRE")
///                     .build_struct(),
///             )
///             .reputation_options(
///                 ConfigurationSetReputationOptions::builder()
///                     .reputationMetricsEnabled(false)
///                     .build_struct(),
///             )
///             .sending_options(
///                 ConfigurationSetSendingOptions::builder()
///                     .sendingEnabled(true)
///                     .build_struct(),
///             )
///             .suppression_options(
///                 ConfigurationSetSuppressionOptions::builder()
///                     .suppressedReasons(vec!["BOUNCE", "COMPLAINT",])
///                     .build_struct(),
///             )
///             .tracking_options(
///                 ConfigurationSetTrackingOptions::builder()
///                     .customRedirectDomain("example.com")
///                     .httpsPolicy("REQUIRE")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SESv2 (Simple Email V2) Configuration Set using the `configuration_set_name`. For example:
///
/// ```sh
/// $ pulumi import aws:sesv2/configurationSet:ConfigurationSet example example
/// ```
pub mod configuration_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationSetArgs {
        /// The name of the configuration set.
        #[builder(into)]
        pub configuration_set_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// An object that defines the dedicated IP pool that is used to send emails that you send using the configuration set. See `delivery_options` Block for details.
        #[builder(into, default)]
        pub delivery_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::sesv2::ConfigurationSetDeliveryOptions>,
        >,
        /// An object that defines whether or not Amazon SES collects reputation metrics for the emails that you send that use the configuration set. See `reputation_options` Block for details.
        #[builder(into, default)]
        pub reputation_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::sesv2::ConfigurationSetReputationOptions>,
        >,
        /// An object that defines whether or not Amazon SES can send email that you send using the configuration set. See `sending_options` Block for details.
        #[builder(into, default)]
        pub sending_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::sesv2::ConfigurationSetSendingOptions>,
        >,
        /// An object that contains information about the suppression list preferences for your account. See `suppression_options` Block for details.
        #[builder(into, default)]
        pub suppression_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::sesv2::ConfigurationSetSuppressionOptions>,
        >,
        /// A map of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An object that defines the open and click tracking options for emails that you send using the configuration set. See `tracking_options` Block for details.
        #[builder(into, default)]
        pub tracking_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::sesv2::ConfigurationSetTrackingOptions>,
        >,
        /// An object that defines the VDM settings that apply to emails that you send using the configuration set. See `vdm_options` Block for details.
        #[builder(into, default)]
        pub vdm_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::sesv2::ConfigurationSetVdmOptions>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigurationSetResult {
        /// ARN of the Configuration Set.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the configuration set.
        pub configuration_set_name: pulumi_wasm_rust::Output<String>,
        /// An object that defines the dedicated IP pool that is used to send emails that you send using the configuration set. See `delivery_options` Block for details.
        pub delivery_options: pulumi_wasm_rust::Output<
            Option<super::super::types::sesv2::ConfigurationSetDeliveryOptions>,
        >,
        /// An object that defines whether or not Amazon SES collects reputation metrics for the emails that you send that use the configuration set. See `reputation_options` Block for details.
        pub reputation_options: pulumi_wasm_rust::Output<
            super::super::types::sesv2::ConfigurationSetReputationOptions,
        >,
        /// An object that defines whether or not Amazon SES can send email that you send using the configuration set. See `sending_options` Block for details.
        pub sending_options: pulumi_wasm_rust::Output<
            super::super::types::sesv2::ConfigurationSetSendingOptions,
        >,
        /// An object that contains information about the suppression list preferences for your account. See `suppression_options` Block for details.
        pub suppression_options: pulumi_wasm_rust::Output<
            Option<super::super::types::sesv2::ConfigurationSetSuppressionOptions>,
        >,
        /// A map of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// An object that defines the open and click tracking options for emails that you send using the configuration set. See `tracking_options` Block for details.
        pub tracking_options: pulumi_wasm_rust::Output<
            Option<super::super::types::sesv2::ConfigurationSetTrackingOptions>,
        >,
        /// An object that defines the VDM settings that apply to emails that you send using the configuration set. See `vdm_options` Block for details.
        pub vdm_options: pulumi_wasm_rust::Output<
            Option<super::super::types::sesv2::ConfigurationSetVdmOptions>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ConfigurationSetArgs,
    ) -> ConfigurationSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_set_name_binding = args
            .configuration_set_name
            .get_output(context)
            .get_inner();
        let delivery_options_binding = args
            .delivery_options
            .get_output(context)
            .get_inner();
        let reputation_options_binding = args
            .reputation_options
            .get_output(context)
            .get_inner();
        let sending_options_binding = args
            .sending_options
            .get_output(context)
            .get_inner();
        let suppression_options_binding = args
            .suppression_options
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let tracking_options_binding = args
            .tracking_options
            .get_output(context)
            .get_inner();
        let vdm_options_binding = args.vdm_options.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sesv2/configurationSet:ConfigurationSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurationSetName".into(),
                    value: &configuration_set_name_binding,
                },
                register_interface::ObjectField {
                    name: "deliveryOptions".into(),
                    value: &delivery_options_binding,
                },
                register_interface::ObjectField {
                    name: "reputationOptions".into(),
                    value: &reputation_options_binding,
                },
                register_interface::ObjectField {
                    name: "sendingOptions".into(),
                    value: &sending_options_binding,
                },
                register_interface::ObjectField {
                    name: "suppressionOptions".into(),
                    value: &suppression_options_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "trackingOptions".into(),
                    value: &tracking_options_binding,
                },
                register_interface::ObjectField {
                    name: "vdmOptions".into(),
                    value: &vdm_options_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "configurationSetName".into(),
                },
                register_interface::ResultField {
                    name: "deliveryOptions".into(),
                },
                register_interface::ResultField {
                    name: "reputationOptions".into(),
                },
                register_interface::ResultField {
                    name: "sendingOptions".into(),
                },
                register_interface::ResultField {
                    name: "suppressionOptions".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "trackingOptions".into(),
                },
                register_interface::ResultField {
                    name: "vdmOptions".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConfigurationSetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            configuration_set_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationSetName").unwrap(),
            ),
            delivery_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deliveryOptions").unwrap(),
            ),
            reputation_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reputationOptions").unwrap(),
            ),
            sending_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sendingOptions").unwrap(),
            ),
            suppression_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("suppressionOptions").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            tracking_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trackingOptions").unwrap(),
            ),
            vdm_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vdmOptions").unwrap(),
            ),
        }
    }
}
