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
pub mod suppression {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SuppressionArgs {
        /// The Name which should be used for this Advisor suppression. Changing this forces a new Advisor suppression to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Advisor recommendation to suppress. Changing this forces a new Advisor suppression to be created.
        #[builder(into)]
        pub recommendation_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Resource to suppress the Advisor recommendation for. Changing this forces a new Advisor suppression to be created.
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// A optional time to live value. If omitted, the suppression will not expire. Changing this forces a new Advisor suppression to be created.
        #[builder(into, default)]
        pub ttl: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SuppressionResult {
        /// The Name which should be used for this Advisor suppression. Changing this forces a new Advisor suppression to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Advisor recommendation to suppress. Changing this forces a new Advisor suppression to be created.
        pub recommendation_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Resource to suppress the Advisor recommendation for. Changing this forces a new Advisor suppression to be created.
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// The GUID of the suppression.
        pub suppression_id: pulumi_wasm_rust::Output<String>,
        /// A optional time to live value. If omitted, the suppression will not expire. Changing this forces a new Advisor suppression to be created.
        pub ttl: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SuppressionArgs) -> SuppressionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let recommendation_id_binding = args.recommendation_id.get_inner();
        let resource_id_binding = args.resource_id.get_inner();
        let ttl_binding = args.ttl.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:advisor/suppression:Suppression".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "recommendationId".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "suppressionId".into(),
                },
                register_interface::ResultField {
                    name: "ttl".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SuppressionResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            recommendation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recommendationId").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            suppression_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("suppressionId").unwrap(),
            ),
            ttl: pulumi_wasm_rust::__private::into_domain(hashmap.remove("ttl").unwrap()),
        }
    }
}