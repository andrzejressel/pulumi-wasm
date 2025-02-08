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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SuppressionArgs,
    ) -> SuppressionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let recommendation_id_binding = args
            .recommendation_id
            .get_output(context)
            .get_inner();
        let resource_id_binding = args.resource_id.get_output(context).get_inner();
        let ttl_binding = args.ttl.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:advisor/suppression:Suppression".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recommendationId".into(),
                    value: &recommendation_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "ttl".into(),
                    value: &ttl_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SuppressionResult {
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            recommendation_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recommendationId"),
            ),
            resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
            suppression_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("suppressionId"),
            ),
            ttl: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ttl")),
        }
    }
}
