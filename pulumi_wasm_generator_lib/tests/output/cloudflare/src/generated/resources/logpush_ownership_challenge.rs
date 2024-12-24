#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct LogpushOwnershipChallengeArgs {
    /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Uniquely identifies a resource (such as an s3 bucket) where data will be pushed. Additional configuration parameters supported by the destination may be included. See [Logpush destination documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#destination). **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub destination_conf: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct LogpushOwnershipChallengeResult {
    /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Uniquely identifies a resource (such as an s3 bucket) where data will be pushed. Additional configuration parameters supported by the destination may be included. See [Logpush destination documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#destination). **Modifying this attribute will force creation of a new resource.**
    pub destination_conf: pulumi_wasm_rust::Output<String>,
    /// The filename of the ownership challenge which	contains the contents required for Logpush Job creation.
    pub ownership_challenge_filename: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: LogpushOwnershipChallengeArgs,
) -> LogpushOwnershipChallengeResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let destination_conf_binding = args.destination_conf.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/logpushOwnershipChallenge:LogpushOwnershipChallenge"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "destinationConf".into(),
                value: &destination_conf_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "destinationConf".into() },
            register_interface::ResultField { name : "ownershipChallengeFilename".into()
            }, register_interface::ResultField { name : "zoneId".into() },
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
    LogpushOwnershipChallengeResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        destination_conf: into_domain(hashmap.remove("destinationConf").unwrap()),
        ownership_challenge_filename: into_domain(
            hashmap.remove("ownershipChallengeFilename").unwrap(),
        ),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
