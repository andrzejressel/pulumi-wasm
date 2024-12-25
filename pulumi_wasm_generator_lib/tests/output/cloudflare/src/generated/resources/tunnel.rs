#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TunnelArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Indicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel on the Zero Trust dashboard or using tunnel*config, tunnel*route or tunnel*virtual*network resources. Available values: `local`, `cloudflare`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub config_src: pulumi_wasm_rust::Output<Option<String>>,
    /// A user-friendly name chosen when the tunnel is created. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// 32 or more bytes, encoded as a base64 string. The Create Argo Tunnel endpoint sets this as the tunnel's password. Anyone wishing to run the tunnel needs this password. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub secret: pulumi_wasm_rust::Output<String>,
}
pub struct TunnelResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Usable CNAME for accessing the Tunnel.
    pub cname: pulumi_wasm_rust::Output<String>,
    /// Indicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel on the Zero Trust dashboard or using tunnel*config, tunnel*route or tunnel*virtual*network resources. Available values: `local`, `cloudflare`. **Modifying this attribute will force creation of a new resource.**
    pub config_src: pulumi_wasm_rust::Output<Option<String>>,
    /// A user-friendly name chosen when the tunnel is created. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// 32 or more bytes, encoded as a base64 string. The Create Argo Tunnel endpoint sets this as the tunnel's password. Anyone wishing to run the tunnel needs this password. **Modifying this attribute will force creation of a new resource.**
    pub secret: pulumi_wasm_rust::Output<String>,
    /// Token used by a connector to authenticate and run the tunnel.
    pub tunnel_token: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: TunnelArgs) -> TunnelResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let config_src_binding = args.config_src.get_inner();
    let name_binding = args.name.get_inner();
    let secret_binding = args.secret.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/tunnel:Tunnel".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "configSrc".into(),
                value: &config_src_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "secret".into(),
                value: &secret_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "cname".into() },
            register_interface::ResultField { name : "configSrc".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "secret".into() },
            register_interface::ResultField { name : "tunnelToken".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    TunnelResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        cname: into_domain(hashmap.remove("cname").unwrap()),
        config_src: into_domain(hashmap.remove("configSrc").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        secret: into_domain(hashmap.remove("secret").unwrap()),
        tunnel_token: into_domain(hashmap.remove("tunnelToken").unwrap()),
    }
}
