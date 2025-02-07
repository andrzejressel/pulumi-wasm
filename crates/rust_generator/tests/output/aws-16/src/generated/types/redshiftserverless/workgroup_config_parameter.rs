#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkgroupConfigParameter {
    /// The key of the parameter. The options are `auto_mv`, `datestyle`, `enable_case_sensitive_identifier`, `enable_user_activity_logging`, `query_group`, `search_path`, `require_ssl`, `use_fips_ssl`, and [query monitoring metrics](https://docs.aws.amazon.com/redshift/latest/dg/cm-c-wlm-query-monitoring-rules.html#cm-c-wlm-query-monitoring-metrics-serverless) that let you define performance boundaries: `max_query_cpu_time`, `max_query_blocks_read`, `max_scan_row_count`, `max_query_execution_time`, `max_query_queue_time`, `max_query_cpu_usage_percent`, `max_query_temp_blocks_to_disk`, `max_join_row_count` and `max_nested_loop_join_row_count`.
    #[builder(into)]
    #[serde(rename = "parameterKey")]
    pub r#parameter_key: Box<String>,
    /// The value of the parameter to set.
    #[builder(into)]
    #[serde(rename = "parameterValue")]
    pub r#parameter_value: Box<String>,
}
