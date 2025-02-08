#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SoftwareUpdateConfigurationLinux {
    /// Specifies the list of update classifications included in the Software Update Configuration. Possible values are `Unclassified`, `Critical`, `Security` and `Other`.
    /// 
    /// > **NOTE:** The `classifications_included` property will become `Required` in version 4.0 of the Provider.
    #[builder(into)]
    #[serde(rename = "classificationsIncludeds")]
    pub r#classifications_includeds: Box<Vec<String>>,
    /// Specifies a list of packages to excluded from the Software Update Configuration.
    #[builder(into, default)]
    #[serde(rename = "excludedPackages")]
    pub r#excluded_packages: Box<Option<Vec<String>>>,
    /// Specifies a list of packages to included from the Software Update Configuration.
    #[builder(into, default)]
    #[serde(rename = "includedPackages")]
    pub r#included_packages: Box<Option<Vec<String>>>,
    /// Specifies the reboot settings after software update, possible values are `IfRequired`, `Never`, `RebootOnly` and `Always`. Defaults to `IfRequired`.
    #[builder(into, default)]
    #[serde(rename = "reboot")]
    pub r#reboot: Box<Option<String>>,
}
