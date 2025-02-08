#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EmailTemplateEmailTemplateHeader {
    /// Name of the message header. The header name can contain up to 126 characters.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Value of the message header. The header value can contain up to 870 characters, including the length of any rendered attributes. For example if you add the {CreationDate} attribute, it renders as YYYY-MM-DDTHH:MM:SS.SSSZ and is 24 characters in length.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
