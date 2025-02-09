/// Use the `aws.pinpoint.SmsChannel` resource to manage Pinpoint SMS Channels.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let app = app::create("app", AppArgs::builder().build_struct());
///     let sms = sms_channel::create(
///         "sms",
///         SmsChannelArgs::builder().application_id("${app.applicationId}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the Pinpoint SMS Channel using the `application_id`. For example:
///
/// ```sh
/// $ pulumi import aws:pinpoint/smsChannel:SmsChannel sms application-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sms_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SmsChannelArgs {
        /// ID of the application.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the channel is enabled or disabled. By default, it is set to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Identifier of the sender for your messages.
        #[builder(into, default)]
        pub sender_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Short Code registered with the phone provider.
        #[builder(into, default)]
        pub short_code: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SmsChannelResult {
        /// ID of the application.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// Whether the channel is enabled or disabled. By default, it is set to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Maximum number of promotional messages that can be sent per second.
        pub promotional_messages_per_second: pulumi_gestalt_rust::Output<i32>,
        /// Identifier of the sender for your messages.
        pub sender_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Short Code registered with the phone provider.
        pub short_code: pulumi_gestalt_rust::Output<Option<String>>,
        /// Maximum number of transactional messages per second that can be sent.
        pub transactional_messages_per_second: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SmsChannelArgs,
    ) -> SmsChannelResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_id_binding_1 = args.application_id.get_output(context);
        let application_id_binding = application_id_binding_1.get_inner();
        let enabled_binding_1 = args.enabled.get_output(context);
        let enabled_binding = enabled_binding_1.get_inner();
        let sender_id_binding_1 = args.sender_id.get_output(context);
        let sender_id_binding = sender_id_binding_1.get_inner();
        let short_code_binding_1 = args.short_code.get_output(context);
        let short_code_binding = short_code_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:pinpoint/smsChannel:SmsChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "senderId".into(),
                    value: &sender_id_binding,
                },
                register_interface::ObjectField {
                    name: "shortCode".into(),
                    value: &short_code_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SmsChannelResult {
            application_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationId"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            promotional_messages_per_second: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("promotionalMessagesPerSecond"),
            ),
            sender_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("senderId"),
            ),
            short_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shortCode"),
            ),
            transactional_messages_per_second: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transactionalMessagesPerSecond"),
            ),
        }
    }
}
