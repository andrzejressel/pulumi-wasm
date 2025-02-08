#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRecommendationsRecommendation {
    /// The category of the recommendation.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: Box<String>,
    /// The description of the issue or the opportunity identified by the recommendation.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The business impact of the recommendation.
    #[builder(into)]
    #[serde(rename = "impact")]
    pub r#impact: Box<String>,
    /// The name of the Advisor Recommendation.
    #[builder(into)]
    #[serde(rename = "recommendationName")]
    pub r#recommendation_name: Box<String>,
    /// The recommendation type id of the Advisor Recommendation.
    #[builder(into)]
    #[serde(rename = "recommendationTypeId")]
    pub r#recommendation_type_id: Box<String>,
    /// The name of the identified resource of the Advisor Recommendation.
    #[builder(into)]
    #[serde(rename = "resourceName")]
    pub r#resource_name: Box<String>,
    /// The type of the identified resource of the Advisor Recommendation.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Box<String>,
    /// A list of Advisor Suppression names of the Advisor Recommendation.
    #[builder(into)]
    #[serde(rename = "suppressionNames")]
    pub r#suppression_names: Box<Vec<String>>,
    /// The most recent time that Advisor checked the validity of the recommendation..
    #[builder(into)]
    #[serde(rename = "updatedTime")]
    pub r#updated_time: Box<String>,
}
