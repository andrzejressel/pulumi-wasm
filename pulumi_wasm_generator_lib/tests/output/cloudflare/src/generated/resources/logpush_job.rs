#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct LogpushJobArgs {
    /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The kind of the dataset to use with the logpush job. Available values: `access_requests`, `casb_findings`, `firewall_events`, `http_requests`, `spectrum_events`, `nel_reports`, `audit_logs`, `gateway_dns`, `gateway_http`, `gateway_network`, `dns_logs`, `network_analytics_logs`, `workers_trace_events`, `device_posture_results`, `zero_trust_network_sessions`, `magic_ids_detections`, `page_shield_events`.
    #[builder(into)]
    pub dataset: pulumi_wasm_rust::Output<String>,
    /// Uniquely identifies a resource (such as an s3 bucket) where data will be pushed. Additional configuration parameters supported by the destination may be included. See [Logpush destination documentation](https://developers.cloudflare.com/logs/reference/logpush-api-configuration#destination).
    #[builder(into)]
    pub destination_conf: pulumi_wasm_rust::Output<String>,
    /// Whether to enable the job.
    #[builder(into, default)]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Use filters to select the events to include and/or remove from your logs. For more information, refer to [Filters](https://developers.cloudflare.com/logs/reference/logpush-api-configuration/filters/).
    #[builder(into, default)]
    pub filter: pulumi_wasm_rust::Output<Option<String>>,
    /// A higher frequency will result in logs being pushed on faster with smaller files. `low` frequency will push logs less often with larger files. Available values: `high`, `low`. Defaults to `high`.
    #[builder(into, default)]
    pub frequency: pulumi_wasm_rust::Output<Option<String>>,
    /// The kind of logpush job to create. Available values: `edge`, `instant-logs`, `""`.
    #[builder(into, default)]
    pub kind: pulumi_wasm_rust::Output<Option<String>>,
    /// Configuration string for the Logshare API. It specifies things like requested fields and timestamp formats. See [Logpush options documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#options).
    #[builder(into, default)]
    pub logpull_options: pulumi_wasm_rust::Output<Option<String>>,
    /// The maximum uncompressed file size of a batch of logs. Value must be between 5MB and 1GB.
    #[builder(into, default)]
    pub max_upload_bytes: pulumi_wasm_rust::Output<Option<i32>>,
    /// The maximum interval in seconds for log batches. Value must be between 30 and 300.
    #[builder(into, default)]
    pub max_upload_interval_seconds: pulumi_wasm_rust::Output<Option<i32>>,
    /// The maximum number of log lines per batch. Value must be between 1000 and 1,000,000.
    #[builder(into, default)]
    pub max_upload_records: pulumi_wasm_rust::Output<Option<i32>>,
    /// The name of the logpush job to create.
    #[builder(into, default)]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// Structured replacement for logpull*options. When including this field, the logpull*option field will be ignored.
    #[builder(into, default)]
    pub output_options: pulumi_wasm_rust::Output<
        Option<super::types::LogpushJobOutputOptions>,
    >,
    /// Ownership challenge token to prove destination ownership, required when destination is Amazon S3, Google Cloud Storage, Microsoft Azure or Sumo Logic. See [Developer documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#usage).
    #[builder(into, default)]
    pub ownership_challenge: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct LogpushJobResult {
    /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The kind of the dataset to use with the logpush job. Available values: `access_requests`, `casb_findings`, `firewall_events`, `http_requests`, `spectrum_events`, `nel_reports`, `audit_logs`, `gateway_dns`, `gateway_http`, `gateway_network`, `dns_logs`, `network_analytics_logs`, `workers_trace_events`, `device_posture_results`, `zero_trust_network_sessions`, `magic_ids_detections`, `page_shield_events`.
    pub dataset: pulumi_wasm_rust::Output<String>,
    /// Uniquely identifies a resource (such as an s3 bucket) where data will be pushed. Additional configuration parameters supported by the destination may be included. See [Logpush destination documentation](https://developers.cloudflare.com/logs/reference/logpush-api-configuration#destination).
    pub destination_conf: pulumi_wasm_rust::Output<String>,
    /// Whether to enable the job.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Use filters to select the events to include and/or remove from your logs. For more information, refer to [Filters](https://developers.cloudflare.com/logs/reference/logpush-api-configuration/filters/).
    pub filter: pulumi_wasm_rust::Output<Option<String>>,
    /// A higher frequency will result in logs being pushed on faster with smaller files. `low` frequency will push logs less often with larger files. Available values: `high`, `low`. Defaults to `high`.
    pub frequency: pulumi_wasm_rust::Output<Option<String>>,
    /// The kind of logpush job to create. Available values: `edge`, `instant-logs`, `""`.
    pub kind: pulumi_wasm_rust::Output<Option<String>>,
    /// Configuration string for the Logshare API. It specifies things like requested fields and timestamp formats. See [Logpush options documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#options).
    pub logpull_options: pulumi_wasm_rust::Output<Option<String>>,
    /// The maximum uncompressed file size of a batch of logs. Value must be between 5MB and 1GB.
    pub max_upload_bytes: pulumi_wasm_rust::Output<Option<i32>>,
    /// The maximum interval in seconds for log batches. Value must be between 30 and 300.
    pub max_upload_interval_seconds: pulumi_wasm_rust::Output<Option<i32>>,
    /// The maximum number of log lines per batch. Value must be between 1000 and 1,000,000.
    pub max_upload_records: pulumi_wasm_rust::Output<Option<i32>>,
    /// The name of the logpush job to create.
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// Structured replacement for logpull*options. When including this field, the logpull*option field will be ignored.
    pub output_options: pulumi_wasm_rust::Output<
        Option<super::types::LogpushJobOutputOptions>,
    >,
    /// Ownership challenge token to prove destination ownership, required when destination is Amazon S3, Google Cloud Storage, Microsoft Azure or Sumo Logic. See [Developer documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#usage).
    pub ownership_challenge: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: LogpushJobArgs) -> LogpushJobResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let dataset_binding = args.dataset.get_inner();
    let destination_conf_binding = args.destination_conf.get_inner();
    let enabled_binding = args.enabled.get_inner();
    let filter_binding = args.filter.get_inner();
    let frequency_binding = args.frequency.get_inner();
    let kind_binding = args.kind.get_inner();
    let logpull_options_binding = args.logpull_options.get_inner();
    let max_upload_bytes_binding = args.max_upload_bytes.get_inner();
    let max_upload_interval_seconds_binding = args
        .max_upload_interval_seconds
        .get_inner();
    let max_upload_records_binding = args.max_upload_records.get_inner();
    let name_binding = args.name.get_inner();
    let output_options_binding = args.output_options.get_inner();
    let ownership_challenge_binding = args.ownership_challenge.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/logpushJob:LogpushJob".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "dataset".into(),
                value: &dataset_binding,
            },
            register_interface::ObjectField {
                name: "destinationConf".into(),
                value: &destination_conf_binding,
            },
            register_interface::ObjectField {
                name: "enabled".into(),
                value: &enabled_binding,
            },
            register_interface::ObjectField {
                name: "filter".into(),
                value: &filter_binding,
            },
            register_interface::ObjectField {
                name: "frequency".into(),
                value: &frequency_binding,
            },
            register_interface::ObjectField {
                name: "kind".into(),
                value: &kind_binding,
            },
            register_interface::ObjectField {
                name: "logpullOptions".into(),
                value: &logpull_options_binding,
            },
            register_interface::ObjectField {
                name: "maxUploadBytes".into(),
                value: &max_upload_bytes_binding,
            },
            register_interface::ObjectField {
                name: "maxUploadIntervalSeconds".into(),
                value: &max_upload_interval_seconds_binding,
            },
            register_interface::ObjectField {
                name: "maxUploadRecords".into(),
                value: &max_upload_records_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "outputOptions".into(),
                value: &output_options_binding,
            },
            register_interface::ObjectField {
                name: "ownershipChallenge".into(),
                value: &ownership_challenge_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "dataset".into() },
            register_interface::ResultField { name : "destinationConf".into() },
            register_interface::ResultField { name : "enabled".into() },
            register_interface::ResultField { name : "filter".into() },
            register_interface::ResultField { name : "frequency".into() },
            register_interface::ResultField { name : "kind".into() },
            register_interface::ResultField { name : "logpullOptions".into() },
            register_interface::ResultField { name : "maxUploadBytes".into() },
            register_interface::ResultField { name : "maxUploadIntervalSeconds".into() },
            register_interface::ResultField { name : "maxUploadRecords".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "outputOptions".into() },
            register_interface::ResultField { name : "ownershipChallenge".into() },
            register_interface::ResultField { name : "zoneId".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    LogpushJobResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        dataset: into_domain(hashmap.remove("dataset").unwrap()),
        destination_conf: into_domain(hashmap.remove("destinationConf").unwrap()),
        enabled: into_domain(hashmap.remove("enabled").unwrap()),
        filter: into_domain(hashmap.remove("filter").unwrap()),
        frequency: into_domain(hashmap.remove("frequency").unwrap()),
        kind: into_domain(hashmap.remove("kind").unwrap()),
        logpull_options: into_domain(hashmap.remove("logpullOptions").unwrap()),
        max_upload_bytes: into_domain(hashmap.remove("maxUploadBytes").unwrap()),
        max_upload_interval_seconds: into_domain(
            hashmap.remove("maxUploadIntervalSeconds").unwrap(),
        ),
        max_upload_records: into_domain(hashmap.remove("maxUploadRecords").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        output_options: into_domain(hashmap.remove("outputOptions").unwrap()),
        ownership_challenge: into_domain(hashmap.remove("ownershipChallenge").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
