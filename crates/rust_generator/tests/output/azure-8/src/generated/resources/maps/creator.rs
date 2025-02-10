/// Manages an Azure Maps Creator.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:maps:Account
///     name: example
///     properties:
///       name: example-maps-account
///       resourceGroupName: ${example.name}
///       skuName: G2
///       tags:
///         environment: Test
///   exampleCreator:
///     type: azure:maps:Creator
///     name: example
///     properties:
///       name: example-maps-creator
///       mapsAccountId: ${exampleAccount.id}
///       location: ${example.location}
///       storageUnits: 1
///       tags:
///         environment: Test
/// ```
///
/// ## Import
///
/// An Azure Maps Creators can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:maps/creator:Creator example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Maps/accounts/account1/creators/creator1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod creator {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CreatorArgs {
        /// The Azure Region where the Azure Maps Creator should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Azure Maps Creator. Changing this forces a new resource to be created.
        #[builder(into)]
        pub maps_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Azure Maps Creator. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The storage units to be allocated. Integer values from 1 to 100, inclusive.
        #[builder(into)]
        pub storage_units: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A mapping of tags which should be assigned to the Azure Maps Creator.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CreatorResult {
        /// The Azure Region where the Azure Maps Creator should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Azure Maps Creator. Changing this forces a new resource to be created.
        pub maps_account_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Azure Maps Creator. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The storage units to be allocated. Integer values from 1 to 100, inclusive.
        pub storage_units: pulumi_gestalt_rust::Output<i32>,
        /// A mapping of tags which should be assigned to the Azure Maps Creator.
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
        args: CreatorArgs,
    ) -> CreatorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let maps_account_id_binding = args.maps_account_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let storage_units_binding = args.storage_units.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:maps/creator:Creator".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mapsAccountId".into(),
                    value: maps_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageUnits".into(),
                    value: storage_units_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CreatorResult {
            location: o.get_field("location"),
            maps_account_id: o.get_field("mapsAccountId"),
            name: o.get_field("name"),
            storage_units: o.get_field("storageUnits"),
            tags: o.get_field("tags"),
        }
    }
}
