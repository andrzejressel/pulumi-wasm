/// Manages a Automation Powershell 7.2 Module.
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
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .location("${example.location}")
///             .name("account1")
///             .resource_group_name("${example.name}")
///             .sku_name("Basic")
///             .build_struct(),
///     );
///     let examplePowershell72Module = powershell_72_module::create(
///         "examplePowershell72Module",
///         Powershell72ModuleArgs::builder()
///             .automation_account_id("${exampleAccount.id}")
///             .module_link(
///                 Powershell72ModuleModuleLink::builder()
///                     .uri(
///                         "https://devopsgallerystorage.blob.core.windows.net/packages/xactivedirectory.2.19.0.nupkg",
///                     )
///                     .build_struct(),
///             )
///             .name("xActiveDirectory")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Automation Modules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/powershell72Module:Powershell72Module module1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/powerShell72Modules/module1
/// ```
///
pub mod powershell_72_module {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Powershell72ModuleArgs {
        /// The ID of Automation Account to manage this Watcher. Changing this forces a new Watcher to be created.
        #[builder(into)]
        pub automation_account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `module_link` block as defined below.
        #[builder(into)]
        pub module_link: pulumi_wasm_rust::InputOrOutput<
            super::super::types::automation::Powershell72ModuleModuleLink,
        >,
        /// Specifies the name of the Module. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct Powershell72ModuleResult {
        /// The ID of Automation Account to manage this Watcher. Changing this forces a new Watcher to be created.
        pub automation_account_id: pulumi_wasm_rust::Output<String>,
        /// A `module_link` block as defined below.
        pub module_link: pulumi_wasm_rust::Output<
            super::super::types::automation::Powershell72ModuleModuleLink,
        >,
        /// Specifies the name of the Module. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: Powershell72ModuleArgs,
    ) -> Powershell72ModuleResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automation_account_id_binding = args
            .automation_account_id
            .get_output(context)
            .get_inner();
        let module_link_binding = args.module_link.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/powershell72Module:Powershell72Module".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountId".into(),
                    value: &automation_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "moduleLink".into(),
                    value: &module_link_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        Powershell72ModuleResult {
            automation_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("automationAccountId"),
            ),
            module_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("moduleLink"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
