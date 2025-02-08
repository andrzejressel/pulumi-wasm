#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BucketObjectRetention {
    /// The retention policy mode. Either `Locked` or `Unlocked`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// The time to retain the object until in RFC 3339 format, for example 2012-11-15T16:19:00.094Z.
    /// 
    /// <a name>
    #[builder(into)]
    #[serde(rename = "retainUntilTime")]
    pub r#retain_until_time: Box<String>,
}
