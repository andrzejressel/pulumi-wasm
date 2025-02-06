#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccessLevelConditionDevicePolicyOsConstraint {
    /// The minimum allowed OS version. If not set, any version
    /// of this OS satisfies the constraint.
    /// Format: "major.minor.patch" such as "10.5.301", "9.2.1".
    #[builder(into, default)]
    #[serde(rename = "minimumVersion")]
    pub r#minimum_version: Box<Option<String>>,
    /// The operating system type of the device.
    /// Possible values are: `OS_UNSPECIFIED`, `DESKTOP_MAC`, `DESKTOP_WINDOWS`, `DESKTOP_LINUX`, `DESKTOP_CHROME_OS`, `ANDROID`, `IOS`.
    #[builder(into)]
    #[serde(rename = "osType")]
    pub r#os_type: Box<String>,
}
