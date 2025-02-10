/// <!-- Note: This documentation is generated. Any manual changes will be overwritten -->
///
/// Manages a Dev Center.
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
///     let exampleDevCenter = dev_center::create(
///         "exampleDevCenter",
///         DevCenterArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
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
/// ## Blocks Reference
///
/// ### `identity` Block
///
///
/// The `identity` block supports the following arguments:
///
/// * `type` - (Required) Specifies the type of Managed Identity that should be assigned to this Dev Center. Possible values are `SystemAssigned`, `SystemAssigned, UserAssigned` and `UserAssigned`.
/// * `identity_ids` - (Optional) A list of the User Assigned Identity IDs that should be assigned to this Dev Center.
///
///
/// In addition to the arguments defined above, the `identity` block exports the following attributes:
///
/// * `principal_id` - The Principal ID for the System-Assigned Managed Identity assigned to this Dev Center.
/// * `tenant_id` - The Tenant ID for the System-Assigned Managed Identity assigned to this Dev Center.
///
/// ## Import
///
/// An existing Dev Center can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devcenter/devCenter:DevCenter example /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.DevCenter/devCenters/{devCenterName}
/// ```
///
/// * Where `{subscriptionId}` is the ID of the Azure Subscription where the Dev Center exists. For example `12345678-1234-9876-4563-123456789012`.
///
/// * Where `{resourceGroupName}` is the name of Resource Group where this Dev Center exists. For example `example-resource-group`.
///
/// * Where `{devCenterName}` is the name of the Dev Center. For example `devCenterValue`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dev_center {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DevCenterArgs {
        /// An `identity` block as defined below. Specifies the Managed Identity which should be assigned to this Dev Center.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::devcenter::DevCenterIdentity>,
        >,
        /// The Azure Region where the Dev Center should exist. Changing this forces a new Dev Center to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of this Dev Center. Changing this forces a new Dev Center to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group within which this Dev Center should exist. Changing this forces a new Dev Center to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Dev Center.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DevCenterResult {
        /// The URI of the Dev Center.
        pub dev_center_uri: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below. Specifies the Managed Identity which should be assigned to this Dev Center.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::devcenter::DevCenterIdentity>,
        >,
        /// The Azure Region where the Dev Center should exist. Changing this forces a new Dev Center to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Dev Center. Changing this forces a new Dev Center to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group within which this Dev Center should exist. Changing this forces a new Dev Center to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Dev Center.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DevCenterArgs,
    ) -> DevCenterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:devcenter/devCenter:DevCenter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
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
        DevCenterResult {
            dev_center_uri: o.get_field("devCenterUri"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
