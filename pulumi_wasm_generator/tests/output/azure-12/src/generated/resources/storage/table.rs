/// Manages a Table within an Azure Storage Account.
///
/// > **Note on Authentication** Shared Key authentication will always be used for this resource, as AzureAD authentication is not supported when setting or retrieving ACLs for Tables.
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
///             .name("azuretest")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("azureteststorage1")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleTable = table::create(
///         "exampleTable",
///         TableArgs::builder()
///             .name("mysampletable")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Table's within a Storage Account can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/table:Table table1 "https://example.table.core.windows.net/Tables('replace-with-table-name')"
/// ```
///
pub mod table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableArgs {
        /// One or more `acl` blocks as defined below.
        #[builder(into, default)]
        pub acls: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::storage::TableAcl>>,
        >,
        /// The name of the storage table. Only Alphanumeric characters allowed, starting with a letter. Must be unique within the storage account the table is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the storage account in which to create the storage table. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TableResult {
        /// One or more `acl` blocks as defined below.
        pub acls: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::storage::TableAcl>>,
        >,
        /// The name of the storage table. Only Alphanumeric characters allowed, starting with a letter. Must be unique within the storage account the table is located. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the storage account in which to create the storage table. Changing this forces a new resource to be created.
        pub storage_account_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TableArgs) -> TableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let acls_binding = args.acls.get_inner();
        let name_binding = args.name.get_inner();
        let storage_account_name_binding = args.storage_account_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/table:Table".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acls".into(),
                    value: &acls_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "acls".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TableResult {
            acls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acls").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            storage_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountName").unwrap(),
            ),
        }
    }
}
