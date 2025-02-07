#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpaceOwnershipSettings {
    /// The user profile who is the owner of the private space.
    #[builder(into)]
    #[serde(rename = "ownerUserProfileName")]
    pub r#owner_user_profile_name: Box<String>,
}
