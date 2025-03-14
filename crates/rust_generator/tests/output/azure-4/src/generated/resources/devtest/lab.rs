/// Manages a Dev Test Lab.
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
///   exampleLab:
///     type: azure:devtest:Lab
///     name: example
///     properties:
///       name: example-devtestlab
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tags:
///         Sydney: Australia
/// ```
///
/// ## Import
///
/// Dev Test Labs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devtest/lab:Lab lab1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DevTestLab/labs/lab1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lab {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LabArgs {
        /// Specifies the supported Azure location where the Dev Test Lab should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Dev Test Lab. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group under which the Dev Test Lab resource has to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LabResult {
        /// The ID of the Storage Account used for Artifact Storage.
        pub artifacts_storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Default Premium Storage Account for this Dev Test Lab.
        pub default_premium_storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Default Storage Account for this Dev Test Lab.
        pub default_storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Key used for this Dev Test Lab.
        pub key_vault_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the Dev Test Lab should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Dev Test Lab. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Storage Account used for Storage of Premium Data Disk.
        pub premium_data_disk_storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group under which the Dev Test Lab resource has to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The unique immutable identifier of the Dev Test Lab.
        pub unique_identifier: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LabArgs,
    ) -> LabResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:devtest/lab:Lab".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LabResult {
            artifacts_storage_account_id: o.get_field("artifactsStorageAccountId"),
            default_premium_storage_account_id: o
                .get_field("defaultPremiumStorageAccountId"),
            default_storage_account_id: o.get_field("defaultStorageAccountId"),
            key_vault_id: o.get_field("keyVaultId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            premium_data_disk_storage_account_id: o
                .get_field("premiumDataDiskStorageAccountId"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            unique_identifier: o.get_field("uniqueIdentifier"),
        }
    }
}
