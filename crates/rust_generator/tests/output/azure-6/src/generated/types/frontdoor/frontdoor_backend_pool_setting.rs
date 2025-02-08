#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FrontdoorBackendPoolSetting {
    /// Specifies the send and receive timeout on forwarding request to the backend. When the timeout is reached, the request fails and returns. Possible values are between `0` - `240`. Defaults to `60`.
    #[builder(into, default)]
    #[serde(rename = "backendPoolsSendReceiveTimeoutSeconds")]
    pub r#backend_pools_send_receive_timeout_seconds: Box<Option<i32>>,
    /// Enforce certificate name check on `HTTPS` requests to all backend pools, this setting will have no effect on `HTTP` requests. Permitted values are `true` or `false`.
    /// 
    /// > **NOTE:** `backend_pools_send_receive_timeout_seconds` and `enforce_backend_pools_certificate_name_check` apply to all backend pools.
    #[builder(into)]
    #[serde(rename = "enforceBackendPoolsCertificateNameCheck")]
    pub r#enforce_backend_pools_certificate_name_check: Box<bool>,
}
