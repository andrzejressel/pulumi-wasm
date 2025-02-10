pub mod functions {
    include!("functions/list_configurations.rs");
    include!("functions/list_product_families.rs");
}
pub mod types {
    include!("types/availability_information_response.rs");
    include!("types/billing_meter_details_response.rs");
    include!("types/configuration_filters.rs");
    include!("types/configuration_response.rs");
    include!("types/cost_information_response.rs");
    include!("types/customer_subscription_details.rs");
    include!("types/customer_subscription_registered_features.rs");
    include!("types/description_response.rs");
    include!("types/dimensions_response.rs");
    include!("types/filterable_property.rs");
    include!("types/filterable_property_response.rs");
    include!("types/hierarchy_information.rs");
    include!("types/hierarchy_information_response.rs");
    include!("types/image_information_response.rs");
    include!("types/link_response.rs");
    include!("types/pav_2_meter_details_response.rs");
    include!("types/product_family_response.rs");
    include!("types/product_line_response.rs");
    include!("types/product_response.rs");
    include!("types/purchase_meter_details_response.rs");
    include!("types/specification_response.rs");
    include!("types/supported_filter_types.rs");
}
#[doc(hidden)]
pub mod constants {
    pulumi_gestalt_rust::__private::constant::generate_string_const!(
        ConstStringPav2, "Pav2"
    );
    pulumi_gestalt_rust::__private::constant::generate_string_const!(
        ConstStringPurchase, "Purchase"
    );
}
#[link_section = "pulumi_gestalt_provider::myedgeorder"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_MYEDGEORDER: [u8; 44] = *b"{\"version\":\"0.0.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "0.0.1".to_string()
}
