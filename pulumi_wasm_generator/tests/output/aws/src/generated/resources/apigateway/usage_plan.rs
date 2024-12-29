/// Provides an API Gateway Usage Plan.
///
/// ## Import
///
/// Using `pulumi import`, import AWS API Gateway Usage Plan using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/usagePlan:UsagePlan myusageplan <usage_plan_id>
/// ```
pub mod usage_plan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UsagePlanArgs {
        /// Associated API stages of the usage plan.
        #[builder(into, default)]
        pub api_stages: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apigateway::UsagePlanApiStage>>,
        >,
        /// Description of a usage plan.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the usage plan.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// AWS Marketplace product identifier to associate with the usage plan as a SaaS product on AWS Marketplace.
        #[builder(into, default)]
        pub product_code: pulumi_wasm_rust::Output<Option<String>>,
        /// Quota of the usage plan.
        #[builder(into, default)]
        pub quota_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::apigateway::UsagePlanQuotaSettings>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Throttling limits of the usage plan.
        #[builder(into, default)]
        pub throttle_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::apigateway::UsagePlanThrottleSettings>,
        >,
    }
    #[allow(dead_code)]
    pub struct UsagePlanResult {
        /// Associated API stages of the usage plan.
        pub api_stages: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apigateway::UsagePlanApiStage>>,
        >,
        /// ARN
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of a usage plan.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the usage plan.
        pub name: pulumi_wasm_rust::Output<String>,
        /// AWS Marketplace product identifier to associate with the usage plan as a SaaS product on AWS Marketplace.
        pub product_code: pulumi_wasm_rust::Output<Option<String>>,
        /// Quota of the usage plan.
        pub quota_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::apigateway::UsagePlanQuotaSettings>,
        >,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Throttling limits of the usage plan.
        pub throttle_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::apigateway::UsagePlanThrottleSettings>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: UsagePlanArgs) -> UsagePlanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_stages_binding = args.api_stages.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let product_code_binding = args.product_code.get_inner();
        let quota_settings_binding = args.quota_settings.get_inner();
        let tags_binding = args.tags.get_inner();
        let throttle_settings_binding = args.throttle_settings.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/usagePlan:UsagePlan".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiStages".into(),
                    value: &api_stages_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "productCode".into(),
                    value: &product_code_binding,
                },
                register_interface::ObjectField {
                    name: "quotaSettings".into(),
                    value: &quota_settings_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "throttleSettings".into(),
                    value: &throttle_settings_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiStages".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "productCode".into(),
                },
                register_interface::ResultField {
                    name: "quotaSettings".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "throttleSettings".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UsagePlanResult {
            api_stages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiStages").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            product_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productCode").unwrap(),
            ),
            quota_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quotaSettings").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            throttle_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("throttleSettings").unwrap(),
            ),
        }
    }
}
