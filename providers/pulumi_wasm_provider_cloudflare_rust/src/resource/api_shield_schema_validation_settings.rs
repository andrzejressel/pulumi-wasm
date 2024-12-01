//! Provides a resource to manage settings in API Shield Schema Validation 2.0.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = api_shield_schema_validation_settings::create(
//!         "example",
//!         ApiShieldSchemaValidationSettingsArgs::builder()
//!             .validationDefaultMitigationAction("log")
//!             .validationOverrideMitigationAction("none")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ApiShieldSchemaValidationSettingsArgs {
    /// The default mitigation action used when there is no mitigation action defined on the operation.
    #[builder(into)]
    pub validation_default_mitigation_action: pulumi_wasm_rust::Output<String>,
    /// When set, this overrides both zone level and operation level mitigation actions.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub validation_override_mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ApiShieldSchemaValidationSettingsResult {
    /// The default mitigation action used when there is no mitigation action defined on the operation.
    pub validation_default_mitigation_action: pulumi_wasm_rust::Output<String>,
    /// When set, this overrides both zone level and operation level mitigation actions.
    pub validation_override_mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ApiShieldSchemaValidationSettingsArgs) -> ApiShieldSchemaValidationSettingsResult {

    let result = crate::bindings::pulumi::cloudflare::api_shield_schema_validation_settings::invoke(name, &crate::bindings::pulumi::cloudflare::api_shield_schema_validation_settings::Args {
        validation_default_mitigation_action: &args.validation_default_mitigation_action.get_inner(),
        validation_override_mitigation_action: &args.validation_override_mitigation_action.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ApiShieldSchemaValidationSettingsResult {
        validation_default_mitigation_action: crate::into_domain(result.validation_default_mitigation_action),
        validation_override_mitigation_action: crate::into_domain(result.validation_override_mitigation_action),
        zone_id: crate::into_domain(result.zone_id),
    }
}
