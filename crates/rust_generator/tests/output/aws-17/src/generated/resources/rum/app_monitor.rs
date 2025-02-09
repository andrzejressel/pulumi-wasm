/// Provides a CloudWatch RUM App Monitor resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = app_monitor::create(
///         "example",
///         AppMonitorArgs::builder().domain("localhost").name("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cloudwatch RUM App Monitor using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:rum/appMonitor:AppMonitor example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod app_monitor {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppMonitorArgs {
        /// configuration data for the app monitor. See app_monitor_configuration below.
        #[builder(into, default)]
        pub app_monitor_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rum::AppMonitorAppMonitorConfiguration>,
        >,
        /// Specifies whether this app monitor allows the web client to define and send custom events. If you omit this parameter, custom events are `DISABLED`. See custom_events below.
        #[builder(into, default)]
        pub custom_events: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rum::AppMonitorCustomEvents>,
        >,
        /// Data collected by RUM is kept by RUM for 30 days and then deleted. This parameter  specifies whether RUM sends a copy of this telemetry data to Amazon CloudWatch Logs in your account. This enables you to keep the telemetry data for more than 30 days, but it does incur Amazon CloudWatch Logs charges. Default value is `false`.
        #[builder(into, default)]
        pub cw_log_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The top-level internet domain name for which your application has administrative authority.
        #[builder(into)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the log stream.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AppMonitorResult {
        /// configuration data for the app monitor. See app_monitor_configuration below.
        pub app_monitor_configuration: pulumi_gestalt_rust::Output<
            super::super::types::rum::AppMonitorAppMonitorConfiguration,
        >,
        /// The unique ID of the app monitor. Useful for JS templates.
        pub app_monitor_id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) specifying the app monitor.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether this app monitor allows the web client to define and send custom events. If you omit this parameter, custom events are `DISABLED`. See custom_events below.
        pub custom_events: pulumi_gestalt_rust::Output<
            super::super::types::rum::AppMonitorCustomEvents,
        >,
        /// Data collected by RUM is kept by RUM for 30 days and then deleted. This parameter  specifies whether RUM sends a copy of this telemetry data to Amazon CloudWatch Logs in your account. This enables you to keep the telemetry data for more than 30 days, but it does incur Amazon CloudWatch Logs charges. Default value is `false`.
        pub cw_log_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the log group where the copies are stored.
        pub cw_log_group: pulumi_gestalt_rust::Output<String>,
        /// The top-level internet domain name for which your application has administrative authority.
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// The name of the log stream.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AppMonitorArgs,
    ) -> AppMonitorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_monitor_configuration_binding = args
            .app_monitor_configuration
            .get_output(context);
        let custom_events_binding = args.custom_events.get_output(context);
        let cw_log_enabled_binding = args.cw_log_enabled.get_output(context);
        let domain_binding = args.domain.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rum/appMonitor:AppMonitor".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appMonitorConfiguration".into(),
                    value: app_monitor_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customEvents".into(),
                    value: custom_events_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cwLogEnabled".into(),
                    value: cw_log_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AppMonitorResult {
            app_monitor_configuration: o.get_field("appMonitorConfiguration"),
            app_monitor_id: o.get_field("appMonitorId"),
            arn: o.get_field("arn"),
            custom_events: o.get_field("customEvents"),
            cw_log_enabled: o.get_field("cwLogEnabled"),
            cw_log_group: o.get_field("cwLogGroup"),
            domain: o.get_field("domain"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
