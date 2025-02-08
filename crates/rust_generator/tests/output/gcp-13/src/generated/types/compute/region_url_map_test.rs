#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RegionUrlMapTest {
    /// Description of this test case.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Host portion of the URL.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// Path portion of the URL.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// A reference to expected RegionBackendService resource the given URL should be mapped to.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
