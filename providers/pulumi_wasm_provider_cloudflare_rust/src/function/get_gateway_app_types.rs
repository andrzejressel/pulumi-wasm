//! Use this data source to retrieve all Gateway application types for an account.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```yaml
//! variables:
//!   example:
//!     fn::invoke:
//!       Function: cloudflare:getGatewayAppTypes
//!       Arguments:
//!         accountId: f037e56e89293a057740de681ac9abbe
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetGatewayAppTypesArgs {
    /// The account ID to fetch Gateway App Types from.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
}

pub struct GetGatewayAppTypesResult {
    /// The account ID to fetch Gateway App Types from.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A list of Gateway App Types.
    pub app_types: pulumi_wasm_rust::Output<Vec<crate::types::GetGatewayAppTypesAppType>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetGatewayAppTypesArgs
) -> GetGatewayAppTypesResult {

    let result = crate::bindings::pulumi::cloudflare::get_gateway_app_types::invoke(
        &crate::bindings::pulumi::cloudflare::get_gateway_app_types::Args {
                account_id: &args.account_id.get_inner(),
        }
    );

    GetGatewayAppTypesResult {
        account_id: crate::into_domain(result.account_id),
        app_types: crate::into_domain(result.app_types),
        id: crate::into_domain(result.id),
    }
}
