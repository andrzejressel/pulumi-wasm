/// Resource for managing an AWS DevOps Guru Notification Channel.
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod notification_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotificationChannelArgs {
        /// Filter configurations for the Amazon SNS notification topic. See the `filters` argument reference below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::devopsguru::NotificationChannelFilters>,
        >,
        /// SNS noficiation channel configurations. See the `sns` argument reference below.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub sns: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::devopsguru::NotificationChannelSns>,
        >,
    }
    #[allow(dead_code)]
    pub struct NotificationChannelResult {
        /// Filter configurations for the Amazon SNS notification topic. See the `filters` argument reference below.
        pub filters: pulumi_gestalt_rust::Output<
            Option<super::super::types::devopsguru::NotificationChannelFilters>,
        >,
        /// SNS noficiation channel configurations. See the `sns` argument reference below.
        ///
        /// The following arguments are optional:
        pub sns: pulumi_gestalt_rust::Output<
            Option<super::super::types::devopsguru::NotificationChannelSns>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NotificationChannelArgs,
    ) -> NotificationChannelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let sns_binding = args.sns.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:devopsguru/notificationChannel:NotificationChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sns".into(),
                    value: sns_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NotificationChannelResult {
            filters: o.get_field("filters"),
            sns: o.get_field("sns"),
        }
    }
}
