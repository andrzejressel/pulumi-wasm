/// <!-- Note: This documentation is generated. Any manual changes will be overwritten -->
///
/// Manages a User Assigned Identity.
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
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleUserAssignedIdentity = user_assigned_identity::create(
///         "exampleUserAssignedIdentity",
///         UserAssignedIdentityArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// An existing User Assigned Identity can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:msi/userAssignedIdentity:UserAssignedIdentity example /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{userAssignedIdentityName}
/// ```
///
/// * Where `{subscriptionId}` is the ID of the Azure Subscription where the User Assigned Identity exists. For example `12345678-1234-9876-4563-123456789012`.
///
/// * Where `{resourceGroupName}` is the name of Resource Group where this User Assigned Identity exists. For example `example-resource-group`.
///
/// * Where `{userAssignedIdentityName}` is the name of the User Assigned Identity. For example `userAssignedIdentityValue`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_assigned_identity {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserAssignedIdentityArgs {
        /// The Azure Region where the User Assigned Identity should exist. Changing this forces a new User Assigned Identity to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of this User Assigned Identity. Changing this forces a new User Assigned Identity to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group within which this User Assigned Identity should exist. Changing this forces a new User Assigned Identity to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the User Assigned Identity.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct UserAssignedIdentityResult {
        /// The ID of the app associated with the Identity.
        pub client_id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the User Assigned Identity should exist. Changing this forces a new User Assigned Identity to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this User Assigned Identity. Changing this forces a new User Assigned Identity to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Service Principal object associated with the created Identity.
        pub principal_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group within which this User Assigned Identity should exist. Changing this forces a new User Assigned Identity to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the User Assigned Identity.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Tenant which the Identity belongs to.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserAssignedIdentityArgs,
    ) -> UserAssignedIdentityResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:msi/userAssignedIdentity:UserAssignedIdentity".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserAssignedIdentityResult {
            client_id: o.get_field("clientId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            principal_id: o.get_field("principalId"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            tenant_id: o.get_field("tenantId"),
        }
    }
}
