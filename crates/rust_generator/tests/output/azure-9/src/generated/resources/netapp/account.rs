/// Manages a NetApp Account.
///
/// > **NOTE:** Azure allows only one active directory can be joined to a single subscription at a time for NetApp Account.
///
/// ## NetApp Account Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleUserAssignedIdentity:
///     type: azure:authorization:UserAssignedIdentity
///     name: example
///     properties:
///       name: anf-user-assigned-identity
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleAccount:
///     type: azure:netapp:Account
///     name: example
///     properties:
///       name: netappaccount
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       activeDirectory:
///         username: aduser
///         password: aduserpwd
///         smbServerName: SMBSERVER
///         dnsServers:
///           - 1.2.3.4
///         domain: westcentralus.com
///         organizationalUnit: OU=FirstLevel
///       identity:
///         type: UserAssigned
///         identityIds:
///           - ${exampleUserAssignedIdentity.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// NetApp Accounts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:netapp/account:Account example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.NetApp/netAppAccounts/account1
/// ```
///
/// ~> **IMPORTANT:** When importing a NetApp account, the `active_directory.password` and `active_directory.server_root_ca_certificate` values *cannot* be retrieved from the Azure API and will need to be redeclared within the resource.
///
pub mod account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// A `active_directory` block as defined below.
        #[builder(into, default)]
        pub active_directory: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::netapp::AccountActiveDirectory>,
        >,
        /// The `identity` block where it is used when customer managed keys based encryption will be enabled as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::netapp::AccountIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the NetApp Account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group where the NetApp Account should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// A `active_directory` block as defined below.
        pub active_directory: pulumi_wasm_rust::Output<
            Option<super::super::types::netapp::AccountActiveDirectory>,
        >,
        /// The `identity` block where it is used when customer managed keys based encryption will be enabled as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::netapp::AccountIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the NetApp Account. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group where the NetApp Account should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
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
        args: AccountArgs,
    ) -> AccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let active_directory_binding = args
            .active_directory
            .get_output(context)
            .get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:netapp/account:Account".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "activeDirectory".into(),
                    value: &active_directory_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccountResult {
            active_directory: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("activeDirectory"),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
