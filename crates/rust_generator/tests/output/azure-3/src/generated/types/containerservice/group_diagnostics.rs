#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupDiagnostics {
    /// A `log_analytics` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "logAnalytics")]
    pub r#log_analytics: Box<super::super::types::containerservice::GroupDiagnosticsLogAnalytics>,
}
