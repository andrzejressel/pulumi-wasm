#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_traffic_policy_document {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTrafficPolicyDocumentArgs {
        /// Configuration block for the definitions of the endpoints that you want to use in this traffic policy. See below
        #[builder(into, default)]
        pub endpoints: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::route53::GetTrafficPolicyDocumentEndpoint,
                >,
            >,
        >,
        /// DNS type of all of the resource record sets that Amazon Route 53 will create based on this traffic policy.
        #[builder(into, default)]
        pub record_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for definitions of the rules that you want to use in this traffic policy. See below
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::route53::GetTrafficPolicyDocumentRule>,
            >,
        >,
        /// An endpoint to be as the starting point for the traffic policy.
        #[builder(into, default)]
        pub start_endpoint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A rule to be as the starting point for the traffic policy.
        #[builder(into, default)]
        pub start_rule: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Version of the traffic policy format.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetTrafficPolicyDocumentResult {
        pub endpoints: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::route53::GetTrafficPolicyDocumentEndpoint,
                >,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Standard JSON policy document rendered based on the arguments above.
        pub json: pulumi_gestalt_rust::Output<String>,
        pub record_type: pulumi_gestalt_rust::Output<Option<String>>,
        pub rules: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::route53::GetTrafficPolicyDocumentRule>,
            >,
        >,
        pub start_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
        pub start_rule: pulumi_gestalt_rust::Output<Option<String>>,
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTrafficPolicyDocumentArgs,
    ) -> GetTrafficPolicyDocumentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let endpoints_binding = args.endpoints.get_output(context);
        let record_type_binding = args.record_type.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let start_endpoint_binding = args.start_endpoint.get_output(context);
        let start_rule_binding = args.start_rule.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:route53/getTrafficPolicyDocument:getTrafficPolicyDocument"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpoints".into(),
                    value: endpoints_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recordType".into(),
                    value: record_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startEndpoint".into(),
                    value: start_endpoint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startRule".into(),
                    value: start_rule_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTrafficPolicyDocumentResult {
            endpoints: o.get_field("endpoints"),
            id: o.get_field("id"),
            json: o.get_field("json"),
            record_type: o.get_field("recordType"),
            rules: o.get_field("rules"),
            start_endpoint: o.get_field("startEndpoint"),
            start_rule: o.get_field("startRule"),
            version: o.get_field("version"),
        }
    }
}
