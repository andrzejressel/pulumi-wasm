#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct XssMatchSetXssMatchTuple {
    /// Specifies where in a web request to look for cross-site scripting attacks.
    #[builder(into)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Box<super::super::types::wafregional::XssMatchSetXssMatchTupleFieldToMatch>,
    /// Which text transformation, if any, to perform on the web request before inspecting the request for cross-site scripting attacks.
    #[builder(into)]
    #[serde(rename = "textTransformation")]
    pub r#text_transformation: Box<String>,
}
