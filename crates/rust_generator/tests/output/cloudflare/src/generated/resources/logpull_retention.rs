/// Allows management of the Logpull Retention settings used to control whether or not to retain HTTP request logs.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: cloudflare:LogpullRetention
///     properties:
///       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
///       enabled: 'true'
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/logpullRetention:LogpullRetention example <zone_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod logpull_retention {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogpullRetentionArgs {
        /// Whether you wish to retain logs or not.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LogpullRetentionResult {
        /// Whether you wish to retain logs or not.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogpullRetentionArgs,
    ) -> LogpullRetentionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enabled_binding = args.enabled.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/logpullRetention:LogpullRetention".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LogpullRetentionResult {
            enabled: o.get_field("enabled"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
