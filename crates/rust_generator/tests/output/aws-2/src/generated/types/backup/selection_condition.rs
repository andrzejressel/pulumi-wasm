#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SelectionCondition {
    #[builder(into, default)]
    #[serde(rename = "stringEquals")]
    pub r#string_equals: Box<Option<Vec<super::super::types::backup::SelectionConditionStringEqual>>>,
    #[builder(into, default)]
    #[serde(rename = "stringLikes")]
    pub r#string_likes: Box<Option<Vec<super::super::types::backup::SelectionConditionStringLike>>>,
    #[builder(into, default)]
    #[serde(rename = "stringNotEquals")]
    pub r#string_not_equals: Box<Option<Vec<super::super::types::backup::SelectionConditionStringNotEqual>>>,
    #[builder(into, default)]
    #[serde(rename = "stringNotLikes")]
    pub r#string_not_likes: Box<Option<Vec<super::super::types::backup::SelectionConditionStringNotLike>>>,
}
