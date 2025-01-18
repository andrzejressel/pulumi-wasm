pub mod get_notification_channel {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNotificationChannelArgs {
        /// Filter configurations for the Amazon SNS notification topic. See the `filters` attribute reference below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::devopsguru::GetNotificationChannelFilter>,
            >,
        >,
        /// Unique identifier for the notification channel.
        #[builder(into)]
        pub id: pulumi_wasm_rust::Output<String>,
        /// SNS noficiation channel configurations. See the `sns` attribute reference below.
        #[builder(into, default)]
        pub sns: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::devopsguru::GetNotificationChannelSn>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetNotificationChannelResult {
        /// Filter configurations for the Amazon SNS notification topic. See the `filters` attribute reference below.
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::devopsguru::GetNotificationChannelFilter>,
            >,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
        /// SNS noficiation channel configurations. See the `sns` attribute reference below.
        pub sns: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::devopsguru::GetNotificationChannelSn>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetNotificationChannelArgs) -> GetNotificationChannelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let id_binding = args.id.get_inner();
        let sns_binding = args.sns.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "sns".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNotificationChannelResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            sns: pulumi_wasm_rust::__private::into_domain(hashmap.remove("sns").unwrap()),
        }
    }
}
