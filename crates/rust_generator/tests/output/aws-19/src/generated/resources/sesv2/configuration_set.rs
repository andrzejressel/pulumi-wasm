/// Resource for managing an AWS SESv2 (Simple Email V2) Configuration Set.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod configuration_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationSetArgs {
        /// The name of the configuration set.
        #[builder(into)]
        pub configuration_set_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An object that defines the dedicated IP pool that is used to send emails that you send using the configuration set. See `delivery_options` Block for details.
        #[builder(into, default)]
        pub delivery_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sesv2::ConfigurationSetDeliveryOptions>,
        >,
        /// An object that defines whether or not Amazon SES collects reputation metrics for the emails that you send that use the configuration set. See `reputation_options` Block for details.
        #[builder(into, default)]
        pub reputation_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sesv2::ConfigurationSetReputationOptions>,
        >,
        /// An object that defines whether or not Amazon SES can send email that you send using the configuration set. See `sending_options` Block for details.
        #[builder(into, default)]
        pub sending_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sesv2::ConfigurationSetSendingOptions>,
        >,
        /// An object that contains information about the suppression list preferences for your account. See `suppression_options` Block for details.
        #[builder(into, default)]
        pub suppression_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sesv2::ConfigurationSetSuppressionOptions>,
        >,
        /// A map of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An object that defines the open and click tracking options for emails that you send using the configuration set. See `tracking_options` Block for details.
        #[builder(into, default)]
        pub tracking_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sesv2::ConfigurationSetTrackingOptions>,
        >,
        /// An object that defines the VDM settings that apply to emails that you send using the configuration set. See `vdm_options` Block for details.
        #[builder(into, default)]
        pub vdm_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sesv2::ConfigurationSetVdmOptions>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigurationSetResult {
        /// ARN of the Configuration Set.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the configuration set.
        pub configuration_set_name: pulumi_gestalt_rust::Output<String>,
        /// An object that defines the dedicated IP pool that is used to send emails that you send using the configuration set. See `delivery_options` Block for details.
        pub delivery_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::sesv2::ConfigurationSetDeliveryOptions>,
        >,
        /// An object that defines whether or not Amazon SES collects reputation metrics for the emails that you send that use the configuration set. See `reputation_options` Block for details.
        pub reputation_options: pulumi_gestalt_rust::Output<
            super::super::types::sesv2::ConfigurationSetReputationOptions,
        >,
        /// An object that defines whether or not Amazon SES can send email that you send using the configuration set. See `sending_options` Block for details.
        pub sending_options: pulumi_gestalt_rust::Output<
            super::super::types::sesv2::ConfigurationSetSendingOptions,
        >,
        /// An object that contains information about the suppression list preferences for your account. See `suppression_options` Block for details.
        pub suppression_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::sesv2::ConfigurationSetSuppressionOptions>,
        >,
        /// A map of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// An object that defines the open and click tracking options for emails that you send using the configuration set. See `tracking_options` Block for details.
        pub tracking_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::sesv2::ConfigurationSetTrackingOptions>,
        >,
        /// An object that defines the VDM settings that apply to emails that you send using the configuration set. See `vdm_options` Block for details.
        pub vdm_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::sesv2::ConfigurationSetVdmOptions>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigurationSetArgs,
    ) -> ConfigurationSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_set_name_binding = args
            .configuration_set_name
            .get_output(context);
        let delivery_options_binding = args.delivery_options.get_output(context);
        let reputation_options_binding = args.reputation_options.get_output(context);
        let sending_options_binding = args.sending_options.get_output(context);
        let suppression_options_binding = args.suppression_options.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tracking_options_binding = args.tracking_options.get_output(context);
        let vdm_options_binding = args.vdm_options.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sesv2/configurationSet:ConfigurationSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationSetName".into(),
                    value: configuration_set_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deliveryOptions".into(),
                    value: delivery_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reputationOptions".into(),
                    value: reputation_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sendingOptions".into(),
                    value: sending_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "suppressionOptions".into(),
                    value: suppression_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trackingOptions".into(),
                    value: tracking_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vdmOptions".into(),
                    value: vdm_options_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConfigurationSetResult {
            arn: o.get_field("arn"),
            configuration_set_name: o.get_field("configurationSetName"),
            delivery_options: o.get_field("deliveryOptions"),
            reputation_options: o.get_field("reputationOptions"),
            sending_options: o.get_field("sendingOptions"),
            suppression_options: o.get_field("suppressionOptions"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tracking_options: o.get_field("trackingOptions"),
            vdm_options: o.get_field("vdmOptions"),
        }
    }
}
