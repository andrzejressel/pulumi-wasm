#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ActivityLogAlertCriteria {
    /// The email address or Azure Active Directory identifier of the user who performed the operation.
    #[builder(into, default)]
    #[serde(rename = "caller")]
    pub r#caller: Box<Option<String>>,
    /// The category of the operation. Possible values are `Administrative`, `Autoscale`, `Policy`, `Recommendation`, `ResourceHealth`, `Security` and `ServiceHealth`.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: Box<String>,
    /// The severity level of the event. Possible values are `Verbose`, `Informational`, `Warning`, `Error`, and `Critical`.
    #[builder(into, default)]
    #[serde(rename = "level")]
    pub r#level: Box<Option<String>>,
    /// A list of severity level of the event. Possible values are `Verbose`, `Informational`, `Warning`, `Error`, and `Critical`.
    /// 
    /// > **NOTE:** `level` and `levels` are mutually exclusive.
    #[builder(into, default)]
    #[serde(rename = "levels")]
    pub r#levels: Box<Option<Vec<String>>>,
    /// The Resource Manager Role-Based Access Control operation name. Supported operation should be of the form: `<resourceProvider>/<resourceType>/<operation>`.
    #[builder(into, default)]
    #[serde(rename = "operationName")]
    pub r#operation_name: Box<Option<String>>,
    /// The recommendation category of the event. Possible values are `Cost`, `Reliability`, `OperationalExcellence`, `HighAvailability` and `Performance`. It is only allowed when `category` is `Recommendation`.
    #[builder(into, default)]
    #[serde(rename = "recommendationCategory")]
    pub r#recommendation_category: Box<Option<String>>,
    /// The recommendation impact of the event. Possible values are `High`, `Medium` and `Low`. It is only allowed when `category` is `Recommendation`.
    #[builder(into, default)]
    #[serde(rename = "recommendationImpact")]
    pub r#recommendation_impact: Box<Option<String>>,
    /// The recommendation type of the event. It is only allowed when `category` is `Recommendation`.
    #[builder(into, default)]
    #[serde(rename = "recommendationType")]
    pub r#recommendation_type: Box<Option<String>>,
    /// The name of resource group monitored by the activity log alert.
    #[builder(into, default)]
    #[serde(rename = "resourceGroup")]
    pub r#resource_group: Box<Option<String>>,
    /// A list of names of resource groups monitored by the activity log alert.
    /// 
    /// > **NOTE:** `resource_group` and `resource_groups` are mutually exclusive.
    #[builder(into, default)]
    #[serde(rename = "resourceGroups")]
    pub r#resource_groups: Box<Option<Vec<String>>>,
    /// A block to define fine grain resource health settings.
    #[builder(into, default)]
    #[serde(rename = "resourceHealth")]
    pub r#resource_health: Box<Option<super::super::types::monitoring::ActivityLogAlertCriteriaResourceHealth>>,
    /// The specific resource monitored by the activity log alert. It should be within one of the `scopes`.
    #[builder(into, default)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: Box<Option<String>>,
    /// A list of specific resources monitored by the activity log alert. It should be within one of the `scopes`.
    /// 
    /// > **NOTE:** `resource_id` and `resource_ids` are mutually exclusive.
    #[builder(into, default)]
    #[serde(rename = "resourceIds")]
    pub r#resource_ids: Box<Option<Vec<String>>>,
    /// The name of the resource provider monitored by the activity log alert.
    #[builder(into, default)]
    #[serde(rename = "resourceProvider")]
    pub r#resource_provider: Box<Option<String>>,
    /// A list of names of resource providers monitored by the activity log alert.
    /// 
    /// > **NOTE:** `resource_provider` and `resource_providers` are mutually exclusive.
    #[builder(into, default)]
    #[serde(rename = "resourceProviders")]
    pub r#resource_providers: Box<Option<Vec<String>>>,
    /// The resource type monitored by the activity log alert.
    #[builder(into, default)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Box<Option<String>>,
    /// A list of resource types monitored by the activity log alert.
    /// 
    /// > **NOTE:** `resource_type` and `resource_types` are mutually exclusive.
    #[builder(into, default)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Box<Option<Vec<String>>>,
    /// A block to define fine grain service health settings.
    #[builder(into, default)]
    #[serde(rename = "serviceHealth")]
    pub r#service_health: Box<Option<super::super::types::monitoring::ActivityLogAlertCriteriaServiceHealth>>,
    /// The status of the event. For example, `Started`, `Failed`, or `Succeeded`.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// A list of status of the event. For example, `Started`, `Failed`, or `Succeeded`.
    /// 
    /// > **NOTE:** `status` and `statuses` are mutually exclusive.
    #[builder(into, default)]
    #[serde(rename = "statuses")]
    pub r#statuses: Box<Option<Vec<String>>>,
    /// The sub status of the event.
    #[builder(into, default)]
    #[serde(rename = "subStatus")]
    pub r#sub_status: Box<Option<String>>,
    /// A list of sub status of the event.
    /// 
    /// > **NOTE:** `sub_status` and `sub_statuses` are mutually exclusive.
    #[builder(into, default)]
    #[serde(rename = "subStatuses")]
    pub r#sub_statuses: Box<Option<Vec<String>>>,
}
