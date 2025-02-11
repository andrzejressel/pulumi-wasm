/// Provides an API Gateway Usage Plan.
///
/// ## Import
///
/// Using `pulumi import`, import AWS API Gateway Usage Plan using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/usagePlan:UsagePlan myusageplan <usage_plan_id>
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod usage_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UsagePlanArgs {
        /// Associated API stages of the usage plan.
        #[builder(into, default)]
        pub api_stages: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::apigateway::UsagePlanApiStage>>,
        >,
        /// Description of a usage plan.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the usage plan.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// AWS Marketplace product identifier to associate with the usage plan as a SaaS product on AWS Marketplace.
        #[builder(into, default)]
        pub product_code: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Quota of the usage plan.
        #[builder(into, default)]
        pub quota_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigateway::UsagePlanQuotaSettings>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Throttling limits of the usage plan.
        #[builder(into, default)]
        pub throttle_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigateway::UsagePlanThrottleSettings>,
        >,
    }
    #[allow(dead_code)]
    pub struct UsagePlanResult {
        /// Associated API stages of the usage plan.
        pub api_stages: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::apigateway::UsagePlanApiStage>>,
        >,
        /// ARN
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of a usage plan.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the usage plan.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// AWS Marketplace product identifier to associate with the usage plan as a SaaS product on AWS Marketplace.
        pub product_code: pulumi_gestalt_rust::Output<Option<String>>,
        /// Quota of the usage plan.
        pub quota_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigateway::UsagePlanQuotaSettings>,
        >,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Throttling limits of the usage plan.
        pub throttle_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigateway::UsagePlanThrottleSettings>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UsagePlanArgs,
    ) -> UsagePlanResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_stages_binding = args.api_stages.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let product_code_binding = args.product_code.get_output(context);
        let quota_settings_binding = args.quota_settings.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let throttle_settings_binding = args.throttle_settings.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/usagePlan:UsagePlan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiStages".into(),
                    value: &api_stages_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "productCode".into(),
                    value: &product_code_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "quotaSettings".into(),
                    value: &quota_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "throttleSettings".into(),
                    value: &throttle_settings_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        UsagePlanResult {
            api_stages: o.get_field("apiStages"),
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            product_code: o.get_field("productCode"),
            quota_settings: o.get_field("quotaSettings"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            throttle_settings: o.get_field("throttleSettings"),
        }
    }
}
