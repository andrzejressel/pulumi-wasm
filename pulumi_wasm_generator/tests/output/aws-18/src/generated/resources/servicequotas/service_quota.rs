/// Manages an individual Service Quota.
///
/// > **NOTE:** Global quotas apply to all AWS regions, but can only be accessed in `us-east-1` in the Commercial partition or `us-gov-west-1` in the GovCloud partition. In other regions, the AWS API will return the error `The request failed because the specified service does not exist.`
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = service_quota::create(
///         "example",
///         ServiceQuotaArgs::builder()
///             .quota_code("L-F678F1CE")
///             .service_code("vpc")
///             .value(75)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicequotas_service_quota` using the service code and quota code, separated by a front slash (`/`). For example:
///
/// ~> __NOTE:__ This resource does not require explicit import and will assume management of an existing service quota on Pulumi resource creation.
///
/// ```sh
/// $ pulumi import aws:servicequotas/serviceQuota:ServiceQuota example vpc/L-F678F1CE
/// ```
pub mod service_quota {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceQuotaArgs {
        /// Code of the service quota to track. For example: `L-F678F1CE`. Available values can be found with the [AWS CLI service-quotas list-service-quotas command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-service-quotas.html).
        #[builder(into)]
        pub quota_code: pulumi_wasm_rust::InputOrOutput<String>,
        /// Code of the service to track. For example: `vpc`. Available values can be found with the [AWS CLI service-quotas list-services command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-services.html).
        #[builder(into)]
        pub service_code: pulumi_wasm_rust::InputOrOutput<String>,
        /// Float specifying the desired value for the service quota. If the desired value is higher than the current value, a quota increase request is submitted. When a known request is submitted and pending, the value reflects the desired value of the pending request.
        #[builder(into)]
        pub value: pulumi_wasm_rust::InputOrOutput<f64>,
    }
    #[allow(dead_code)]
    pub struct ServiceQuotaResult {
        /// Whether the service quota can be increased.
        pub adjustable: pulumi_wasm_rust::Output<bool>,
        /// Amazon Resource Name (ARN) of the service quota.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Default value of the service quota.
        pub default_value: pulumi_wasm_rust::Output<f64>,
        /// Code of the service quota to track. For example: `L-F678F1CE`. Available values can be found with the [AWS CLI service-quotas list-service-quotas command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-service-quotas.html).
        pub quota_code: pulumi_wasm_rust::Output<String>,
        /// Name of the quota.
        pub quota_name: pulumi_wasm_rust::Output<String>,
        pub request_id: pulumi_wasm_rust::Output<String>,
        pub request_status: pulumi_wasm_rust::Output<String>,
        /// Code of the service to track. For example: `vpc`. Available values can be found with the [AWS CLI service-quotas list-services command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-services.html).
        pub service_code: pulumi_wasm_rust::Output<String>,
        /// Name of the service.
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// Information about the measurement.
        pub usage_metrics: pulumi_wasm_rust::Output<
            Vec<super::super::types::servicequotas::ServiceQuotaUsageMetric>,
        >,
        /// Float specifying the desired value for the service quota. If the desired value is higher than the current value, a quota increase request is submitted. When a known request is submitted and pending, the value reflects the desired value of the pending request.
        pub value: pulumi_wasm_rust::Output<f64>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServiceQuotaArgs,
    ) -> ServiceQuotaResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let quota_code_binding = args.quota_code.get_output(context).get_inner();
        let service_code_binding = args.service_code.get_output(context).get_inner();
        let value_binding = args.value.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicequotas/serviceQuota:ServiceQuota".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "quotaCode".into(),
                    value: &quota_code_binding,
                },
                register_interface::ObjectField {
                    name: "serviceCode".into(),
                    value: &service_code_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceQuotaResult {
            adjustable: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adjustable"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            default_value: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultValue"),
            ),
            quota_code: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("quotaCode"),
            ),
            quota_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("quotaName"),
            ),
            request_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requestId"),
            ),
            request_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requestStatus"),
            ),
            service_code: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceCode"),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceName"),
            ),
            usage_metrics: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("usageMetrics"),
            ),
            value: pulumi_wasm_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
