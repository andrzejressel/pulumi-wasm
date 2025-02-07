#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SearchResource {
    /// Amazon resource name of resource.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// The date and time that the information about this resource property was last updated.
    #[builder(into)]
    #[serde(rename = "lastReportedAt")]
    pub r#last_reported_at: Box<String>,
    /// Amazon Web Services account that owns the resource.
    #[builder(into)]
    #[serde(rename = "owningAccountId")]
    pub r#owning_account_id: Box<String>,
    /// Structure with additional type-specific details about the resource.  See `properties` below.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Vec<super::super::types::resourceexplorer::SearchResourceProperty>>,
    /// Amazon Web Services Region in which the resource was created and exists.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
    /// Type of the resource.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Box<String>,
    /// Amazon Web Service that owns the resource and is responsible for creating and updating it.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
