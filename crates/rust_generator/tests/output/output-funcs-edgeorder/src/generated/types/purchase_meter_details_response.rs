#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PurchaseMeterDetailsResponse {
    /// Represents billing type.
    /// Expected value is 'Purchase'.
    #[builder(skip)]
    #[serde(rename = "billingType")]
    r#billing_type: Box<super::constants::ConstStringPurchase>,
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
