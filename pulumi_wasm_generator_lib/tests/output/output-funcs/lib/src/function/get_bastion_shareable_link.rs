//! Response for all the Bastion Shareable Link endpoints.
//! API Version: 2020-11-01.

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetBastionShareableLinkArgs {
    /// The name of the Bastion Host.
    #[builder(into)]
    pub bastion_host_name: pulumi_wasm_rust::Output<String>,
    /// The name of the resource group.
    #[builder(into)]
    pub resource_group_name: pulumi_wasm_rust::Output<String>,
    /// List of VM references.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub vms: pulumi_wasm_rust::Output<Option<Vec<crate::types::BastionShareableLink>>>,
}

pub struct GetBastionShareableLinkResult {
    /// The URL to get the next set of results.
    pub next_link: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetBastionShareableLinkArgs
) -> GetBastionShareableLinkResult {

    let result = crate::bindings::pulumi::mypkg::get_bastion_shareable_link::invoke(
        &crate::bindings::pulumi::mypkg::get_bastion_shareable_link::Args {
                bastion_host_name: &args.bastion_host_name.get_inner(),
                resource_group_name: &args.resource_group_name.get_inner(),
                vms: &args.vms.get_inner(),
        }
    );

    GetBastionShareableLinkResult {
        next_link: crate::into_domain(result.next_link),
    }
}
