#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustKeyAccessKeyConfigurationArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Number of days to trigger a rotation of the keys.
    #[builder(into, default)]
    pub key_rotation_interval_days: pulumi_wasm_rust::Output<Option<i32>>,
}
pub struct ZeroTrustKeyAccessKeyConfigurationResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Number of days to trigger a rotation of the keys.
    pub key_rotation_interval_days: pulumi_wasm_rust::Output<i32>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(
    name: &str,
    args: ZeroTrustKeyAccessKeyConfigurationArgs,
) -> ZeroTrustKeyAccessKeyConfigurationResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let key_rotation_interval_days_binding = args.key_rotation_interval_days.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/zeroTrustKeyAccessKeyConfiguration:ZeroTrustKeyAccessKeyConfiguration"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "keyRotationIntervalDays".into(),
                value: &key_rotation_interval_days_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "keyRotationIntervalDays".into() },
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
    ZeroTrustKeyAccessKeyConfigurationResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        key_rotation_interval_days: into_domain(
            hashmap.remove("keyRotationIntervalDays").unwrap(),
        ),
    }
}
