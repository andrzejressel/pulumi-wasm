//! Billing type Purchase meter details

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct PurchaseMeterDetailsResponse {
    /// Represents billing type.
    /// Expected value is 'Purchase'.
    #[builder(into, default)]
    #[serde(rename = "billingType")]
    pub r#billing_type: Box<crate::ConstString_Purchase>,
    /// Charging type.
    #[builder(into)]
    #[serde(rename = "chargingType")]
    pub r#charging_type: Box<String>,
    /// Billing unit applicable for Pav2 billing
    #[builder(into)]
    #[serde(rename = "multiplier")]
    pub r#multiplier: Box<f64>,
    /// Product Id
    #[builder(into)]
    #[serde(rename = "productId")]
    pub r#product_id: Box<String>,
    /// Sku Id
    #[builder(into)]
    #[serde(rename = "skuId")]
    pub r#sku_id: Box<String>,
    /// Term Id
    #[builder(into)]
    #[serde(rename = "termId")]
    pub r#term_id: Box<String>,
}
