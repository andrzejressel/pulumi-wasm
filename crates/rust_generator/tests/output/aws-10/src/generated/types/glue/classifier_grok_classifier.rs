#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClassifierGrokClassifier {
    /// An identifier of the data format that the classifier matches, such as Twitter, JSON, Omniture logs, Amazon CloudWatch Logs, and so on.
    #[builder(into)]
    #[serde(rename = "classification")]
    pub r#classification: Box<String>,
    /// Custom grok patterns used by this classifier.
    #[builder(into, default)]
    #[serde(rename = "customPatterns")]
    pub r#custom_patterns: Box<Option<String>>,
    /// The grok pattern used by this classifier.
    #[builder(into)]
    #[serde(rename = "grokPattern")]
    pub r#grok_pattern: Box<String>,
}
