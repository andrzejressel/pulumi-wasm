pub mod get_service_quota {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceQuotaArgs {
        /// Quota code within the service. When configured, the data source directly looks up the service quota. Available values can be found with the [AWS CLI service-quotas list-service-quotas command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-service-quotas.html). One of `quota_code` or `quota_name` must be specified.
        #[builder(into, default)]
        pub quota_code: pulumi_wasm_rust::Output<Option<String>>,
        /// Quota name within the service. When configured, the data source searches through all service quotas to find the matching quota name. Available values can be found with the [AWS CLI service-quotas list-service-quotas command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-service-quotas.html). One of `quota_name` or `quota_code` must be specified.
        #[builder(into, default)]
        pub quota_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Service code for the quota. Available values can be found with the `aws.servicequotas.getService` data source or [AWS CLI service-quotas list-services command](https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-services.html).
        #[builder(into)]
        pub service_code: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceQuotaResult {
        /// Whether the service quota is adjustable.
        pub adjustable: pulumi_wasm_rust::Output<bool>,
        /// ARN of the service quota.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Default value of the service quota.
        pub default_value: pulumi_wasm_rust::Output<f64>,
        /// Whether the service quota is global for the AWS account.
        pub global_quota: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub quota_code: pulumi_wasm_rust::Output<String>,
        pub quota_name: pulumi_wasm_rust::Output<String>,
        pub service_code: pulumi_wasm_rust::Output<String>,
        /// Name of the service.
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// Information about the measurement.
        pub usage_metrics: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::servicequotas::GetServiceQuotaUsageMetric>,
        >,
        /// Current value of the service quota.
        pub value: pulumi_wasm_rust::Output<f64>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetServiceQuotaArgs) -> GetServiceQuotaResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let quota_code_binding = args.quota_code.get_inner();
        let quota_name_binding = args.quota_name.get_inner();
        let service_code_binding = args.service_code.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:servicequotas/getServiceQuota:getServiceQuota".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "quotaCode".into(),
                    value: &quota_code_binding,
                },
                register_interface::ObjectField {
                    name: "quotaName".into(),
                    value: &quota_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceCode".into(),
                    value: &service_code_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "adjustable".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "defaultValue".into(),
                },
                register_interface::ResultField {
                    name: "globalQuota".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "quotaCode".into(),
                },
                register_interface::ResultField {
                    name: "quotaName".into(),
                },
                register_interface::ResultField {
                    name: "serviceCode".into(),
                },
                register_interface::ResultField {
                    name: "serviceName".into(),
                },
                register_interface::ResultField {
                    name: "usageMetrics".into(),
                },
                register_interface::ResultField {
                    name: "value".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServiceQuotaResult {
            adjustable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adjustable").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            default_value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultValue").unwrap(),
            ),
            global_quota: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalQuota").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            quota_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quotaCode").unwrap(),
            ),
            quota_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quotaName").unwrap(),
            ),
            service_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceCode").unwrap(),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceName").unwrap(),
            ),
            usage_metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("usageMetrics").unwrap(),
            ),
            value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("value").unwrap(),
            ),
        }
    }
}
