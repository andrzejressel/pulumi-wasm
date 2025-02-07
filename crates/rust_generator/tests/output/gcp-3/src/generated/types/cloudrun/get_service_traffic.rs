#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTraffic {
    /// LatestRevision may be optionally provided to indicate that the latest ready
    /// Revision of the Configuration should be used for this traffic target. When
    /// provided LatestRevision must be true if RevisionName is empty; it must be
    /// false when RevisionName is non-empty.
    #[builder(into)]
    #[serde(rename = "latestRevision")]
    pub r#latest_revision: Box<bool>,
    /// Percent specifies percent of the traffic to this Revision or Configuration.
    #[builder(into)]
    #[serde(rename = "percent")]
    pub r#percent: Box<i32>,
    /// RevisionName of a specific revision to which to send this portion of traffic.
    #[builder(into)]
    #[serde(rename = "revisionName")]
    pub r#revision_name: Box<String>,
    /// Tag is optionally used to expose a dedicated url for referencing this target exclusively.
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Box<String>,
    /// URL displays the URL for accessing tagged traffic targets. URL is displayed in status,
    /// and is disallowed on spec. URL must contain a scheme (e.g. http://) and a hostname,
    /// but may not contain anything else (e.g. basic auth, url path, etc.)
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
