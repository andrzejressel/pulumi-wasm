/// Specifies a suppression for an Azure Advisor recommendation.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleSuppression:
///     type: azure:advisor:Suppression
///     name: example
///     properties:
///       name: HardcodedSuppressionName
///       recommendationId: ${test.recommendations[0].recommendationName}
///       resourceId: /subscriptions/${current.subscriptionId}
///       ttl: 01:00:00:00
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   example:
///     fn::invoke:
///       function: azure:advisor:getRecommendations
///       arguments: {}
/// ```
///
/// ## Import
///
/// Advisor suppressions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:advisor/suppression:Suppression example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Advisor/recommendations/00000000-0000-0000-0000-000000000000/suppressions/name
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod suppression {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SuppressionArgs {
        /// The Name which should be used for this Advisor suppression. Changing this forces a new Advisor suppression to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Advisor recommendation to suppress. Changing this forces a new Advisor suppression to be created.
        #[builder(into)]
        pub recommendation_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Resource to suppress the Advisor recommendation for. Changing this forces a new Advisor suppression to be created.
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A optional time to live value. If omitted, the suppression will not expire. Changing this forces a new Advisor suppression to be created.
        #[builder(into, default)]
        pub ttl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SuppressionResult {
        /// The Name which should be used for this Advisor suppression. Changing this forces a new Advisor suppression to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Advisor recommendation to suppress. Changing this forces a new Advisor suppression to be created.
        pub recommendation_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Resource to suppress the Advisor recommendation for. Changing this forces a new Advisor suppression to be created.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// The GUID of the suppression.
        pub suppression_id: pulumi_gestalt_rust::Output<String>,
        /// A optional time to live value. If omitted, the suppression will not expire. Changing this forces a new Advisor suppression to be created.
        pub ttl: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SuppressionArgs,
    ) -> SuppressionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let recommendation_id_binding = args.recommendation_id.get_output(context);
        let resource_id_binding = args.resource_id.get_output(context);
        let ttl_binding = args.ttl.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:advisor/suppression:Suppression".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recommendationId".into(),
                    value: recommendation_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceId".into(),
                    value: resource_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ttl".into(),
                    value: ttl_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SuppressionResult {
            name: o.get_field("name"),
            recommendation_id: o.get_field("recommendationId"),
            resource_id: o.get_field("resourceId"),
            suppression_id: o.get_field("suppressionId"),
            ttl: o.get_field("ttl"),
        }
    }
}
