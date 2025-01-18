/// Manages Azure Batch Application instance.
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
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("examplesa")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleAccount2 = account::create(
///         "exampleAccount2",
///         AccountArgs::builder()
///             .location("${example.location}")
///             .name("exampleba")
///             .pool_allocation_mode("BatchService")
///             .resource_group_name("${example.name}")
///             .storage_account_authentication_mode("StorageKeys")
///             .storage_account_id("${exampleAccount.id}")
///             .build_struct(),
///     );
///     let exampleApplication = application::create(
///         "exampleApplication",
///         ApplicationArgs::builder()
///             .account_name("${exampleAccount2.name}")
///             .name("example-batch-application")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Batch Applications can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:batch/application:Application example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-rg/providers/Microsoft.Batch/batchAccounts/exampleba/applications/example-batch-application
/// ```
///
pub mod application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// The name of the Batch account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// A value indicating whether packages within the application may be overwritten using the same version string. Defaults to `true`.
        #[builder(into, default)]
        pub allow_updates: pulumi_wasm_rust::Output<Option<bool>>,
        /// The package to use if a client requests the application but does not specify a version. This property can only be set to the name of an existing package.
        #[builder(into, default)]
        pub default_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name for the application.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the application. This must be unique within the account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group that contains the Batch account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// The name of the Batch account. Changing this forces a new resource to be created.
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// A value indicating whether packages within the application may be overwritten using the same version string. Defaults to `true`.
        pub allow_updates: pulumi_wasm_rust::Output<Option<bool>>,
        /// The package to use if a client requests the application but does not specify a version. This property can only be set to the name of an existing package.
        pub default_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name for the application.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the application. This must be unique within the account. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group that contains the Batch account. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApplicationArgs) -> ApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_inner();
        let allow_updates_binding = args.allow_updates.get_inner();
        let default_version_binding = args.default_version.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:batch/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "allowUpdates".into(),
                    value: &allow_updates_binding,
                },
                register_interface::ObjectField {
                    name: "defaultVersion".into(),
                    value: &default_version_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountName".into(),
                },
                register_interface::ResultField {
                    name: "allowUpdates".into(),
                },
                register_interface::ResultField {
                    name: "defaultVersion".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountName").unwrap(),
            ),
            allow_updates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowUpdates").unwrap(),
            ),
            default_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultVersion").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}
