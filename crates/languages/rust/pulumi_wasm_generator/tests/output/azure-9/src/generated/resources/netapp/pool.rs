/// Manages a Pool within a NetApp Account.
///
/// ## NetApp Pool Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PoolArgs {
        /// The name of the NetApp account in which the NetApp Pool should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The encryption type of the pool. Valid values include `Single`, and `Double`. Defaults to `Single`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub encryption_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the NetApp Pool. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// QoS Type of the pool. Valid values include `Auto` or `Manual`. Defaults to `Auto`.
        #[builder(into, default)]
        pub qos_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group where the NetApp Pool should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The service level of the file system. Valid values include `Premium`, `Standard`, and `Ultra`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub service_level: pulumi_wasm_rust::InputOrOutput<String>,
        /// Provisioned size of the pool in TB. Value must be between `1` and `2048`.
        ///
        /// > **NOTE** `2` TB capacity pool sizing is currently in preview. You can only take advantage of the `2` TB minimum if all the volumes in the capacity pool are using `Standard` network features. If any volume is using `Basic` network features, the minimum size is `4` TB. Please see the product [documentation](https://learn.microsoft.com/azure/azure-netapp-files/azure-netapp-files-set-up-capacity-pool) for more information.
        ///
        /// > **NOTE** The maximum `size_in_tb` is goverened by regional quotas. You may request additional capacity from Azure, currently up to `2048`.
        #[builder(into)]
        pub size_in_tb: pulumi_wasm_rust::InputOrOutput<i32>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PoolResult {
        /// The name of the NetApp account in which the NetApp Pool should be created. Changing this forces a new resource to be created.
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// The encryption type of the pool. Valid values include `Single`, and `Double`. Defaults to `Single`. Changing this forces a new resource to be created.
        pub encryption_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the NetApp Pool. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// QoS Type of the pool. Valid values include `Auto` or `Manual`. Defaults to `Auto`.
        pub qos_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group where the NetApp Pool should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The service level of the file system. Valid values include `Premium`, `Standard`, and `Ultra`. Changing this forces a new resource to be created.
        pub service_level: pulumi_wasm_rust::Output<String>,
        /// Provisioned size of the pool in TB. Value must be between `1` and `2048`.
        ///
        /// > **NOTE** `2` TB capacity pool sizing is currently in preview. You can only take advantage of the `2` TB minimum if all the volumes in the capacity pool are using `Standard` network features. If any volume is using `Basic` network features, the minimum size is `4` TB. Please see the product [documentation](https://learn.microsoft.com/azure/azure-netapp-files/azure-netapp-files-set-up-capacity-pool) for more information.
        ///
        /// > **NOTE** The maximum `size_in_tb` is goverened by regional quotas. You may request additional capacity from Azure, currently up to `2048`.
        pub size_in_tb: pulumi_wasm_rust::Output<i32>,
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
        args: PoolArgs,
    ) -> PoolResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_output(context).get_inner();
        let encryption_type_binding = args
            .encryption_type
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let qos_type_binding = args.qos_type.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let service_level_binding = args.service_level.get_output(context).get_inner();
        let size_in_tb_binding = args.size_in_tb.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:netapp/pool:Pool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionType".into(),
                    value: &encryption_type_binding,
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
                    name: "qosType".into(),
                    value: &qos_type_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceLevel".into(),
                    value: &service_level_binding,
                },
                register_interface::ObjectField {
                    name: "sizeInTb".into(),
                    value: &size_in_tb_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PoolResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountName"),
            ),
            encryption_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryptionType"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            qos_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("qosType"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            service_level: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceLevel"),
            ),
            size_in_tb: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sizeInTb"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
