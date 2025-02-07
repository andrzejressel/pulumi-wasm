#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SearchEngineCommonConfig {
    /// The name of the company, business or entity that is associated with the engine. Setting this may help improve LLM related features.cd
    #[builder(into, default)]
    #[serde(rename = "companyName")]
    pub r#company_name: Box<Option<String>>,
}
