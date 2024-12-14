//! Provides a resource to manage a schema in API Shield Schema Validation 2.0.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   petstoreSchema:
//!     type: cloudflare:ApiShieldSchema
//!     name: petstore_schema
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       name: myschema
//!       kind: openapi_v3
//!       validationEnabled: true # optional, default false
//!       source:
//!         fn::invoke:
//!           Function: std:file
//!           Arguments:
//!             input: ./schemas/petstore.json
//!           Return: result
//! ```

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ApiShieldSchemaArgs {
    /// Kind of schema. Defaults to `openapi_v3`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub kind: pulumi_wasm_rust::Output<Option<String>>,
    /// Name of the schema. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Schema file bytes. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub source: pulumi_wasm_rust::Output<String>,
    /// Flag whether schema is enabled for validation.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub validation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ApiShieldSchemaResult {
    /// Kind of schema. Defaults to `openapi_v3`. **Modifying this attribute will force creation of a new resource.**
    pub kind: pulumi_wasm_rust::Output<Option<String>>,
    /// Name of the schema. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// Schema file bytes. **Modifying this attribute will force creation of a new resource.**
    pub source: pulumi_wasm_rust::Output<String>,
    /// Flag whether schema is enabled for validation.
    pub validation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ApiShieldSchemaArgs) -> ApiShieldSchemaResult {

    let result = crate::bindings::pulumi::cloudflare::api_shield_schema::invoke(name, &crate::bindings::pulumi::cloudflare::api_shield_schema::Args {
        kind: &args.kind.get_inner(),
        name: &args.name.get_inner(),
        source: &args.source.get_inner(),
        validation_enabled: &args.validation_enabled.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ApiShieldSchemaResult {
        kind: crate::into_domain(result.kind),
        name: crate::into_domain(result.name),
        source: crate::into_domain(result.source),
        validation_enabled: crate::into_domain(result.validation_enabled),
        zone_id: crate::into_domain(result.zone_id),
    }
}
