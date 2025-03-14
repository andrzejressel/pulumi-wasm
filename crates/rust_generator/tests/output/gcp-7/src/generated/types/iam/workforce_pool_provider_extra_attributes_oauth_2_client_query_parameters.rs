#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkforcePoolProviderExtraAttributesOauth2ClientQueryParameters {
    /// The filter used to request specific records from IdP. In case of attributes type as AZURE_AD_GROUPS_MAIL, it represents the
    /// filter used to request specific groups for users from IdP. By default, all of the groups associated with the user are fetched. The
    /// groups should be mail enabled and security enabled. See https://learn.microsoft.com/en-us/graph/search-query-parameter for more details.
    #[builder(into, default)]
    #[serde(rename = "filter")]
    pub r#filter: Box<Option<String>>,
}
