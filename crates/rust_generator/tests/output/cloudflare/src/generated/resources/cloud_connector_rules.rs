/// The Cloud Connector Rules resource allows you to create and manage cloud connector rules for a zone.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: cloudflare:CloudConnectorRules
///     properties:
///       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
///       rules:
///         - description: connect aws bucket
///           enabled: true
///           expression: http.uri
///           provider: aws_s3
///           parameters:
///             - host: mystorage.s3.ams.amazonaws.com
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cloud_connector_rules {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CloudConnectorRulesArgs {
        /// List of Cloud Connector Rules
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::CloudConnectorRulesRule>>,
        >,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CloudConnectorRulesResult {
        /// List of Cloud Connector Rules
        pub rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::CloudConnectorRulesRule>>,
        >,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CloudConnectorRulesArgs,
    ) -> CloudConnectorRulesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let rules_binding = args.rules.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/cloudConnectorRules:CloudConnectorRules".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CloudConnectorRulesResult {
            rules: o.get_field("rules"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
