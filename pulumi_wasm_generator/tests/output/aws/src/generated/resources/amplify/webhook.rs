/// Provides an Amplify Webhook resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = app::create("example", AppArgs::builder().name("app").build_struct());
///     let master = branch::create(
///         "master",
///         BranchArgs::builder()
///             .app_id("${example.id}")
///             .branch_name("master")
///             .build_struct(),
///     );
///     let masterWebhook = webhook::create(
///         "masterWebhook",
///         WebhookArgs::builder()
///             .app_id("${example.id}")
///             .branch_name("${master.branchName}")
///             .description("triggermaster")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amplify webhook using a webhook ID. For example:
///
/// ```sh
/// $ pulumi import aws:amplify/webhook:Webhook master a26b22a0-748b-4b57-b9a0-ae7e601fe4b1
/// ```
pub mod webhook {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebhookArgs {
        /// Unique ID for an Amplify app.
        #[builder(into)]
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// Name for a branch that is part of the Amplify app.
        #[builder(into)]
        pub branch_name: pulumi_wasm_rust::Output<String>,
        /// Description for a webhook.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct WebhookResult {
        /// Unique ID for an Amplify app.
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// ARN for the webhook.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name for a branch that is part of the Amplify app.
        pub branch_name: pulumi_wasm_rust::Output<String>,
        /// Description for a webhook.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// URL of the webhook.
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WebhookArgs) -> WebhookResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_id_binding = args.app_id.get_inner();
        let branch_name_binding = args.branch_name.get_inner();
        let description_binding = args.description.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:amplify/webhook:Webhook".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appId".into(),
                    value: &app_id_binding,
                },
                register_interface::ObjectField {
                    name: "branchName".into(),
                    value: &branch_name_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "branchName".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WebhookResult {
            app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            branch_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("branchName").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
