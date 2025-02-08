#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BackendCredentials {
    /// An `authorization` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "authorization")]
    pub r#authorization: Box<Option<super::super::types::apimanagement::BackendCredentialsAuthorization>>,
    /// A list of client certificate thumbprints to present to the backend host. The certificates must exist within the API Management Service.
    #[builder(into, default)]
    #[serde(rename = "certificates")]
    pub r#certificates: Box<Option<Vec<String>>>,
    /// A mapping of header parameters to pass to the backend host. The keys are the header names and the values are a comma separated string of header values. This is converted to a list before being passed to the API.
    #[builder(into, default)]
    #[serde(rename = "header")]
    pub r#header: Box<Option<std::collections::HashMap<String, String>>>,
    /// A mapping of query parameters to pass to the backend host. The keys are the query names and the values are a comma separated string of query values. This is converted to a list before being passed to the API.
    #[builder(into, default)]
    #[serde(rename = "query")]
    pub r#query: Box<Option<std::collections::HashMap<String, String>>>,
}
