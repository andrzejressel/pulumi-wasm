//! Provides a resource to manage operation-level settings in API Shield Schema Validation 2.0.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = api_shield_operation::create(
//!         "example",
//!         ApiShieldOperationArgs::builder()
//!             .endpoint("/path")
//!             .host("api.example.com")
//!             .method("GET")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//!     let exampleApiShieldOperationSchemaValidationSettings = api_shield_operation_schema_validation_settings::create(
//!         "exampleApiShieldOperationSchemaValidationSettings",
//!         ApiShieldOperationSchemaValidationSettingsArgs::builder()
//!             .mitigationAction("block")
//!             .operationId("${example.id}")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ApiShieldOperationSchemaValidationSettingsArgs {
    /// The mitigation action to apply to this operation.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    /// Operation ID these settings should apply to. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub operation_id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ApiShieldOperationSchemaValidationSettingsResult {
    /// The mitigation action to apply to this operation.
    pub mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    /// Operation ID these settings should apply to. **Modifying this attribute will force creation of a new resource.**
    pub operation_id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ApiShieldOperationSchemaValidationSettingsArgs) -> ApiShieldOperationSchemaValidationSettingsResult {

    let result = crate::bindings::pulumi::cloudflare::api_shield_operation_schema_validation_settings::invoke(name, &crate::bindings::pulumi::cloudflare::api_shield_operation_schema_validation_settings::Args {
        mitigation_action: &args.mitigation_action.get_inner(),
        operation_id: &args.operation_id.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ApiShieldOperationSchemaValidationSettingsResult {
        mitigation_action: crate::into_domain(result.mitigation_action),
        operation_id: crate::into_domain(result.operation_id),
        zone_id: crate::into_domain(result.zone_id),
    }
}
