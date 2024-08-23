pub struct NetworkArgs {
    pub attachable: pulumi_wasm_rust::Output<Option<bool>>,
    pub check_duplicate: pulumi_wasm_rust::Output<Option<bool>>,
    pub driver: pulumi_wasm_rust::Output<Option<String>>,
    pub ingress: pulumi_wasm_rust::Output<Option<bool>>,
    pub internal: pulumi_wasm_rust::Output<Option<bool>>,
    pub ipam_configs: pulumi_wasm_rust::Output<Option<Vec<crate::types::NetworkIpamConfig>>>,
    pub ipam_driver: pulumi_wasm_rust::Output<Option<String>>,
    pub ipam_options: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    pub ipv6: pulumi_wasm_rust::Output<Option<bool>>,
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::NetworkLabel>>>,
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    pub options: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
}

pub struct NetworkResult {
    pub attachable: pulumi_wasm_rust::Output<Option<bool>>,
    pub check_duplicate: pulumi_wasm_rust::Output<Option<bool>>,
    pub driver: pulumi_wasm_rust::Output<String>,
    pub ingress: pulumi_wasm_rust::Output<Option<bool>>,
    pub internal: pulumi_wasm_rust::Output<bool>,
    pub ipam_configs: pulumi_wasm_rust::Output<Vec<crate::types::NetworkIpamConfig>>,
    pub ipam_driver: pulumi_wasm_rust::Output<Option<String>>,
    pub ipam_options: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    pub ipv6: pulumi_wasm_rust::Output<Option<bool>>,
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::NetworkLabel>>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub options: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub scope: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: NetworkArgs) -> NetworkResult {
    let result = crate::bindings::pulumi::docker::network::invoke(
        name,
        &crate::bindings::pulumi::docker::network::Args {
            attachable: args.attachable.get_inner(),
            check_duplicate: args.check_duplicate.get_inner(),
            driver: args.driver.get_inner(),
            ingress: args.ingress.get_inner(),
            internal: args.internal.get_inner(),
            ipam_configs: args.ipam_configs.get_inner(),
            ipam_driver: args.ipam_driver.get_inner(),
            ipam_options: args.ipam_options.get_inner(),
            ipv6: args.ipv6.get_inner(),
            labels: args.labels.get_inner(),
            name: args.name.get_inner(),
            options: args.options.get_inner(),
        },
    );

    NetworkResult {
        attachable: crate::into_domain(result.attachable),
        check_duplicate: crate::into_domain(result.check_duplicate),
        driver: crate::into_domain(result.driver),
        ingress: crate::into_domain(result.ingress),
        internal: crate::into_domain(result.internal),
        ipam_configs: crate::into_domain(result.ipam_configs),
        ipam_driver: crate::into_domain(result.ipam_driver),
        ipam_options: crate::into_domain(result.ipam_options),
        ipv6: crate::into_domain(result.ipv6),
        labels: crate::into_domain(result.labels),
        name: crate::into_domain(result.name),
        options: crate::into_domain(result.options),
        scope: crate::into_domain(result.scope),
    }
}
