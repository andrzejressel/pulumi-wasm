#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLinuxFunctionAppStickySetting {
    /// A list of `app_setting` names that the Linux Function App will not swap between Slots when a swap operation is triggered.
    #[builder(into)]
    #[serde(rename = "appSettingNames")]
    pub r#app_setting_names: Box<Vec<String>>,
    /// A list of `connection_string` names that the Linux Function App will not swap between Slots when a swap operation is triggered.
    #[builder(into)]
    #[serde(rename = "connectionStringNames")]
    pub r#connection_string_names: Box<Vec<String>>,
}
