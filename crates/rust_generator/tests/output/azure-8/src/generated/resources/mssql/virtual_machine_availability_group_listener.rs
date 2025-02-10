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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_machine_availability_group_listener {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineAvailabilityGroupListenerArgs {
        /// The name of the Availability Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub availability_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `load_balancer_configuration` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Either one of `load_balancer_configuration` or `multi_subnet_ip_configuration` must be specified.
        #[builder(into, default)]
        pub load_balancer_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::mssql::VirtualMachineAvailabilityGroupListenerLoadBalancerConfiguration,
            >,
        >,
        /// One or more `multi_subnet_ip_configuration` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub multi_subnet_ip_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::mssql::VirtualMachineAvailabilityGroupListenerMultiSubnetIpConfiguration,
                >,
            >,
        >,
        /// The name which should be used for the Microsoft SQL Virtual Machine Availability Group Listener. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The port of the listener. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// One or more `replica` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub replicas: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::types::mssql::VirtualMachineAvailabilityGroupListenerReplica,
            >,
        >,
        /// The ID of the SQL Virtual Machine Group to create the listener. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sql_virtual_machine_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineAvailabilityGroupListenerResult {
        /// The name of the Availability Group. Changing this forces a new resource to be created.
        pub availability_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `load_balancer_configuration` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Either one of `load_balancer_configuration` or `multi_subnet_ip_configuration` must be specified.
        pub load_balancer_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::mssql::VirtualMachineAvailabilityGroupListenerLoadBalancerConfiguration,
            >,
        >,
        /// One or more `multi_subnet_ip_configuration` blocks as defined below. Changing this forces a new resource to be created.
        pub multi_subnet_ip_configurations: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::mssql::VirtualMachineAvailabilityGroupListenerMultiSubnetIpConfiguration,
                >,
            >,
        >,
        /// The name which should be used for the Microsoft SQL Virtual Machine Availability Group Listener. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The port of the listener. Changing this forces a new resource to be created.
        pub port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// One or more `replica` blocks as defined below. Changing this forces a new resource to be created.
        pub replicas: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::mssql::VirtualMachineAvailabilityGroupListenerReplica,
            >,
        >,
        /// The ID of the SQL Virtual Machine Group to create the listener. Changing this forces a new resource to be created.
        pub sql_virtual_machine_group_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualMachineAvailabilityGroupListenerArgs,
    ) -> VirtualMachineAvailabilityGroupListenerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let availability_group_name_binding = args
            .availability_group_name
            .get_output(context);
        let load_balancer_configuration_binding = args
            .load_balancer_configuration
            .get_output(context);
        let multi_subnet_ip_configurations_binding = args
            .multi_subnet_ip_configurations
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let port_binding = args.port.get_output(context);
        let replicas_binding = args.replicas.get_output(context);
        let sql_virtual_machine_group_id_binding = args
            .sql_virtual_machine_group_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mssql/virtualMachineAvailabilityGroupListener:VirtualMachineAvailabilityGroupListener"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityGroupName".into(),
                    value: availability_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancerConfiguration".into(),
                    value: load_balancer_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiSubnetIpConfigurations".into(),
                    value: multi_subnet_ip_configurations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicas".into(),
                    value: replicas_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlVirtualMachineGroupId".into(),
                    value: sql_virtual_machine_group_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualMachineAvailabilityGroupListenerResult {
            availability_group_name: o.get_field("availabilityGroupName"),
            load_balancer_configuration: o.get_field("loadBalancerConfiguration"),
            multi_subnet_ip_configurations: o.get_field("multiSubnetIpConfigurations"),
            name: o.get_field("name"),
            port: o.get_field("port"),
            replicas: o.get_field("replicas"),
            sql_virtual_machine_group_id: o.get_field("sqlVirtualMachineGroupId"),
        }
    }
}
