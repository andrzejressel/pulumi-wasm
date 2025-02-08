#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserIdentityInfo {
    /// The email address. If you are using SAML for identity management and include this parameter, an error is returned. Note that updates to the `email` is supported. From the [UpdateUserIdentityInfo API documentation](https://docs.aws.amazon.com/connect/latest/APIReference/API_UpdateUserIdentityInfo.html) it is strongly recommended to limit who has the ability to invoke `UpdateUserIdentityInfo`. Someone with that ability can change the login credentials of other users by changing their email address. This poses a security risk to your organization. They can change the email address of a user to the attacker's email address, and then reset the password through email. For more information, see [Best Practices for Security Profiles](https://docs.aws.amazon.com/connect/latest/adminguide/security-profile-best-practices.html) in the Amazon Connect Administrator Guide.
    #[builder(into, default)]
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    /// The first name. This is required if you are using Amazon Connect or SAML for identity management. Minimum length of 1. Maximum length of 100.
    #[builder(into, default)]
    #[serde(rename = "firstName")]
    pub r#first_name: Box<Option<String>>,
    /// The last name. This is required if you are using Amazon Connect or SAML for identity management. Minimum length of 1. Maximum length of 100.
    #[builder(into, default)]
    #[serde(rename = "lastName")]
    pub r#last_name: Box<Option<String>>,
}
