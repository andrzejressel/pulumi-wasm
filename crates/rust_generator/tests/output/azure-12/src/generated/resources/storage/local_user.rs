/// Manages a Storage Account Local User.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod local_user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalUserArgs {
        /// The home directory of the Storage Account Local User.
        #[builder(into, default)]
        pub home_directory: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Storage Account Local User. Changing this forces a new Storage Account Local User to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `permission_scope` blocks as defined below.
        #[builder(into, default)]
        pub permission_scopes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::storage::LocalUserPermissionScope>>,
        >,
        /// One or more `ssh_authorized_key` blocks as defined below.
        #[builder(into, default)]
        pub ssh_authorized_keys: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::storage::LocalUserSshAuthorizedKey>>,
        >,
        /// Specifies whether SSH Key Authentication is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub ssh_key_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether SSH Password Authentication is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub ssh_password_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Storage Account that this Storage Account Local User resides in. Changing this forces a new Storage Account Local User to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LocalUserResult {
        /// The home directory of the Storage Account Local User.
        pub home_directory: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Storage Account Local User. Changing this forces a new Storage Account Local User to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The value of the password, which is only available when `ssh_password_enabled` is set to `true`.
        pub password: pulumi_gestalt_rust::Output<String>,
        /// One or more `permission_scope` blocks as defined below.
        pub permission_scopes: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::storage::LocalUserPermissionScope>>,
        >,
        /// The unique Security Identifier of this Storage Account Local User.
        pub sid: pulumi_gestalt_rust::Output<String>,
        /// One or more `ssh_authorized_key` blocks as defined below.
        pub ssh_authorized_keys: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::storage::LocalUserSshAuthorizedKey>>,
        >,
        /// Specifies whether SSH Key Authentication is enabled. Defaults to `false`.
        pub ssh_key_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether SSH Password Authentication is enabled. Defaults to `false`.
        pub ssh_password_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Storage Account that this Storage Account Local User resides in. Changing this forces a new Storage Account Local User to be created.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocalUserArgs,
    ) -> LocalUserResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let home_directory_binding = args.home_directory.get_output(context);
        let name_binding = args.name.get_output(context);
        let permission_scopes_binding = args.permission_scopes.get_output(context);
        let ssh_authorized_keys_binding = args.ssh_authorized_keys.get_output(context);
        let ssh_key_enabled_binding = args.ssh_key_enabled.get_output(context);
        let ssh_password_enabled_binding = args.ssh_password_enabled.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/localUser:LocalUser".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "homeDirectory".into(),
                    value: home_directory_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissionScopes".into(),
                    value: permission_scopes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sshAuthorizedKeys".into(),
                    value: ssh_authorized_keys_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sshKeyEnabled".into(),
                    value: ssh_key_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sshPasswordEnabled".into(),
                    value: ssh_password_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: storage_account_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LocalUserResult {
            home_directory: o.get_field("homeDirectory"),
            name: o.get_field("name"),
            password: o.get_field("password"),
            permission_scopes: o.get_field("permissionScopes"),
            sid: o.get_field("sid"),
            ssh_authorized_keys: o.get_field("sshAuthorizedKeys"),
            ssh_key_enabled: o.get_field("sshKeyEnabled"),
            ssh_password_enabled: o.get_field("sshPasswordEnabled"),
            storage_account_id: o.get_field("storageAccountId"),
        }
    }
}
