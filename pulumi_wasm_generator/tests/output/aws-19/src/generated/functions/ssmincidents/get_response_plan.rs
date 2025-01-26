pub mod get_response_plan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResponsePlanArgs {
        /// The Amazon Resource Name (ARN) of the response plan.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The tags applied to the response plan.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetResponsePlanResult {
        /// (Optional) The actions that the response plan starts at the beginning of an incident.
        pub actions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ssmincidents::GetResponsePlanAction>,
        >,
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Chatbot chat channel used for collaboration during an incident.
        pub chat_channels: pulumi_wasm_rust::Output<Vec<String>>,
        /// The long format of the response plan name. This field can contain spaces.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the contacts and escalation plans that the response plan engages during an incident.
        pub engagements: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub incident_templates: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::ssmincidents::GetResponsePlanIncidentTemplate,
            >,
        >,
        /// Information about third-party services integrated into the response plan. The following values are supported:
        pub integrations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ssmincidents::GetResponsePlanIntegration>,
        >,
        /// The name of the PagerDuty configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The tags applied to the response plan.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetResponsePlanArgs,
    ) -> GetResponsePlanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssmincidents/getResponsePlan:getResponsePlan".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "actions".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "chatChannels".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "engagements".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "incidentTemplates".into(),
                },
                register_interface::ResultField {
                    name: "integrations".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResponsePlanResult {
            actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actions").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            chat_channels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("chatChannels").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            engagements: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engagements").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            incident_templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("incidentTemplates").unwrap(),
            ),
            integrations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrations").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
