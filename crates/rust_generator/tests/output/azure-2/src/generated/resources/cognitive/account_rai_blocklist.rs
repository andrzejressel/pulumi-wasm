/// Manages a Cognitive Account Rai Blocklist.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account_rai_blocklist {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountRaiBlocklistArgs {
        /// The ID of the Cognitive Services Account. Changing this forces a new Cognitive Account Rai Blocklist to be created.
        #[builder(into)]
        pub cognitive_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A short description for the Cognitive Account Rai Blocklist.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Cognitive Account Rai Blocklist. Changing this forces a new Cognitive Account Rai Blocklist to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountRaiBlocklistResult {
        /// The ID of the Cognitive Services Account. Changing this forces a new Cognitive Account Rai Blocklist to be created.
        pub cognitive_account_id: pulumi_gestalt_rust::Output<String>,
        /// A short description for the Cognitive Account Rai Blocklist.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Cognitive Account Rai Blocklist. Changing this forces a new Cognitive Account Rai Blocklist to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AccountRaiBlocklistArgs,
    ) -> AccountRaiBlocklistResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cognitive_account_id_binding = args
            .cognitive_account_id
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cognitive/accountRaiBlocklist:AccountRaiBlocklist".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccountRaiBlocklistResult {
            cognitive_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cognitiveAccountId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
