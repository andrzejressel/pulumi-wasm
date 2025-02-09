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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod private_link_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrivateLinkAssociationArgs {
        /// Specifies the Management Group ID within which this Private Link Association should exist. Changing this forces a new Private Link Association to be created.
        ///
        /// > **Note:** For now, `management_group_id` must be the ID of [Root Management Group](https://learn.microsoft.com/en-us/azure/governance/management-groups/overview#root-management-group-for-each-directory).
        #[builder(into)]
        pub management_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of this Private Link Association, which should be a UUID. If `name` is not provided, a UUID will be generated, you should use the `ignore_changes` attribute to ignore changes to this field. Changing this forces a new Private Link Association to be created.
        ///
        /// ```ignore
        /// use pulumi_gestalt_rust::Output;
        /// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether public network access is allowed. Changing this forces a new Private Link Association to be created.
        #[builder(into)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The Resource ID of Resource Management Private Link. Changing this forces a new Private Link Association to be created.
        #[builder(into)]
        pub resource_management_private_link_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
    }
    #[allow(dead_code)]
    pub struct PrivateLinkAssociationResult {
        /// Specifies the Management Group ID within which this Private Link Association should exist. Changing this forces a new Private Link Association to be created.
        ///
        /// > **Note:** For now, `management_group_id` must be the ID of [Root Management Group](https://learn.microsoft.com/en-us/azure/governance/management-groups/overview#root-management-group-for-each-directory).
        pub management_group_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Private Link Association, which should be a UUID. If `name` is not provided, a UUID will be generated, you should use the `ignore_changes` attribute to ignore changes to this field. Changing this forces a new Private Link Association to be created.
        ///
        /// ```ignore
        /// use pulumi_gestalt_rust::Output;
        /// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether public network access is allowed. Changing this forces a new Private Link Association to be created.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Resource ID of Resource Management Private Link. Changing this forces a new Private Link Association to be created.
        pub resource_management_private_link_id: pulumi_gestalt_rust::Output<String>,
        /// The Tenant ID.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrivateLinkAssociationArgs,
    ) -> PrivateLinkAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let management_group_id_binding = args.management_group_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let resource_management_private_link_id_binding = args
            .resource_management_private_link_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:management/privateLinkAssociation:PrivateLinkAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managementGroupId".into(),
                    value: management_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: public_network_access_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceManagementPrivateLinkId".into(),
                    value: resource_management_private_link_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PrivateLinkAssociationResult {
            management_group_id: o.get_field("managementGroupId"),
            name: o.get_field("name"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_management_private_link_id: o
                .get_field("resourceManagementPrivateLinkId"),
            tenant_id: o.get_field("tenantId"),
        }
    }
}
