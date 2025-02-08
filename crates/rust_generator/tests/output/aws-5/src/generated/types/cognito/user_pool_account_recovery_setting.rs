#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct UserPoolAccountRecoverySetting {
    /// List of Account Recovery Options of the following structure:
    #[builder(into, default)]
    #[serde(rename = "recoveryMechanisms")]
    pub r#recovery_mechanisms: Box<Option<Vec<super::super::types::cognito::UserPoolAccountRecoverySettingRecoveryMechanism>>>,
}
