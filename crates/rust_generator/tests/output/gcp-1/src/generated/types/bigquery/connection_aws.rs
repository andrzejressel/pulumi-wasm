#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectionAws {
    /// Authentication using Google owned service account to assume into customer's AWS IAM Role.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "accessRole")]
    pub r#access_role: Box<super::super::types::bigquery::ConnectionAwsAccessRole>,
}
