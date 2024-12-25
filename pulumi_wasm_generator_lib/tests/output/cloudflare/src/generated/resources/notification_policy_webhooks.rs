#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct NotificationPolicyWebhooksArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the webhook destination.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// An optional secret can be provided that will be passed in the `cf-webhook-auth` header when dispatching a webhook notification. Secrets are not returned in any API response body. Refer to the [documentation](https://api.cloudflare.com/#notification-webhooks-create-webhook) for more details.
    #[builder(into, default)]
    pub secret: pulumi_wasm_rust::Output<Option<String>>,
    /// The URL of the webhook destinations. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub url: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct NotificationPolicyWebhooksResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Timestamp of when the notification webhook was created.
    pub created_at: pulumi_wasm_rust::Output<String>,
    /// Timestamp of when the notification webhook last failed.
    pub last_failure: pulumi_wasm_rust::Output<String>,
    /// Timestamp of when the notification webhook was last successful.
    pub last_success: pulumi_wasm_rust::Output<String>,
    /// The name of the webhook destination.
    pub name: pulumi_wasm_rust::Output<String>,
    /// An optional secret can be provided that will be passed in the `cf-webhook-auth` header when dispatching a webhook notification. Secrets are not returned in any API response body. Refer to the [documentation](https://api.cloudflare.com/#notification-webhooks-create-webhook) for more details.
    pub secret: pulumi_wasm_rust::Output<Option<String>>,
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The URL of the webhook destinations. **Modifying this attribute will force creation of a new resource.**
    pub url: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(
    name: &str,
    args: NotificationPolicyWebhooksArgs,
) -> NotificationPolicyWebhooksResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let name_binding = args.name.get_inner();
    let secret_binding = args.secret.get_inner();
    let url_binding = args.url.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/notificationPolicyWebhooks:NotificationPolicyWebhooks"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "secret".into(),
                value: &secret_binding,
            },
            register_interface::ObjectField {
                name: "url".into(),
                value: &url_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "createdAt".into() },
            register_interface::ResultField { name : "lastFailure".into() },
            register_interface::ResultField { name : "lastSuccess".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "secret".into() },
            register_interface::ResultField { name : "type".into() },
            register_interface::ResultField { name : "url".into() },
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
    NotificationPolicyWebhooksResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        created_at: into_domain(hashmap.remove("createdAt").unwrap()),
        last_failure: into_domain(hashmap.remove("lastFailure").unwrap()),
        last_success: into_domain(hashmap.remove("lastSuccess").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        secret: into_domain(hashmap.remove("secret").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
        url: into_domain(hashmap.remove("url").unwrap()),
    }
}
