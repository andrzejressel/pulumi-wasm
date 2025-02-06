#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthomationRuleActionIncident {
    /// The classification of the incident, when closing it. Possible values are: `BenignPositive_SuspiciousButExpected`, `FalsePositive_InaccurateData`, `FalsePositive_IncorrectAlertLogic`, `TruePositive_SuspiciousActivity` and `Undetermined`.
    /// 
    /// > **Note:** The `classification` is required when `status` is `Closed`.
    #[builder(into, default)]
    #[serde(rename = "classification")]
    pub r#classification: Box<Option<String>>,
    /// The comment why the incident is to be closed.
    /// 
    /// > **Note:** The `classification_comment` is allowed to set only when `status` is `Closed`.
    #[builder(into, default)]
    #[serde(rename = "classificationComment")]
    pub r#classification_comment: Box<Option<String>>,
    /// Specifies a list of labels to add to the incident.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<Vec<String>>>,
    /// The execution order of this action.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Box<i32>,
    /// The object ID of the entity this incident is assigned to.
    #[builder(into, default)]
    #[serde(rename = "ownerId")]
    pub r#owner_id: Box<Option<String>>,
    /// The severity to add to the incident. Possible values are `High`, `Informational`, `Low` and `Medium`.
    /// 
    /// > **Note:**: At least one of `status`, `labels`, `owner_id` and `severity` has to be set.
    #[builder(into, default)]
    #[serde(rename = "severity")]
    pub r#severity: Box<Option<String>>,
    /// The status to set to the incident. Possible values are: `Active`, `Closed`, `New`.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
