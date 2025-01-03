#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccessLevelBasicConditionDevicePolicyOsConstraint {
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
    /// If you specify DESKTOP_CHROME_OS for osType, you can optionally include requireVerifiedChromeOs to require Chrome Verified Access.
    #[builder(into, default)]
    #[serde(rename = "requireVerifiedChromeOs")]
    pub r#require_verified_chrome_os: Box<Option<bool>>,
}
