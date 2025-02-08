#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FrontdoorBackendPoolHealthProbe {
    /// Is this health probe enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The ID of the FrontDoor.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The number of seconds between each Health Probe. Defaults to `120`.
    #[builder(into, default)]
    #[serde(rename = "intervalInSeconds")]
    pub r#interval_in_seconds: Box<Option<i32>>,
    /// Specifies the name of the Health Probe.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The path to use for the Health Probe. Default is `/`.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Specifies HTTP method the health probe uses when querying the backend pool instances. Possible values include: `GET` and `HEAD`. Defaults to `GET`.
    /// 
    /// > **NOTE:** Use the `HEAD` method if you do not need to check the response body of your health probe.
    #[builder(into, default)]
    #[serde(rename = "probeMethod")]
    pub r#probe_method: Box<Option<String>>,
    /// Protocol scheme to use for the Health Probe. Possible values are `Http` and `Https`. Defaults to `Http`.
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
}
