#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkstationConfigAllowedPort {
    /// Starting port number for the current range of ports. Valid ports are 22, 80, and ports within the range 1024-65535.
    #[builder(into, default)]
    #[serde(rename = "first")]
    pub r#first: Box<Option<i32>>,
    /// Ending port number for the current range of ports. Valid ports are 22, 80, and ports within the range 1024-65535.
    #[builder(into, default)]
    #[serde(rename = "last")]
    pub r#last: Box<Option<i32>>,
}
