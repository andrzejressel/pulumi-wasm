pub mod get_notification_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNotificationChannelArgs {
        /// Filter configurations for the Amazon SNS notification topic. See the `filters` attribute reference below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::devopsguru::GetNotificationChannelFilter>,
            >,
        >,
        /// Unique identifier for the notification channel.
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// SNS noficiation channel configurations. See the `sns` attribute reference below.
        #[builder(into, default)]
        pub sns: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::devopsguru::GetNotificationChannelSn>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetNotificationChannelResult {
        /// Filter configurations for the Amazon SNS notification topic. See the `filters` attribute reference below.
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::devopsguru::GetNotificationChannelFilter>,
            >,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// SNS noficiation channel configurations. See the `sns` attribute reference below.
        pub sns: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::devopsguru::GetNotificationChannelSn>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetNotificationChannelArgs,
    ) -> GetNotificationChannelResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let sns_binding = args.sns.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:devopsguru/getNotificationChannel:getNotificationChannel".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "sns".into(),
                    value: &sns_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNotificationChannelResult {
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            sns: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sns")),
        }
    }
}
