#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_s_quota_info {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSQuotaInfoArgs {
        /// The parent of the quota info. Allowed parents are "projects/[project-id / number]" or "folders/[folder-id / number]" or "organizations/[org-id / number].
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the quota, which is unique within the service.
        #[builder(into)]
        pub quota_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the service in which the quota is defined.
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSQuotaInfoResult {
        /// (Output) The container type of the QuotaInfo.
        pub container_type: pulumi_gestalt_rust::Output<String>,
        /// The map of dimensions for this dimensions info. The key of a map entry is "region", "zone" or the name of a service specific dimension, and the value of a map entry is the value of the dimension. If a dimension does not appear in the map of dimensions, the dimensions info applies to all the dimension values except for those that have another DimenisonInfo instance configured for the specific value. Example: {"provider" : "Foo Inc"} where "provider" is a service specific dimension of a quota.
        pub dimensions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// (Output) The collection of dimensions info ordered by their dimensions from more specific ones to less specific ones.
        pub dimensions_infos: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudquota::GetSQuotaInfoDimensionsInfo>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// (Output) Whether the quota is a concurrent quota. Concurrent quotas are enforced on the total number of concurrent operations in flight at any given time.
        pub is_concurrent: pulumi_gestalt_rust::Output<bool>,
        /// (Output) Whether the quota value is fixed or adjustable.
        pub is_fixed: pulumi_gestalt_rust::Output<bool>,
        /// (Output) Whether this is a precise quota. A precise quota is tracked with absolute precision. In contrast, an imprecise quota is not tracked with precision.
        pub is_precise: pulumi_gestalt_rust::Output<bool>,
        /// (Output) The metric of the quota. It specifies the resources consumption the quota is defined for, for example: `compute.googleapis.com/cpus`.
        pub metric: pulumi_gestalt_rust::Output<String>,
        /// (Output) The display name of the quota metric.
        pub metric_display_name: pulumi_gestalt_rust::Output<String>,
        /// (Output) The unit in which the metric value is reported, e.g., `MByte`.
        pub metric_unit: pulumi_gestalt_rust::Output<String>,
        /// (Output) Resource name of this QuotaInfo, for example: `projects/123/locations/global/services/compute.googleapis.com/quotaInfos/CpusPerProjectPerRegion`.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// (Output) The display name of the quota.
        pub quota_display_name: pulumi_gestalt_rust::Output<String>,
        pub quota_id: pulumi_gestalt_rust::Output<String>,
        /// (Output) Whether it is eligible to request a higher quota value for this quota.
        pub quota_increase_eligibilities: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::cloudquota::GetSQuotaInfoQuotaIncreaseEligibility,
            >,
        >,
        /// (Output) The reset time interval for the quota. Refresh interval applies to rate quota only. Example: "minute" for per minute, "day" for per day, or "10 seconds" for every 10 seconds.
        pub refresh_interval: pulumi_gestalt_rust::Output<String>,
        pub service: pulumi_gestalt_rust::Output<String>,
        /// (Output) URI to the page where users can request more quota for the cloud service, for example: `https://console.cloud.google.com/iam-admin/quotas`.
        pub service_request_quota_uri: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSQuotaInfoArgs,
    ) -> GetSQuotaInfoResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let parent_binding = args.parent.get_output(context);
        let quota_id_binding = args.quota_id.get_output(context);
        let service_binding = args.service.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:cloudquota/getSQuotaInfo:getSQuotaInfo".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "quotaId".into(),
                    value: &quota_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "service".into(),
                    value: &service_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSQuotaInfoResult {
            container_type: o.get_field("containerType"),
            dimensions: o.get_field("dimensions"),
            dimensions_infos: o.get_field("dimensionsInfos"),
            id: o.get_field("id"),
            is_concurrent: o.get_field("isConcurrent"),
            is_fixed: o.get_field("isFixed"),
            is_precise: o.get_field("isPrecise"),
            metric: o.get_field("metric"),
            metric_display_name: o.get_field("metricDisplayName"),
            metric_unit: o.get_field("metricUnit"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            quota_display_name: o.get_field("quotaDisplayName"),
            quota_id: o.get_field("quotaId"),
            quota_increase_eligibilities: o.get_field("quotaIncreaseEligibilities"),
            refresh_interval: o.get_field("refreshInterval"),
            service: o.get_field("service"),
            service_request_quota_uri: o.get_field("serviceRequestQuotaUri"),
        }
    }
}
