/// Manages a Microsoft SQL Virtual Machine Group.
///
/// ## Example Usage
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
///     let exampleVirtualMachineGroup = virtual_machine_group::create(
///         "exampleVirtualMachineGroup",
///         VirtualMachineGroupArgs::builder()
///             .location("${example.location}")
///             .name("examplegroup")
///             .resource_group_name("${example.name}")
///             .sql_image_offer("SQL2017-WS2016")
///             .sql_image_sku("Developer")
///             .wsfc_domain_profile(
///                 VirtualMachineGroupWsfcDomainProfile::builder()
///                     .clusterSubnetType("SingleSubnet")
///                     .fqdn("testdomain.com")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Microsoft SQL Virtual Machine Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/virtualMachineGroup:VirtualMachineGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachineGroups/vmgroup1
/// ```
///
pub mod virtual_machine_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineGroupArgs {
        /// The Azure Region where the Microsoft SQL Virtual Machine Group should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for the Microsoft SQL Virtual Machine Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Microsoft SQL Virtual Machine Group should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The offer type of the marketplace image cluster to be used by the SQL Virtual Machine Group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sql_image_offer: pulumi_wasm_rust::Output<String>,
        /// The sku type of the marketplace image cluster to be used by the SQL Virtual Machine Group. Possible values are `Developer` and `Enterprise`.
        #[builder(into)]
        pub sql_image_sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Microsoft SQL Virtual Machine Group.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `wsfc_domain_profile` block as defined below.
        #[builder(into)]
        pub wsfc_domain_profile: pulumi_wasm_rust::Output<
            super::super::types::mssql::VirtualMachineGroupWsfcDomainProfile,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineGroupResult {
        /// The Azure Region where the Microsoft SQL Virtual Machine Group should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for the Microsoft SQL Virtual Machine Group. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Microsoft SQL Virtual Machine Group should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The offer type of the marketplace image cluster to be used by the SQL Virtual Machine Group. Changing this forces a new resource to be created.
        pub sql_image_offer: pulumi_wasm_rust::Output<String>,
        /// The sku type of the marketplace image cluster to be used by the SQL Virtual Machine Group. Possible values are `Developer` and `Enterprise`.
        pub sql_image_sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Microsoft SQL Virtual Machine Group.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `wsfc_domain_profile` block as defined below.
        pub wsfc_domain_profile: pulumi_wasm_rust::Output<
            super::super::types::mssql::VirtualMachineGroupWsfcDomainProfile,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VirtualMachineGroupArgs,
    ) -> VirtualMachineGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sql_image_offer_binding = args.sql_image_offer.get_inner();
        let sql_image_sku_binding = args.sql_image_sku.get_inner();
        let tags_binding = args.tags.get_inner();
        let wsfc_domain_profile_binding = args.wsfc_domain_profile.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/virtualMachineGroup:VirtualMachineGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sqlImageOffer".into(),
                    value: &sql_image_offer_binding,
                },
                register_interface::ObjectField {
                    name: "sqlImageSku".into(),
                    value: &sql_image_sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "wsfcDomainProfile".into(),
                    value: &wsfc_domain_profile_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sqlImageOffer".into(),
                },
                register_interface::ResultField {
                    name: "sqlImageSku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "wsfcDomainProfile".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualMachineGroupResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sql_image_offer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqlImageOffer").unwrap(),
            ),
            sql_image_sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqlImageSku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            wsfc_domain_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("wsfcDomainProfile").unwrap(),
            ),
        }
    }
}
