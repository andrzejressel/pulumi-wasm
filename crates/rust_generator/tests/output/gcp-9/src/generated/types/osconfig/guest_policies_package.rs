#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GuestPoliciesPackage {
    /// The desiredState the agent should maintain for this package. The default is to ensure the package is installed.
    /// Possible values are: `INSTALLED`, `UPDATED`, `REMOVED`.
    #[builder(into, default)]
    #[serde(rename = "desiredState")]
    pub r#desired_state: Box<Option<String>>,
    /// Type of package manager that can be used to install this package. If a system does not have the package manager,
    /// the package is not installed or removed no error message is returned. By default, or if you specify ANY,
    /// the agent attempts to install and remove this package using the default package manager.
    /// This is useful when creating a policy that applies to different types of systems.
    /// The default behavior is ANY.
    /// Default value is `ANY`.
    /// Possible values are: `ANY`, `APT`, `YUM`, `ZYPPER`, `GOO`.
    #[builder(into, default)]
    #[serde(rename = "manager")]
    pub r#manager: Box<Option<String>>,
    /// The name of the package. A package is uniquely identified for conflict validation
    /// by checking the package name and the manager(s) that the package targets.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
