/// Manages a Cognitive Account Rai Blocklist.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("Brazil South")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .kind("OpenAI")
///             .location("${example.location}")
///             .name("example-ca")
///             .resource_group_name("${example.name}")
///             .sku_name("S0")
///             .build_struct(),
///     );
///     let exampleAccountRaiBlocklist = account_rai_blocklist::create(
///         "exampleAccountRaiBlocklist",
///         AccountRaiBlocklistArgs::builder()
///             .cognitive_account_id("${exampleAccount.id}")
///             .description("Azure OpenAI Rai Blocklist")
///             .name("example-crb")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Cognitive Account Rai Blocklist can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cognitive/accountRaiBlocklist:AccountRaiBlocklist example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.CognitiveServices/accounts/account1/raiBlocklists/raiblocklist1
/// ```
///
pub mod account_rai_blocklist {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountRaiBlocklistArgs {
        /// The ID of the Cognitive Services Account. Changing this forces a new Cognitive Account Rai Blocklist to be created.
        #[builder(into)]
        pub cognitive_account_id: pulumi_wasm_rust::Output<String>,
        /// A short description for the Cognitive Account Rai Blocklist.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Cognitive Account Rai Blocklist. Changing this forces a new Cognitive Account Rai Blocklist to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountRaiBlocklistResult {
        /// The ID of the Cognitive Services Account. Changing this forces a new Cognitive Account Rai Blocklist to be created.
        pub cognitive_account_id: pulumi_wasm_rust::Output<String>,
        /// A short description for the Cognitive Account Rai Blocklist.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Cognitive Account Rai Blocklist. Changing this forces a new Cognitive Account Rai Blocklist to be created.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AccountRaiBlocklistArgs,
    ) -> AccountRaiBlocklistResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cognitive_account_id_binding = args.cognitive_account_id.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cognitive/accountRaiBlocklist:AccountRaiBlocklist".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cognitiveAccountId".into(),
                    value: &cognitive_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cognitiveAccountId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountRaiBlocklistResult {
            cognitive_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cognitiveAccountId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}