#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSecretReplication {
    /// The Secret will automatically be replicated without any restrictions.
    #[builder(into)]
    #[serde(rename = "autos")]
    pub r#autos: Box<Vec<super::super::types::secretmanager::GetSecretReplicationAuto>>,
    /// The Secret will be replicated to the regions specified by the user.
    #[builder(into)]
    #[serde(rename = "userManageds")]
    pub r#user_manageds: Box<Vec<super::super::types::secretmanager::GetSecretReplicationUserManaged>>,
}
