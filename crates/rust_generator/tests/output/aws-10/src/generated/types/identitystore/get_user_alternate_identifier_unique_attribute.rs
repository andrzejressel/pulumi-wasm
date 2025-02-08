#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetUserAlternateIdentifierUniqueAttribute {
    /// Attribute path that is used to specify which attribute name to search. For example: `UserName`. Refer to the [User data type](https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/API_User.html).
    #[builder(into)]
    #[serde(rename = "attributePath")]
    pub r#attribute_path: Box<String>,
    /// Value for an attribute.
    #[builder(into)]
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Box<String>,
}
