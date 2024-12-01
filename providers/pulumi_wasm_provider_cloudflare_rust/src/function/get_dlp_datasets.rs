//! Use this data source to retrieve all DLP datasets for an account.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```yaml
//! variables:
//!   example:
//!     fn::invoke:
//!       Function: cloudflare:getDlpDatasets
//!       Arguments:
//!         accountId: f037e56e89293a057740de681ac9abbe
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetDlpDatasetsArgs {
    /// The account ID to fetch DLP Datasets from.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
}

pub struct GetDlpDatasetsResult {
    /// The account ID to fetch DLP Datasets from.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A list of DLP Datasets.
    pub datasets: pulumi_wasm_rust::Output<Vec<crate::types::GetDlpDatasetsDataset>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetDlpDatasetsArgs
) -> GetDlpDatasetsResult {

    let result = crate::bindings::pulumi::cloudflare::get_dlp_datasets::invoke(
        &crate::bindings::pulumi::cloudflare::get_dlp_datasets::Args {
                account_id: &args.account_id.get_inner(),
        }
    );

    GetDlpDatasetsResult {
        account_id: crate::into_domain(result.account_id),
        datasets: crate::into_domain(result.datasets),
        id: crate::into_domain(result.id),
    }
}
