#[allow(clippy::doc_lazy_continuation)]
pub mod get_lab {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLabArgs {
        /// The name of the Dev Test Lab.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the Dev Test Lab exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLabResult {
        /// The ID of the Storage Account used for Artifact Storage.
        pub artifacts_storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Default Premium Storage Account for this Dev Test Lab.
        pub default_premium_storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Default Storage Account for this Dev Test Lab.
        pub default_storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Key used for this Dev Test Lab.
        pub key_vault_id: pulumi_gestalt_rust::Output<String>,
        /// The Azure location where the Dev Test Lab exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Storage Account used for Storage of Premium Data Disk.
        pub premium_data_disk_storage_account_id: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The type of storage used by the Dev Test Lab.
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The unique immutable identifier of the Dev Test Lab.
        pub unique_identifier: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLabArgs,
    ) -> GetLabResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:devtest/getLab:getLab".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLabResult {
            artifacts_storage_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("artifactsStorageAccountId"),
            ),
            default_premium_storage_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultPremiumStorageAccountId"),
            ),
            default_storage_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultStorageAccountId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            key_vault_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultId"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            premium_data_disk_storage_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("premiumDataDiskStorageAccountId"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            storage_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageType"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            unique_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("uniqueIdentifier"),
            ),
        }
    }
}
