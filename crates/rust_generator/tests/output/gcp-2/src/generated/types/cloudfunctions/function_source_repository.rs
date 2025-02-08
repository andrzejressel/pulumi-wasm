#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FunctionSourceRepository {
    /// The URL pointing to the hosted repository where the function was defined at the time of deployment.
    #[builder(into, default)]
    #[serde(rename = "deployedUrl")]
    pub r#deployed_url: Box<Option<String>>,
    /// The URL pointing to the hosted repository where the function is defined. There are supported Cloud Source Repository URLs in the following formats:
    /// 
    /// * To refer to a specific commit: `https://source.developers.google.com/projects/*/repos/*/revisions/*/paths/*`
    /// * To refer to a moveable alias (branch): `https://source.developers.google.com/projects/*/repos/*/moveable-aliases/*/paths/*`. To refer to HEAD, use the `master` moveable alias.
    /// * To refer to a specific fixed alias (tag): `https://source.developers.google.com/projects/*/repos/*/fixed-aliases/*/paths/*`
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
