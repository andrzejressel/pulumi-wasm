#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service_quota {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceQuotaArgs {
        /// Quota code within the service. When configured, the data source directly looks up the service quota. Available values can be found with the [AWS CLI service-quotas list-service-quotas command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-service-quotas.html). One of `quota_code` or `quota_name` must be specified.
        #[builder(into, default)]
        pub quota_code: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Quota name within the service. When configured, the data source searches through all service quotas to find the matching quota name. Available values can be found with the [AWS CLI service-quotas list-service-quotas command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-service-quotas.html). One of `quota_name` or `quota_code` must be specified.
        #[builder(into, default)]
        pub quota_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Service code for the quota. Available values can be found with the `aws.servicequotas.getService` data source or [AWS CLI service-quotas list-services command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-services.html).
        #[builder(into)]
        pub service_code: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceQuotaResult {
        /// Whether the service quota is adjustable.
        pub adjustable: pulumi_gestalt_rust::Output<bool>,
        /// ARN of the service quota.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Default value of the service quota.
        pub default_value: pulumi_gestalt_rust::Output<f64>,
        /// Whether the service quota is global for the AWS account.
        pub global_quota: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub quota_code: pulumi_gestalt_rust::Output<String>,
        pub quota_name: pulumi_gestalt_rust::Output<String>,
        pub service_code: pulumi_gestalt_rust::Output<String>,
        /// Name of the service.
        pub service_name: pulumi_gestalt_rust::Output<String>,
        /// Information about the measurement.
        pub usage_metrics: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::servicequotas::GetServiceQuotaUsageMetric>,
        >,
        /// Current value of the service quota.
        pub value: pulumi_gestalt_rust::Output<f64>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServiceQuotaArgs,
    ) -> GetServiceQuotaResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let quota_code_binding = args.quota_code.get_output(context);
        let quota_name_binding = args.quota_name.get_output(context);
        let service_code_binding = args.service_code.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:servicequotas/getServiceQuota:getServiceQuota".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "quotaCode".into(),
                    value: quota_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "quotaName".into(),
                    value: quota_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceCode".into(),
                    value: service_code_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServiceQuotaResult {
            adjustable: o.get_field("adjustable"),
            arn: o.get_field("arn"),
            default_value: o.get_field("defaultValue"),
            global_quota: o.get_field("globalQuota"),
            id: o.get_field("id"),
            quota_code: o.get_field("quotaCode"),
            quota_name: o.get_field("quotaName"),
            service_code: o.get_field("serviceCode"),
            service_name: o.get_field("serviceName"),
            usage_metrics: o.get_field("usageMetrics"),
            value: o.get_field("value"),
        }
    }
}
