#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessLevelConditionDevicePolicy {
    /// A list of allowed device management levels.
    /// An empty list allows all management levels.
    /// Each value may be one of: `MANAGEMENT_UNSPECIFIED`, `NONE`, `BASIC`, `COMPLETE`.
    #[builder(into, default)]
    #[serde(rename = "allowedDeviceManagementLevels")]
    pub r#allowed_device_management_levels: Box<Option<Vec<String>>>,
    /// A list of allowed encryptions statuses.
    /// An empty list allows all statuses.
    /// Each value may be one of: `ENCRYPTION_UNSPECIFIED`, `ENCRYPTION_UNSUPPORTED`, `UNENCRYPTED`, `ENCRYPTED`.
    #[builder(into, default)]
    #[serde(rename = "allowedEncryptionStatuses")]
    pub r#allowed_encryption_statuses: Box<Option<Vec<String>>>,
    /// A list of allowed OS versions.
    /// An empty list allows all types and all versions.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "osConstraints")]
    pub r#os_constraints: Box<Option<Vec<super::super::types::accesscontextmanager::AccessLevelConditionDevicePolicyOsConstraint>>>,
    /// Whether the device needs to be approved by the customer admin.
    #[builder(into, default)]
    #[serde(rename = "requireAdminApproval")]
    pub r#require_admin_approval: Box<Option<bool>>,
    /// Whether the device needs to be corp owned.
    #[builder(into, default)]
    #[serde(rename = "requireCorpOwned")]
    pub r#require_corp_owned: Box<Option<bool>>,
    /// Whether or not screenlock is required for the DevicePolicy
    /// to be true. Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "requireScreenLock")]
    pub r#require_screen_lock: Box<Option<bool>>,
}
