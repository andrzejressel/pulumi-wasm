/// Manages a System Center Virtual Machine Manager Server.
///
/// > **Note:** By request of the service team the provider no longer automatically registering the `Microsoft.ScVmm` Resource Provider for this resource. To register it you can run `az provider register --namespace Microsoft.ScVmm`.
///
/// > **Note:** This resource depends on an existing `System Center Virtual Machine Manager Host Machine`, `Arc Resource Bridge` and `Custom Location`. Installing and configuring these dependencies is outside the scope of this document. See [Virtual Machine Manager documentation](https://learn.microsoft.com/en-us/system-center/vmm/?view=sc-vmm-2022) and [Install VMM](https://learn.microsoft.com/en-us/system-center/vmm/install?view=sc-vmm-2022) for more details of `System Center Virtual Machine Manager Host Machine`. See [What is Azure Arc resource bridge](https://learn.microsoft.com/en-us/azure/azure-arc/resource-bridge/overview) and [Overview of Arc-enabled System Center Virtual Machine Manager](https://learn.microsoft.com/en-us/azure/azure-arc/system-center-virtual-machine-manager/overview) for more details of `Arc Resource Bridge/Appliance`. See [Create and manage custom locations on Azure Arc-enabled Kubernetes](https://learn.microsoft.com/en-us/azure/azure-arc/kubernetes/custom-locations) for more details of `Custom Location`. If you encounter issues while configuring, we'd recommend opening a ticket with Microsoft Support.
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
///     let exampleVirtualMachineManagerServer = virtual_machine_manager_server::create(
///         "exampleVirtualMachineManagerServer",
///         VirtualMachineManagerServerArgs::builder()
///             .custom_location_id(
///                 "/subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.ExtendedLocation/customLocations/customLocation1",
///             )
///             .fqdn("example.labtest")
///             .location("${example.location}")
///             .name("example-scvmmms")
///             .password("H@Sh1CoR3!")
///             .resource_group_name("${example.name}")
///             .username("testUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// System Center Virtual Machine Manager Servers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:systemcenter/virtualMachineManagerServer:VirtualMachineManagerServer example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.ScVmm/vmmServers/vmmServer1
/// ```
///
pub mod virtual_machine_manager_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineManagerServerArgs {
        /// The ID of the Custom Location for the System Center Virtual Machine Manager Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub custom_location_id: pulumi_wasm_rust::Output<String>,
        /// The FQDN of the System Center Virtual Machine Manager Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the System Center Virtual Machine Manager Server should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the System Center Virtual Machine Manager Server. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The password that is used to connect to the System Center Virtual Machine Manager Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub password: pulumi_wasm_rust::Output<String>,
        /// The port on which the System Center Virtual Machine Manager Server is listening. Possible values are between `1` and `65535`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the Resource Group where the System Center Virtual Machine Manager should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the System Center Virtual Machine Manager Server.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The username that is used to connect to the System Center Virtual Machine Manager Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub username: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineManagerServerResult {
        /// The ID of the Custom Location for the System Center Virtual Machine Manager Server. Changing this forces a new resource to be created.
        pub custom_location_id: pulumi_wasm_rust::Output<String>,
        /// The FQDN of the System Center Virtual Machine Manager Server. Changing this forces a new resource to be created.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the System Center Virtual Machine Manager Server should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the System Center Virtual Machine Manager Server. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The password that is used to connect to the System Center Virtual Machine Manager Server. Changing this forces a new resource to be created.
        pub password: pulumi_wasm_rust::Output<String>,
        /// The port on which the System Center Virtual Machine Manager Server is listening. Possible values are between `1` and `65535`. Changing this forces a new resource to be created.
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the Resource Group where the System Center Virtual Machine Manager should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the System Center Virtual Machine Manager Server.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The username that is used to connect to the System Center Virtual Machine Manager Server. Changing this forces a new resource to be created.
        pub username: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VirtualMachineManagerServerArgs,
    ) -> VirtualMachineManagerServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_location_id_binding = args.custom_location_id.get_inner();
        let fqdn_binding = args.fqdn.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let password_binding = args.password.get_inner();
        let port_binding = args.port.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let username_binding = args.username.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:systemcenter/virtualMachineManagerServer:VirtualMachineManagerServer"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customLocationId".into(),
                    value: &custom_location_id_binding,
                },
                register_interface::ObjectField {
                    name: "fqdn".into(),
                    value: &fqdn_binding,
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
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "username".into(),
                    value: &username_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "customLocationId".into(),
                },
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "username".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualMachineManagerServerResult {
            custom_location_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customLocationId").unwrap(),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("username").unwrap(),
            ),
        }
    }
}
