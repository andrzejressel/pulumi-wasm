/// Manages an AWS End User Messaging SMS phone number.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = smsvoicev_2_phone_number::create(
///         "example",
///         Smsvoicev2PhoneNumberArgs::builder()
///             .iso_country_code("US")
///             .message_type("TRANSACTIONAL")
///             .number_capabilities(vec!["SMS",])
///             .number_type("TOLL_FREE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import phone numbers using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:pinpoint/smsvoicev2PhoneNumber:Smsvoicev2PhoneNumber example phone-abcdef0123456789abcdef0123456789
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod smsvoicev_2_phone_number {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Smsvoicev2PhoneNumberArgs {
        /// By default this is set to `false`. When set to true the phone number can’t be deleted.
        #[builder(into, default)]
        pub deletion_protection_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The two-character code, in ISO 3166-1 alpha-2 format, for the country or region.
        #[builder(into)]
        pub iso_country_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of message. Valid values are `TRANSACTIONAL` for messages that are critical or time-sensitive and `PROMOTIONAL` for messages that aren’t critical or time-sensitive.
        #[builder(into)]
        pub message_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Describes if the origination identity can be used for text messages, voice calls or both. valid values are `SMS` and `VOICE`.
        #[builder(into)]
        pub number_capabilities: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The type of phone number to request. Possible values are `LONG_CODE`, `TOLL_FREE`, `TEN_DLC`, or `SIMULATOR`.
        #[builder(into)]
        pub number_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the opt-out list to associate with the phone number.
        #[builder(into, default)]
        pub opt_out_list_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Use this field to attach your phone number for an external registration process.
        #[builder(into, default)]
        pub registration_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When set to `false` an end recipient sends a message that begins with HELP or STOP to one of your dedicated numbers, AWS End User Messaging SMS and Voice automatically replies with a customizable message and adds the end recipient to the opt-out list. When set to true you’re responsible for responding to HELP and STOP requests. You’re also responsible for tracking and honoring opt-out request.
        #[builder(into, default)]
        pub self_managed_opt_outs_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pinpoint::Smsvoicev2PhoneNumberTimeouts>,
        >,
        /// The Amazon Resource Name (ARN) of the two way channel.
        #[builder(into, default)]
        pub two_way_channel_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// By default this is set to `false`. When set to `true` you can receive incoming text messages from your end recipients.
        #[builder(into, default)]
        pub two_way_channel_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct Smsvoicev2PhoneNumberResult {
        /// ARN of the phone number.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// By default this is set to `false`. When set to true the phone number can’t be deleted.
        pub deletion_protection_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The two-character code, in ISO 3166-1 alpha-2 format, for the country or region.
        pub iso_country_code: pulumi_gestalt_rust::Output<String>,
        /// The type of message. Valid values are `TRANSACTIONAL` for messages that are critical or time-sensitive and `PROMOTIONAL` for messages that aren’t critical or time-sensitive.
        pub message_type: pulumi_gestalt_rust::Output<String>,
        /// The monthly price, in US dollars, to lease the phone number.
        pub monthly_leasing_price: pulumi_gestalt_rust::Output<String>,
        /// Describes if the origination identity can be used for text messages, voice calls or both. valid values are `SMS` and `VOICE`.
        pub number_capabilities: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The type of phone number to request. Possible values are `LONG_CODE`, `TOLL_FREE`, `TEN_DLC`, or `SIMULATOR`.
        pub number_type: pulumi_gestalt_rust::Output<String>,
        /// The name of the opt-out list to associate with the phone number.
        pub opt_out_list_name: pulumi_gestalt_rust::Output<String>,
        /// The new phone number that was requested.
        pub phone_number: pulumi_gestalt_rust::Output<String>,
        /// Use this field to attach your phone number for an external registration process.
        pub registration_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// When set to `false` an end recipient sends a message that begins with HELP or STOP to one of your dedicated numbers, AWS End User Messaging SMS and Voice automatically replies with a customizable message and adds the end recipient to the opt-out list. When set to true you’re responsible for responding to HELP and STOP requests. You’re also responsible for tracking and honoring opt-out request.
        pub self_managed_opt_outs_enabled: pulumi_gestalt_rust::Output<bool>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::pinpoint::Smsvoicev2PhoneNumberTimeouts>,
        >,
        /// The Amazon Resource Name (ARN) of the two way channel.
        pub two_way_channel_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// By default this is set to `false`. When set to `true` you can receive incoming text messages from your end recipients.
        pub two_way_channel_enabled: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: Smsvoicev2PhoneNumberArgs,
    ) -> Smsvoicev2PhoneNumberResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let deletion_protection_enabled_binding = args
            .deletion_protection_enabled
            .get_output(context);
        let iso_country_code_binding = args.iso_country_code.get_output(context);
        let message_type_binding = args.message_type.get_output(context);
        let number_capabilities_binding = args.number_capabilities.get_output(context);
        let number_type_binding = args.number_type.get_output(context);
        let opt_out_list_name_binding = args.opt_out_list_name.get_output(context);
        let registration_id_binding = args.registration_id.get_output(context);
        let self_managed_opt_outs_enabled_binding = args
            .self_managed_opt_outs_enabled
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let two_way_channel_arn_binding = args.two_way_channel_arn.get_output(context);
        let two_way_channel_enabled_binding = args
            .two_way_channel_enabled
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:pinpoint/smsvoicev2PhoneNumber:Smsvoicev2PhoneNumber".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtectionEnabled".into(),
                    value: deletion_protection_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isoCountryCode".into(),
                    value: iso_country_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "messageType".into(),
                    value: message_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "numberCapabilities".into(),
                    value: number_capabilities_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "numberType".into(),
                    value: number_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "optOutListName".into(),
                    value: opt_out_list_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registrationId".into(),
                    value: registration_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selfManagedOptOutsEnabled".into(),
                    value: self_managed_opt_outs_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "twoWayChannelArn".into(),
                    value: two_way_channel_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "twoWayChannelEnabled".into(),
                    value: two_way_channel_enabled_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        Smsvoicev2PhoneNumberResult {
            arn: o.get_field("arn"),
            deletion_protection_enabled: o.get_field("deletionProtectionEnabled"),
            iso_country_code: o.get_field("isoCountryCode"),
            message_type: o.get_field("messageType"),
            monthly_leasing_price: o.get_field("monthlyLeasingPrice"),
            number_capabilities: o.get_field("numberCapabilities"),
            number_type: o.get_field("numberType"),
            opt_out_list_name: o.get_field("optOutListName"),
            phone_number: o.get_field("phoneNumber"),
            registration_id: o.get_field("registrationId"),
            self_managed_opt_outs_enabled: o.get_field("selfManagedOptOutsEnabled"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
            two_way_channel_arn: o.get_field("twoWayChannelArn"),
            two_way_channel_enabled: o.get_field("twoWayChannelEnabled"),
        }
    }
}
