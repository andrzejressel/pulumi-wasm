#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FrontdoorOriginGroupHealthProbe {
    /// Specifies the number of seconds between health probes. Possible values are between `1` and `255` seconds (inclusive).
    #[builder(into)]
    #[serde(rename = "intervalInSeconds")]
    pub r#interval_in_seconds: Box<i32>,
    /// Specifies the path relative to the origin that is used to determine the health of the origin. Defaults to `/`.
    /// 
    /// > **NOTE:** Health probes can only be disabled if there is a single enabled origin in a single enabled origin group. For more information about the `health_probe` settings please see the [product documentation](https://docs.microsoft.com/azure/frontdoor/health-probes).
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Specifies the protocol to use for health probe. Possible values are `Http` and `Https`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// Specifies the type of health probe request that is made. Possible values are `GET` and `HEAD`. Defaults to `HEAD`.
    #[builder(into, default)]
    #[serde(rename = "requestType")]
    pub r#request_type: Box<Option<String>>,
}
