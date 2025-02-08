#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutonomousDatabasePropertiesCustomerContact {
    /// The email address used by Oracle to send notifications regarding databases
    /// and infrastructure.
    /// 
    /// <a name="nested_apex_details"></a>The `apex_details` block contains:
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Box<String>,
}
