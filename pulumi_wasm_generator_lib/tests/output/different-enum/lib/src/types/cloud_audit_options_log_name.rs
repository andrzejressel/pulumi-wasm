//! The log_name to populate in the Cloud Audit Record. This is added to regress pulumi/pulumi issue #7913

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
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
