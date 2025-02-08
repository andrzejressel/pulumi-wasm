#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EnterpriseKeyAndroidSettings {
    /// If set to true, it means allowed_package_names will not be enforced.
    #[builder(into, default)]
    #[serde(rename = "allowAllPackageNames")]
    pub r#allow_all_package_names: Box<Option<bool>>,
    /// Android package names of apps allowed to use the key. Example: 'com.companyname.appname'
    #[builder(into, default)]
    #[serde(rename = "allowedPackageNames")]
    pub r#allowed_package_names: Box<Option<Vec<String>>>,
}
