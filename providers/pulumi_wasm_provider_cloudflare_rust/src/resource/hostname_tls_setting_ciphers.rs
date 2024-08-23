pub struct HostnameTlsSettingCiphersArgs {
    pub hostname: pulumi_wasm_rust::Output<String>,
    pub ports: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
    pub values: pulumi_wasm_rust::Output<Vec<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct HostnameTlsSettingCiphersResult {
    pub created_at: pulumi_wasm_rust::Output<String>,
    pub hostname: pulumi_wasm_rust::Output<String>,
    pub ports: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
    pub updated_at: pulumi_wasm_rust::Output<String>,
    pub values: pulumi_wasm_rust::Output<Vec<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: HostnameTlsSettingCiphersArgs) -> HostnameTlsSettingCiphersResult {
    let result = crate::bindings::pulumi::cloudflare::hostname_tls_setting_ciphers::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::hostname_tls_setting_ciphers::Args {
            hostname: args.hostname.get_inner(),
            ports: args.ports.get_inner(),
            values: args.values.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    HostnameTlsSettingCiphersResult {
        created_at: crate::into_domain(result.created_at),
        hostname: crate::into_domain(result.hostname),
        ports: crate::into_domain(result.ports),
        updated_at: crate::into_domain(result.updated_at),
        values: crate::into_domain(result.values),
        zone_id: crate::into_domain(result.zone_id),
    }
}
