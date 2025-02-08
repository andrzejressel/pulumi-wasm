#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetFrameworkControl {
    /// One or more input parameter blocks. An example of a control with two parameters is: "backup plan frequency is at least daily and the retention period is at least 1 year". The first parameter is daily. The second parameter is 1 year. Detailed below.
    #[builder(into)]
    #[serde(rename = "inputParameters")]
    pub r#input_parameters: Box<Vec<super::super::types::backup::GetFrameworkControlInputParameter>>,
    /// Backup framework name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Scope of a control. The control scope defines what the control will evaluate. Three examples of control scopes are: a specific backup plan, all backup plans with a specific tag, or all backup plans. Detailed below.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Box<Vec<super::super::types::backup::GetFrameworkControlScope>>,
}
