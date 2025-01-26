/// Manages a Microsoft SQL Virtual Machine Availability Group Listener.
///
/// ## Import
///
/// Microsoft SQL Virtual Machine Availability Group Listeners can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/virtualMachineAvailabilityGroupListener:VirtualMachineAvailabilityGroupListener example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachineGroups/vmgroup1/availabilityGroupListeners/listener1
/// ```
///
pub mod virtual_machine_availability_group_listener {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineAvailabilityGroupListenerArgs {
        /// The name of the Availability Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub availability_group_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `load_balancer_configuration` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Either one of `load_balancer_configuration` or `multi_subnet_ip_configuration` must be specified.
        #[builder(into, default)]
        pub load_balancer_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::mssql::VirtualMachineAvailabilityGroupListenerLoadBalancerConfiguration,
            >,
        >,
        /// One or more `multi_subnet_ip_configuration` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub multi_subnet_ip_configurations: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::mssql::VirtualMachineAvailabilityGroupListenerMultiSubnetIpConfiguration,
                >,
            >,
        >,
        /// The name which should be used for the Microsoft SQL Virtual Machine Availability Group Listener. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The port of the listener. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// One or more `replica` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub replicas: pulumi_wasm_rust::InputOrOutput<
            Vec<
                super::super::types::mssql::VirtualMachineAvailabilityGroupListenerReplica,
            >,
        >,
        /// The ID of the SQL Virtual Machine Group to create the listener. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sql_virtual_machine_group_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineAvailabilityGroupListenerResult {
        /// The name of the Availability Group. Changing this forces a new resource to be created.
        pub availability_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `load_balancer_configuration` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Either one of `load_balancer_configuration` or `multi_subnet_ip_configuration` must be specified.
        pub load_balancer_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::mssql::VirtualMachineAvailabilityGroupListenerLoadBalancerConfiguration,
            >,
        >,
        /// One or more `multi_subnet_ip_configuration` blocks as defined below. Changing this forces a new resource to be created.
        pub multi_subnet_ip_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::mssql::VirtualMachineAvailabilityGroupListenerMultiSubnetIpConfiguration,
                >,
            >,
        >,
        /// The name which should be used for the Microsoft SQL Virtual Machine Availability Group Listener. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The port of the listener. Changing this forces a new resource to be created.
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// One or more `replica` blocks as defined below. Changing this forces a new resource to be created.
        pub replicas: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::mssql::VirtualMachineAvailabilityGroupListenerReplica,
            >,
        >,
        /// The ID of the SQL Virtual Machine Group to create the listener. Changing this forces a new resource to be created.
        pub sql_virtual_machine_group_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VirtualMachineAvailabilityGroupListenerArgs,
    ) -> VirtualMachineAvailabilityGroupListenerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let availability_group_name_binding = args
            .availability_group_name
            .get_output(context)
            .get_inner();
        let load_balancer_configuration_binding = args
            .load_balancer_configuration
            .get_output(context)
            .get_inner();
        let multi_subnet_ip_configurations_binding = args
            .multi_subnet_ip_configurations
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let replicas_binding = args.replicas.get_output(context).get_inner();
        let sql_virtual_machine_group_id_binding = args
            .sql_virtual_machine_group_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/virtualMachineAvailabilityGroupListener:VirtualMachineAvailabilityGroupListener"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "availabilityGroupName".into(),
                    value: &availability_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancerConfiguration".into(),
                    value: &load_balancer_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "multiSubnetIpConfigurations".into(),
                    value: &multi_subnet_ip_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "replicas".into(),
                    value: &replicas_binding,
                },
                register_interface::ObjectField {
                    name: "sqlVirtualMachineGroupId".into(),
                    value: &sql_virtual_machine_group_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "availabilityGroupName".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancerConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "multiSubnetIpConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "replicas".into(),
                },
                register_interface::ResultField {
                    name: "sqlVirtualMachineGroupId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualMachineAvailabilityGroupListenerResult {
            availability_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityGroupName").unwrap(),
            ),
            load_balancer_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancerConfiguration").unwrap(),
            ),
            multi_subnet_ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiSubnetIpConfigurations").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            replicas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicas").unwrap(),
            ),
            sql_virtual_machine_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqlVirtualMachineGroupId").unwrap(),
            ),
        }
    }
}
