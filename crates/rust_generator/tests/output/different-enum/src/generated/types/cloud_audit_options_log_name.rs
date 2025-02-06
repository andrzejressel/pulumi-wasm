#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum CloudAuditOptionsLogName {
    /// Default. Should not be used.
    #[serde(rename = "UNSPECIFIED_LOG_NAME")]
    UnspecifiedLogName,
    /// Corresponds to "cloudaudit.googleapis.com/activity"
    #[serde(rename = "ADMIN_ACTIVITY")]
    AdminActivity,
    /// Corresponds to "cloudaudit.googleapis.com/data_access"
    #[serde(rename = "DATA_ACCESS")]
    DataAccess,
    /// What if triple quotes """ are used in the description
    #[serde(rename = "SYNTHETIC")]
    Synthetic,
}
