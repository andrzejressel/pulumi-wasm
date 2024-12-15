//! ## Example Usage
//! 
//! ## Import
//! 
//! Import an account-scoped job.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/logpushJob:LogpushJob example account/<account_id>/<job_id>
//! ```
//! 
//! Import a zone-scoped job.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/logpushJob:LogpushJob example zone/<zone_id>/<job_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct LogpushJobArgs {
    /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The kind of the dataset to use with the logpush job. Available values: `access_requests`, `casb_findings`, `firewall_events`, `http_requests`, `spectrum_events`, `nel_reports`, `audit_logs`, `gateway_dns`, `gateway_http`, `gateway_network`, `dns_logs`, `network_analytics_logs`, `workers_trace_events`, `device_posture_results`, `zero_trust_network_sessions`, `magic_ids_detections`, `page_shield_events`.
    #[builder(into)]
    pub dataset: pulumi_wasm_rust::Output<String>,
    /// Uniquely identifies a resource (such as an s3 bucket) where data will be pushed. Additional configuration parameters supported by the destination may be included. See [Logpush destination documentation](https://developers.cloudflare.com/logs/reference/logpush-api-configuration#destination).
    #[builder(into)]
    pub destination_conf: pulumi_wasm_rust::Output<String>,
    /// Whether to enable the job.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Use filters to select the events to include and/or remove from your logs. For more information, refer to [Filters](https://developers.cloudflare.com/logs/reference/logpush-api-configuration/filters/).
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub filter: pulumi_wasm_rust::Output<Option<String>>,
    /// A higher frequency will result in logs being pushed on faster with smaller files. `low` frequency will push logs less often with larger files. Available values: `high`, `low`. Defaults to `high`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub frequency: pulumi_wasm_rust::Output<Option<String>>,
    /// The kind of logpush job to create. Available values: `edge`, `instant-logs`, `""`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub kind: pulumi_wasm_rust::Output<Option<String>>,
    /// Configuration string for the Logshare API. It specifies things like requested fields and timestamp formats. See [Logpush options documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#options).
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub logpull_options: pulumi_wasm_rust::Output<Option<String>>,
    /// The maximum uncompressed file size of a batch of logs. Value must be between 5MB and 1GB.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub max_upload_bytes: pulumi_wasm_rust::Output<Option<i32>>,
    /// The maximum interval in seconds for log batches. Value must be between 30 and 300.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub max_upload_interval_seconds: pulumi_wasm_rust::Output<Option<i32>>,
    /// The maximum number of log lines per batch. Value must be between 1000 and 1,000,000.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub max_upload_records: pulumi_wasm_rust::Output<Option<i32>>,
    /// The name of the logpush job to create.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// Structured replacement for logpull*options. When including this field, the logpull*option field will be ignored.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub output_options: pulumi_wasm_rust::Output<Option<crate::types::LogpushJobOutputOptions>>,
    /// Ownership challenge token to prove destination ownership, required when destination is Amazon S3, Google Cloud Storage, Microsoft Azure or Sumo Logic. See [Developer documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#usage).
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ownership_challenge: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
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
    pub output_options: pulumi_wasm_rust::Output<Option<crate::types::LogpushJobOutputOptions>>,
    /// Ownership challenge token to prove destination ownership, required when destination is Amazon S3, Google Cloud Storage, Microsoft Azure or Sumo Logic. See [Developer documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#usage).
    pub ownership_challenge: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: LogpushJobArgs) -> LogpushJobResult {

    let result = crate::bindings::pulumi::cloudflare::logpush_job::invoke(name, &crate::bindings::pulumi::cloudflare::logpush_job::Args {
        account_id: &args.account_id.get_inner(),
        dataset: &args.dataset.get_inner(),
        destination_conf: &args.destination_conf.get_inner(),
        enabled: &args.enabled.get_inner(),
        filter: &args.filter.get_inner(),
        frequency: &args.frequency.get_inner(),
        kind: &args.kind.get_inner(),
        logpull_options: &args.logpull_options.get_inner(),
        max_upload_bytes: &args.max_upload_bytes.get_inner(),
        max_upload_interval_seconds: &args.max_upload_interval_seconds.get_inner(),
        max_upload_records: &args.max_upload_records.get_inner(),
        name: &args.name.get_inner(),
        output_options: &args.output_options.get_inner(),
        ownership_challenge: &args.ownership_challenge.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    LogpushJobResult {
        account_id: crate::into_domain(result.account_id),
        dataset: crate::into_domain(result.dataset),
        destination_conf: crate::into_domain(result.destination_conf),
        enabled: crate::into_domain(result.enabled),
        filter: crate::into_domain(result.filter),
        frequency: crate::into_domain(result.frequency),
        kind: crate::into_domain(result.kind),
        logpull_options: crate::into_domain(result.logpull_options),
        max_upload_bytes: crate::into_domain(result.max_upload_bytes),
        max_upload_interval_seconds: crate::into_domain(result.max_upload_interval_seconds),
        max_upload_records: crate::into_domain(result.max_upload_records),
        name: crate::into_domain(result.name),
        output_options: crate::into_domain(result.output_options),
        ownership_challenge: crate::into_domain(result.ownership_challenge),
        zone_id: crate::into_domain(result.zone_id),
    }
}
