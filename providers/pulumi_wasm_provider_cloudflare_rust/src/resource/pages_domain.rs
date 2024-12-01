//! Provides a resource for managing Cloudflare Pages domains.
//! 
//! > A DNS record for the domain is not automatically created. You need to create
//!    a `cloudflare.Record` resource for the domain you want to use.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```yaml
//! resources:
//!   my-domain:
//!     type: cloudflare:PagesDomain
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       projectName: my-example-project
//!       domain: example.com
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/pagesDomain:PagesDomain example <account_id>/<project_name>/<domain-name>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct PagesDomainArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Custom domain. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub domain: pulumi_wasm_rust::Output<String>,
    /// Name of the Pages Project. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub project_name: pulumi_wasm_rust::Output<String>,
}

pub struct PagesDomainResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Custom domain. **Modifying this attribute will force creation of a new resource.**
    pub domain: pulumi_wasm_rust::Output<String>,
    /// Name of the Pages Project. **Modifying this attribute will force creation of a new resource.**
    pub project_name: pulumi_wasm_rust::Output<String>,
    /// Status of the custom domain.
    pub status: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: PagesDomainArgs) -> PagesDomainResult {

    let result = crate::bindings::pulumi::cloudflare::pages_domain::invoke(name, &crate::bindings::pulumi::cloudflare::pages_domain::Args {
        account_id: &args.account_id.get_inner(),
        domain: &args.domain.get_inner(),
        project_name: &args.project_name.get_inner(),
    });

    PagesDomainResult {
        account_id: crate::into_domain(result.account_id),
        domain: crate::into_domain(result.domain),
        project_name: crate::into_domain(result.project_name),
        status: crate::into_domain(result.status),
    }
}
