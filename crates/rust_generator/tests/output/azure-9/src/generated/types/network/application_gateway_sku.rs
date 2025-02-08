#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationGatewaySku {
    /// The Capacity of the SKU to use for this Application Gateway. When using a V1 SKU this value must be between `1` and `32`, and `1` to `125` for a V2 SKU. When using a `Basic` SKU this property must be between `1` and `2`. This property is optional if `autoscale_configuration` is set.
    #[builder(into, default)]
    #[serde(rename = "capacity")]
    pub r#capacity: Box<Option<i32>>,
    /// The Name of the SKU to use for this Application Gateway. Possible values are `Basic`, `Standard_Small`, `Standard_Medium`, `Standard_Large`, `Basic`, `Standard_v2`, `WAF_Medium`, `WAF_Large`, and `WAF_v2`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Tier of the SKU to use for this Application Gateway. Possible values are `Basic`, `Standard`, `Standard_v2`, `WAF` and `WAF_v2`.
    /// 
    /// !> **NOTE:** The `Standard` and `WAF` SKU have been deprecated in favour of the `Basic`, `Standard_v2` and `WAF_v2` SKU. Please see the [Azure documentation](https://aka.ms/V1retirement) for more details.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: Box<String>,
}
