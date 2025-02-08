#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetVirtualNodeSpecLogging {
    #[builder(into)]
    #[serde(rename = "accessLogs")]
    pub r#access_logs: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecLoggingAccessLog>>,
}
