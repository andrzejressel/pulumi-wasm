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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CloudConnectorRulesArgs,
    ) -> CloudConnectorRulesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let rules_binding = args.rules.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/cloudConnectorRules:CloudConnectorRules".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CloudConnectorRulesResult {
            rules: pulumi_gestalt_rust::__private::into_domain(o.extract_field("rules")),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
