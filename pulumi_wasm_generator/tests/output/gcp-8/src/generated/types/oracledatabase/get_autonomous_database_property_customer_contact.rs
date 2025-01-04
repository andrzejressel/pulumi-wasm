#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAutonomousDatabasePropertyCustomerContact {
    /// The email address used by Oracle to send notifications regarding databases
    /// and infrastructure.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Box<String>,
}
