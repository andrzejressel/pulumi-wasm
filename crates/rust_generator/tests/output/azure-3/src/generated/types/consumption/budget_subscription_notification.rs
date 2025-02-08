#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BudgetSubscriptionNotification {
    /// Specifies a list of email addresses to send the budget notification to when the threshold is exceeded.
    #[builder(into, default)]
    #[serde(rename = "contactEmails")]
    pub r#contact_emails: Box<Option<Vec<String>>>,
    /// Specifies a list of Action Group IDs to send the budget notification to when the threshold is exceeded.
    #[builder(into, default)]
    #[serde(rename = "contactGroups")]
    pub r#contact_groups: Box<Option<Vec<String>>>,
    /// Specifies a list of contact roles to send the budget notification to when the threshold is exceeded.
    #[builder(into, default)]
    #[serde(rename = "contactRoles")]
    pub r#contact_roles: Box<Option<Vec<String>>>,
    /// Should the notification be enabled? Defaults to `true`.
    /// 
    /// > **NOTE:** A `notification` block cannot have all of `contact_emails`, `contact_roles`, and `contact_groups` empty. This means that at least one of the three must be specified.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The comparison operator for the notification. Must be one of `EqualTo`, `GreaterThan`, or `GreaterThanOrEqualTo`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// Threshold value associated with a notification. Notification is sent when the cost exceeded the threshold. It is always percent and has to be between 0 and 1000.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: Box<i32>,
    /// The type of threshold for the notification. This determines whether the notification is triggered by forecasted costs or actual costs. The allowed values are `Actual` and `Forecasted`. Default is `Actual`.
    #[builder(into, default)]
    #[serde(rename = "thresholdType")]
    pub r#threshold_type: Box<Option<String>>,
}
