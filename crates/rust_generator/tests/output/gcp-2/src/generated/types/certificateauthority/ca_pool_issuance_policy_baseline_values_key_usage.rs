#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CaPoolIssuancePolicyBaselineValuesKeyUsage {
    /// Describes high-level ways in which a key may be used.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "baseKeyUsage")]
    pub r#base_key_usage: Box<super::super::types::certificateauthority::CaPoolIssuancePolicyBaselineValuesKeyUsageBaseKeyUsage>,
    /// Describes high-level ways in which a key may be used.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "extendedKeyUsage")]
    pub r#extended_key_usage: Box<super::super::types::certificateauthority::CaPoolIssuancePolicyBaselineValuesKeyUsageExtendedKeyUsage>,
    /// An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "unknownExtendedKeyUsages")]
    pub r#unknown_extended_key_usages: Box<Option<Vec<super::super::types::certificateauthority::CaPoolIssuancePolicyBaselineValuesKeyUsageUnknownExtendedKeyUsage>>>,
}
