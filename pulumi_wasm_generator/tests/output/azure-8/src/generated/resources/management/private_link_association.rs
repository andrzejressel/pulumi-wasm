/// Manages a Resource Management Private Link Association.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example
///       location: West Europe
///   examplePrivateLink:
///     type: azure:management:PrivateLink
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///   exampleRandomUuid:
///     type: random:RandomUuid
///     name: example
///   examplePrivateLinkAssociation:
///     type: azure:management:PrivateLinkAssociation
///     name: example
///     properties:
///       name: ${exampleRandomUuid.result}
///       managementGroupId: ${exampleAzurermManagementGroup.id}
///       resourceManagementPrivateLinkId: ${examplePrivateLink.id}
///       publicNetworkAccessEnabled: true
/// variables:
///   example:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   exampleGetGroup:
///     fn::invoke:
///       function: azure:management:getGroup
///       arguments:
///         name: ${example.tenantId}
/// ```
///
/// ## Import
///
/// An existing Private Link Association can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:management/privateLinkAssociation:PrivateLinkAssociation example /providers/Microsoft.Management/managementGroups/00000000-0000-0000-0000-000000000000/providers/Microsoft.Authorization/privateLinkAssociations/00000000-0000-0000-0000-000000000000
/// ```
///
pub mod private_link_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrivateLinkAssociationArgs {
        /// Specifies the Management Group ID within which this Private Link Association should exist. Changing this forces a new Private Link Association to be created.
        ///
        /// > **Note:** For now, `management_group_id` must be the ID of [Root Management Group](https://learn.microsoft.com/en-us/azure/governance/management-groups/overview#root-management-group-for-each-directory).
        #[builder(into)]
        pub management_group_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Private Link Association, which should be a UUID. If `name` is not provided, a UUID will be generated, you should use the `ignore_changes` attribute to ignore changes to this field. Changing this forces a new Private Link Association to be created.
        ///
        /// ```ignore
        /// use pulumi_wasm_rust::Output;
        /// use pulumi_wasm_rust::{add_export, pulumi_main};
        /// #[pulumi_main]
        /// fn test_main() -> Result<(), Error> {
        ///     let example = private_link_association::create(
        ///         "example",
        ///         PrivateLinkAssociationArgs::builder()
        ///             .management_group_id("${exampleAzurermManagementGroup.id}")
        ///             .public_network_access_enabled(true)
        ///             .resource_management_private_link_id(
        ///                 "${exampleAzurermResourceManagementPrivateLink.id}",
        ///             )
        ///             .build_struct(),
        ///     );
        /// }
        /// ```
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether public network access is allowed. Changing this forces a new Private Link Association to be created.
        #[builder(into)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Resource ID of Resource Management Private Link. Changing this forces a new Private Link Association to be created.
        #[builder(into)]
        pub resource_management_private_link_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PrivateLinkAssociationResult {
        /// Specifies the Management Group ID within which this Private Link Association should exist. Changing this forces a new Private Link Association to be created.
        ///
        /// > **Note:** For now, `management_group_id` must be the ID of [Root Management Group](https://learn.microsoft.com/en-us/azure/governance/management-groups/overview#root-management-group-for-each-directory).
        pub management_group_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Private Link Association, which should be a UUID. If `name` is not provided, a UUID will be generated, you should use the `ignore_changes` attribute to ignore changes to this field. Changing this forces a new Private Link Association to be created.
        ///
        /// ```ignore
        /// use pulumi_wasm_rust::Output;
        /// use pulumi_wasm_rust::{add_export, pulumi_main};
        /// #[pulumi_main]
        /// fn test_main() -> Result<(), Error> {
        ///     let example = private_link_association::create(
        ///         "example",
        ///         PrivateLinkAssociationArgs::builder()
        ///             .management_group_id("${exampleAzurermManagementGroup.id}")
        ///             .public_network_access_enabled(true)
        ///             .resource_management_private_link_id(
        ///                 "${exampleAzurermResourceManagementPrivateLink.id}",
        ///             )
        ///             .build_struct(),
        ///     );
        /// }
        /// ```
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether public network access is allowed. Changing this forces a new Private Link Association to be created.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Resource ID of Resource Management Private Link. Changing this forces a new Private Link Association to be created.
        pub resource_management_private_link_id: pulumi_wasm_rust::Output<String>,
        /// The Tenant ID.
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: PrivateLinkAssociationArgs,
    ) -> PrivateLinkAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let management_group_id_binding = args.management_group_id.get_inner();
        let name_binding = args.name.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let resource_management_private_link_id_binding = args
            .resource_management_private_link_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:management/privateLinkAssociation:PrivateLinkAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "managementGroupId".into(),
                    value: &management_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceManagementPrivateLinkId".into(),
                    value: &resource_management_private_link_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "managementGroupId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceManagementPrivateLinkId".into(),
                },
                register_interface::ResultField {
                    name: "tenantId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PrivateLinkAssociationResult {
            management_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementGroupId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            resource_management_private_link_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceManagementPrivateLinkId").unwrap(),
            ),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantId").unwrap(),
            ),
        }
    }
}
