pub struct Web3HostnameArgs {
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub dnslink: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub target: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct Web3HostnameResult {
    pub created_on: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub dnslink: pulumi_wasm_rust::Output<Option<String>>,
    pub modified_on: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub status: pulumi_wasm_rust::Output<String>,
    pub target: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: Web3HostnameArgs) -> Web3HostnameResult {
    let result = crate::bindings::pulumi::cloudflare::web3_hostname::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::web3_hostname::Args {
            description: args.description.get_inner(),
            dnslink: args.dnslink.get_inner(),
            name: args.name.get_inner(),
            target: args.target.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

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
