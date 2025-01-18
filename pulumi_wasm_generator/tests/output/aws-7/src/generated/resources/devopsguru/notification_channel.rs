/// Resource for managing an AWS DevOps Guru Notification Channel.
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
///     let example = notification_channel::create(
///         "example",
///         NotificationChannelArgs::builder()
///             .sns(
///                 NotificationChannelSns::builder()
///                     .topicArn("${exampleAwsSnsTopic.arn}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Filters
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = notification_channel::create(
///         "example",
///         NotificationChannelArgs::builder()
///             .filters(
///                 NotificationChannelFilters::builder()
///                     .messageTypes(vec!["NEW_INSIGHT",])
///                     .severities(vec!["HIGH",])
///                     .build_struct(),
///             )
///             .sns(
///                 NotificationChannelSns::builder()
///                     .topicArn("${exampleAwsSnsTopic.arn}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DevOps Guru Notification Channel using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:devopsguru/notificationChannel:NotificationChannel example id-12345678
/// ```
pub mod notification_channel {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotificationChannelArgs {
        /// Filter configurations for the Amazon SNS notification topic. See the `filters` argument reference below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<super::super::types::devopsguru::NotificationChannelFilters>,
        >,
        /// SNS noficiation channel configurations. See the `sns` argument reference below.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub sns: pulumi_wasm_rust::Output<
            Option<super::super::types::devopsguru::NotificationChannelSns>,
        >,
    }
    #[allow(dead_code)]
    pub struct NotificationChannelResult {
        /// Filter configurations for the Amazon SNS notification topic. See the `filters` argument reference below.
        pub filters: pulumi_wasm_rust::Output<
            Option<super::super::types::devopsguru::NotificationChannelFilters>,
        >,
        /// SNS noficiation channel configurations. See the `sns` argument reference below.
        ///
        /// The following arguments are optional:
        pub sns: pulumi_wasm_rust::Output<
            Option<super::super::types::devopsguru::NotificationChannelSns>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NotificationChannelArgs,
    ) -> NotificationChannelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let sns_binding = args.sns.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:devopsguru/notificationChannel:NotificationChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
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
                    name: "sns".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NotificationChannelResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            sns: pulumi_wasm_rust::__private::into_domain(hashmap.remove("sns").unwrap()),
        }
    }
}
