/// Manages a Spring Cloud Storage.
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
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("GRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSpringCloudService = spring_cloud_service::create(
///         "exampleSpringCloudService",
///         SpringCloudServiceArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSpringCloudStorage = spring_cloud_storage::create(
///         "exampleSpringCloudStorage",
///         SpringCloudStorageArgs::builder()
///             .name("example")
///             .spring_cloud_service_id("${exampleSpringCloudService.id}")
///             .storage_account_key("${exampleAccount.primaryAccessKey}")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Spring Cloud Storages can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudStorage:SpringCloudStorage example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resourceGroup1/providers/Microsoft.AppPlatform/spring/service1/storages/storage1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_storage {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudStorageArgs {
        /// The name which should be used for this Spring Cloud Storage. Changing this forces a new Spring Cloud Storage to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Spring Cloud Service where the Spring Cloud Storage should exist. Changing this forces a new Spring Cloud Storage to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The access key of the Azure Storage Account.
        #[builder(into)]
        pub storage_account_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The account name of the Azure Storage Account.
        #[builder(into)]
        pub storage_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudStorageResult {
        /// The name which should be used for this Spring Cloud Storage. Changing this forces a new Spring Cloud Storage to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Spring Cloud Service where the Spring Cloud Storage should exist. Changing this forces a new Spring Cloud Storage to be created.
        pub spring_cloud_service_id: pulumi_gestalt_rust::Output<String>,
        /// The access key of the Azure Storage Account.
        pub storage_account_key: pulumi_gestalt_rust::Output<String>,
        /// The account name of the Azure Storage Account.
        pub storage_account_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudStorageArgs,
    ) -> SpringCloudStorageResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let spring_cloud_service_id_binding = args
            .spring_cloud_service_id
            .get_output(context);
        let storage_account_key_binding = args.storage_account_key.get_output(context);
        let storage_account_name_binding = args.storage_account_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudStorage:SpringCloudStorage".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudServiceId".into(),
                    value: spring_cloud_service_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountKey".into(),
                    value: storage_account_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountName".into(),
                    value: storage_account_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudStorageResult {
            name: o.get_field("name"),
            spring_cloud_service_id: o.get_field("springCloudServiceId"),
            storage_account_key: o.get_field("storageAccountKey"),
            storage_account_name: o.get_field("storageAccountName"),
        }
    }
}
