//! Manages Web3 hostnames for IPFS and Ethereum gateways.

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct Web3HostnameArgs {
    /// An optional description of the hostname.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// DNSLink value used if the target is ipfs.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub dnslink: pulumi_wasm_rust::Output<Option<String>>,
    /// The hostname that will point to the target gateway via CNAME.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Target gateway of the hostname.
    #[builder(into)]
    pub target: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct Web3HostnameResult {
    /// Creation time.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// An optional description of the hostname.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// DNSLink value used if the target is ipfs.
    pub dnslink: pulumi_wasm_rust::Output<Option<String>>,
    /// Last modification time.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// The hostname that will point to the target gateway via CNAME.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Status of the hostname's activation.
    pub status: pulumi_wasm_rust::Output<String>,
    /// Target gateway of the hostname.
    pub target: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: Web3HostnameArgs) -> Web3HostnameResult {

    let result = crate::bindings::pulumi::cloudflare::web3_hostname::invoke(name, &crate::bindings::pulumi::cloudflare::web3_hostname::Args {
        description: &args.description.get_inner(),
        dnslink: &args.dnslink.get_inner(),
        name: &args.name.get_inner(),
        target: &args.target.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    Web3HostnameResult {
        created_on: crate::into_domain(result.created_on),
        description: crate::into_domain(result.description),
        dnslink: crate::into_domain(result.dnslink),
        modified_on: crate::into_domain(result.modified_on),
        name: crate::into_domain(result.name),
        status: crate::into_domain(result.status),
        target: crate::into_domain(result.target),
        zone_id: crate::into_domain(result.zone_id),
    }
}
