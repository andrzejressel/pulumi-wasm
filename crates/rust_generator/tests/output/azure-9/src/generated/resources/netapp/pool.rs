/// Manages a Pool within a NetApp Account.
///
/// ## NetApp Pool Usage
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
///             .location("${example.location}")
///             .name("example-netappaccount")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePool = pool::create(
///         "examplePool",
///         PoolArgs::builder()
///             .account_name("${exampleAccount.name}")
///             .location("${example.location}")
///             .name("example-netapppool")
///             .resource_group_name("${example.name}")
///             .service_level("Premium")
///             .size_in_tb(4)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// NetApp Pool can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:netapp/pool:Pool example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.NetApp/netAppAccounts/account1/capacityPools/pool1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PoolArgs {
        /// The name of the NetApp account in which the NetApp Pool should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The encryption type of the pool. Valid values include `Single`, and `Double`. Defaults to `Single`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub encryption_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the NetApp Pool. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// QoS Type of the pool. Valid values include `Auto` or `Manual`. Defaults to `Auto`.
        #[builder(into, default)]
        pub qos_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group where the NetApp Pool should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The service level of the file system. Valid values include `Premium`, `Standard`, and `Ultra`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub service_level: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Provisioned size of the pool in TB. Value must be between `1` and `2048`.
        ///
        /// > **NOTE** `2` TB capacity pool sizing is currently in preview. You can only take advantage of the `2` TB minimum if all the volumes in the capacity pool are using `Standard` network features. If any volume is using `Basic` network features, the minimum size is `4` TB. Please see the product [documentation](https://learn.microsoft.com/azure/azure-netapp-files/azure-netapp-files-set-up-capacity-pool) for more information.
        ///
        /// > **NOTE** The maximum `size_in_tb` is goverened by regional quotas. You may request additional capacity from Azure, currently up to `2048`.
        #[builder(into)]
        pub size_in_tb: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PoolResult {
        /// The name of the NetApp account in which the NetApp Pool should be created. Changing this forces a new resource to be created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// The encryption type of the pool. Valid values include `Single`, and `Double`. Defaults to `Single`. Changing this forces a new resource to be created.
        pub encryption_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the NetApp Pool. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// QoS Type of the pool. Valid values include `Auto` or `Manual`. Defaults to `Auto`.
        pub qos_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group where the NetApp Pool should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The service level of the file system. Valid values include `Premium`, `Standard`, and `Ultra`. Changing this forces a new resource to be created.
        pub service_level: pulumi_gestalt_rust::Output<String>,
        /// Provisioned size of the pool in TB. Value must be between `1` and `2048`.
        ///
        /// > **NOTE** `2` TB capacity pool sizing is currently in preview. You can only take advantage of the `2` TB minimum if all the volumes in the capacity pool are using `Standard` network features. If any volume is using `Basic` network features, the minimum size is `4` TB. Please see the product [documentation](https://learn.microsoft.com/azure/azure-netapp-files/azure-netapp-files-set-up-capacity-pool) for more information.
        ///
        /// > **NOTE** The maximum `size_in_tb` is goverened by regional quotas. You may request additional capacity from Azure, currently up to `2048`.
        pub size_in_tb: pulumi_gestalt_rust::Output<i32>,
        /// A mapping of tags to assign to the resource.
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
        args: PoolArgs,
    ) -> PoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let encryption_type_binding = args.encryption_type.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let qos_type_binding = args.qos_type.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let service_level_binding = args.service_level.get_output(context);
        let size_in_tb_binding = args.size_in_tb.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:netapp/pool:Pool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionType".into(),
                    value: &encryption_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "qosType".into(),
                    value: &qos_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceLevel".into(),
                    value: &service_level_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sizeInTb".into(),
                    value: &size_in_tb_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PoolResult {
            account_name: o.get_field("accountName"),
            encryption_type: o.get_field("encryptionType"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            qos_type: o.get_field("qosType"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_level: o.get_field("serviceLevel"),
            size_in_tb: o.get_field("sizeInTb"),
            tags: o.get_field("tags"),
        }
    }
}
