/// Manages a Storage Account Local User.
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
///             .location("WestEurope")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_kind("StorageV2")
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .is_hns_enabled(true)
///             .location("${example.location}")
///             .name("example-account")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleContainer = container::create(
///         "exampleContainer",
///         ContainerArgs::builder()
///             .name("example-container")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleLocalUser = local_user::create(
///         "exampleLocalUser",
///         LocalUserArgs::builder()
///             .home_directory("example_path")
///             .name("user1")
///             .permission_scopes(
///                 vec![
///                     LocalUserPermissionScope::builder()
///                     .permissions(LocalUserPermissionScopePermissions::builder()
///                     .create(true).read(true).build_struct())
///                     .resourceName("${exampleContainer.name}").service("blob")
///                     .build_struct(),
///                 ],
///             )
///             .ssh_authorized_keys(
///                 vec![
///                     LocalUserSshAuthorizedKey::builder().description("key1")
///                     .key("${firstPublicKey}").build_struct(),
///                     LocalUserSshAuthorizedKey::builder().description("key2")
///                     .key("${secondPublicKey}").build_struct(),
///                 ],
///             )
///             .ssh_key_enabled(true)
///             .ssh_password_enabled(true)
///             .storage_account_id("${exampleAccount.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Account Local Users can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/localUser:LocalUser example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Storage/storageAccounts/storageAccount1/localUsers/user1
/// ```
///
pub mod local_user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalUserArgs {
        /// The home directory of the Storage Account Local User.
        #[builder(into, default)]
        pub home_directory: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Storage Account Local User. Changing this forces a new Storage Account Local User to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `permission_scope` blocks as defined below.
        #[builder(into, default)]
        pub permission_scopes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::storage::LocalUserPermissionScope>>,
        >,
        /// One or more `ssh_authorized_key` blocks as defined below.
        #[builder(into, default)]
        pub ssh_authorized_keys: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::storage::LocalUserSshAuthorizedKey>>,
        >,
        /// Specifies whether SSH Key Authentication is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub ssh_key_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether SSH Password Authentication is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub ssh_password_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Storage Account that this Storage Account Local User resides in. Changing this forces a new Storage Account Local User to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LocalUserResult {
        /// The home directory of the Storage Account Local User.
        pub home_directory: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Storage Account Local User. Changing this forces a new Storage Account Local User to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The value of the password, which is only available when `ssh_password_enabled` is set to `true`.
        pub password: pulumi_wasm_rust::Output<String>,
        /// One or more `permission_scope` blocks as defined below.
        pub permission_scopes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::storage::LocalUserPermissionScope>>,
        >,
        /// The unique Security Identifier of this Storage Account Local User.
        pub sid: pulumi_wasm_rust::Output<String>,
        /// One or more `ssh_authorized_key` blocks as defined below.
        pub ssh_authorized_keys: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::storage::LocalUserSshAuthorizedKey>>,
        >,
        /// Specifies whether SSH Key Authentication is enabled. Defaults to `false`.
        pub ssh_key_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether SSH Password Authentication is enabled. Defaults to `false`.
        pub ssh_password_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Storage Account that this Storage Account Local User resides in. Changing this forces a new Storage Account Local User to be created.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LocalUserArgs) -> LocalUserResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let home_directory_binding = args.home_directory.get_inner();
        let name_binding = args.name.get_inner();
        let permission_scopes_binding = args.permission_scopes.get_inner();
        let ssh_authorized_keys_binding = args.ssh_authorized_keys.get_inner();
        let ssh_key_enabled_binding = args.ssh_key_enabled.get_inner();
        let ssh_password_enabled_binding = args.ssh_password_enabled.get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/localUser:LocalUser".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "homeDirectory".into(),
                    value: &home_directory_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "permissionScopes".into(),
                    value: &permission_scopes_binding,
                },
                register_interface::ObjectField {
                    name: "sshAuthorizedKeys".into(),
                    value: &ssh_authorized_keys_binding,
                },
                register_interface::ObjectField {
                    name: "sshKeyEnabled".into(),
                    value: &ssh_key_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "sshPasswordEnabled".into(),
                    value: &ssh_password_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "homeDirectory".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
                register_interface::ResultField {
                    name: "permissionScopes".into(),
                },
                register_interface::ResultField {
                    name: "sid".into(),
                },
                register_interface::ResultField {
                    name: "sshAuthorizedKeys".into(),
                },
                register_interface::ResultField {
                    name: "sshKeyEnabled".into(),
                },
                register_interface::ResultField {
                    name: "sshPasswordEnabled".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LocalUserResult {
            home_directory: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("homeDirectory").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
            permission_scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissionScopes").unwrap(),
            ),
            sid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sid").unwrap(),
            ),
            ssh_authorized_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sshAuthorizedKeys").unwrap(),
            ),
            ssh_key_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sshKeyEnabled").unwrap(),
            ),
            ssh_password_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sshPasswordEnabled").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
        }
    }
}
