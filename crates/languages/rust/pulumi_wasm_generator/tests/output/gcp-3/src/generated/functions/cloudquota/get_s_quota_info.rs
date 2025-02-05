pub mod get_s_quota_info {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSQuotaInfoArgs {
        /// The parent of the quota info. Allowed parents are "projects/[project-id / number]" or "folders/[folder-id / number]" or "organizations/[org-id / number].
        #[builder(into)]
        pub parent: pulumi_wasm_rust::InputOrOutput<String>,
        /// The id of the quota, which is unique within the service.
        #[builder(into)]
        pub quota_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the service in which the quota is defined.
        #[builder(into)]
        pub service: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSQuotaInfoResult {
        /// (Output) The container type of the QuotaInfo.
        pub container_type: pulumi_wasm_rust::Output<String>,
        /// The map of dimensions for this dimensions info. The key of a map entry is "region", "zone" or the name of a service specific dimension, and the value of a map entry is the value of the dimension. If a dimension does not appear in the map of dimensions, the dimensions info applies to all the dimension values except for those that have another DimenisonInfo instance configured for the specific value. Example: {"provider" : "Foo Inc"} where "provider" is a service specific dimension of a quota.
        pub dimensions: pulumi_wasm_rust::Output<Vec<String>>,
        /// (Output) The collection of dimensions info ordered by their dimensions from more specific ones to less specific ones.
        pub dimensions_infos: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudquota::GetSQuotaInfoDimensionsInfo>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// (Output) Whether the quota is a concurrent quota. Concurrent quotas are enforced on the total number of concurrent operations in flight at any given time.
        pub is_concurrent: pulumi_wasm_rust::Output<bool>,
        /// (Output) Whether the quota value is fixed or adjustable.
        pub is_fixed: pulumi_wasm_rust::Output<bool>,
        /// (Output) Whether this is a precise quota. A precise quota is tracked with absolute precision. In contrast, an imprecise quota is not tracked with precision.
        pub is_precise: pulumi_wasm_rust::Output<bool>,
        /// (Output) The metric of the quota. It specifies the resources consumption the quota is defined for, for example: `compute.googleapis.com/cpus`.
        pub metric: pulumi_wasm_rust::Output<String>,
        /// (Output) The display name of the quota metric.
        pub metric_display_name: pulumi_wasm_rust::Output<String>,
        /// (Output) The unit in which the metric value is reported, e.g., `MByte`.
        pub metric_unit: pulumi_wasm_rust::Output<String>,
        /// (Output) Resource name of this QuotaInfo, for example: `projects/123/locations/global/services/compute.googleapis.com/quotaInfos/CpusPerProjectPerRegion`.
        pub name: pulumi_wasm_rust::Output<String>,
        pub parent: pulumi_wasm_rust::Output<String>,
        /// (Output) The display name of the quota.
        pub quota_display_name: pulumi_wasm_rust::Output<String>,
        pub quota_id: pulumi_wasm_rust::Output<String>,
        /// (Output) Whether it is eligible to request a higher quota value for this quota.
        pub quota_increase_eligibilities: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudquota::GetSQuotaInfoQuotaIncreaseEligibility,
            >,
        >,
        /// (Output) The reset time interval for the quota. Refresh interval applies to rate quota only. Example: "minute" for per minute, "day" for per day, or "10 seconds" for every 10 seconds.
        pub refresh_interval: pulumi_wasm_rust::Output<String>,
        pub service: pulumi_wasm_rust::Output<String>,
        /// (Output) URI to the page where users can request more quota for the cloud service, for example: `https://console.cloud.google.com/iam-admin/quotas`.
        pub service_request_quota_uri: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSQuotaInfoArgs,
    ) -> GetSQuotaInfoResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parent_binding = args.parent.get_output(context).get_inner();
        let quota_id_binding = args.quota_id.get_output(context).get_inner();
        let service_binding = args.service.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudquota/getSQuotaInfo:getSQuotaInfo".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "quotaId".into(),
                    value: &quota_id_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSQuotaInfoResult {
            container_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("containerType"),
            ),
            dimensions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dimensions"),
            ),
            dimensions_infos: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dimensionsInfos"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            is_concurrent: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isConcurrent"),
            ),
            is_fixed: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isFixed"),
            ),
            is_precise: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isPrecise"),
            ),
            metric: pulumi_wasm_rust::__private::into_domain(o.extract_field("metric")),
            metric_display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metricDisplayName"),
            ),
            metric_unit: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metricUnit"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_wasm_rust::__private::into_domain(o.extract_field("parent")),
            quota_display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("quotaDisplayName"),
            ),
            quota_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("quotaId"),
            ),
            quota_increase_eligibilities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("quotaIncreaseEligibilities"),
            ),
            refresh_interval: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("refreshInterval"),
            ),
            service: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("service"),
            ),
            service_request_quota_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceRequestQuotaUri"),
            ),
        }
    }
}
