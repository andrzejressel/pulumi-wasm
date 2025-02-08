#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GuestPoliciesPackageRepository {
    /// An Apt Repository.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "apt")]
    pub r#apt: Box<Option<super::super::types::osconfig::GuestPoliciesPackageRepositoryApt>>,
    /// A Goo Repository.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "goo")]
    pub r#goo: Box<Option<super::super::types::osconfig::GuestPoliciesPackageRepositoryGoo>>,
    /// A Yum Repository.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "yum")]
    pub r#yum: Box<Option<super::super::types::osconfig::GuestPoliciesPackageRepositoryYum>>,
    /// A Zypper Repository.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "zypper")]
    pub r#zypper: Box<Option<super::super::types::osconfig::GuestPoliciesPackageRepositoryZypper>>,
}
