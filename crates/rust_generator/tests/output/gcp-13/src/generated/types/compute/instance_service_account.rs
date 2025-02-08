#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceServiceAccount {
    /// The service account e-mail address.
    /// **Note**: `allow_stopping_for_update` must be set to true or your instance must have a `desired_status` of `TERMINATED` in order to update this field.
    #[builder(into, default)]
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    /// A list of service scopes. Both OAuth2 URLs and gcloud
    /// short names are supported. To allow full access to all Cloud APIs, use the
    /// `cloud-platform` scope. See a complete list of scopes [here](https://cloud.google.com/sdk/gcloud/reference/alpha/compute/instances/set-scopes#--scopes).
    /// **Note**: `allow_stopping_for_update` must be set to true or your instance must have a `desired_status` of `TERMINATED` in order to update this field.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Box<Vec<String>>,
}
