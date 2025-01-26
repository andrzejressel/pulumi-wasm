/// Manages a Machine Learning Compute Instance.
///
/// ## Example Usage
///
/// ```yaml
/// configuration:
///   sshKey:
///     type: string
///     default: ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQCqaZoyiz1qbdOQ8xEf6uEu1cCwYowo5FHtsBhqLoDnnp7KUTEBN+L2NxRIfQ781rxV6Iq5jSav6b2Q8z5KiseOlvKA/RF2wqU0UPYqQviQhLmW6THTpmrv/YkUCuzxDpsH7DUDhZcwySLKVVe0Qm3+5N2Ta6UYH3lsDf9R9wTP2K/+vAnflKebuypNlmocIvakFWoZda18FOmsOoIVXQ8HWFNCuw9ZCunMSN62QGamCe3dL5cXlkgHYv7ekJE15IA9aOJcM7e90oeTqo+7HTcWfdu0qQqPWY5ujyMw/llas8tsXY85LFqRnr3gJ02bAscjc477+X+j/gkpFoN1QEmt terraform@demo.tld
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: west europe
///       tags:
///         stage: example
///   exampleInsights:
///     type: azure:appinsights:Insights
///     name: example
///     properties:
///       name: example-ai
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationType: web
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: example-kv
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: standard
///       purgeProtectionEnabled: true
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplesa
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleWorkspace:
///     type: azure:machinelearning:Workspace
///     name: example
///     properties:
///       name: example-mlw
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationInsightsId: ${exampleInsights.id}
///       keyVaultId: ${exampleKeyVault.id}
///       storageAccountId: ${exampleAccount.id}
///       identity:
///         type: SystemAssigned
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.1.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.1.0.0/24
///   exampleComputeInstance:
///     type: azure:machinelearning:ComputeInstance
///     name: example
///     properties:
///       name: example
///       machineLearningWorkspaceId: ${exampleWorkspace.id}
///       virtualMachineSize: STANDARD_DS2_V2
///       authorizationType: personal
///       ssh:
///         publicKey: ${sshKey}
///       subnetResourceId: ${exampleSubnet.id}
///       description: foo
///       tags:
///         foo: bar
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Machine Learning Compute Instances can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:machinelearning/computeInstance:ComputeInstance example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.MachineLearningServices/workspaces/workspace1/computes/compute1
/// ```
///
pub mod compute_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ComputeInstanceArgs {
        /// A `assign_to_user` block as defined below. A user explicitly assigned to a personal compute instance. Changing this forces a new Machine Learning Compute Instance to be created.
        #[builder(into, default)]
        pub assign_to_user: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::machinelearning::ComputeInstanceAssignToUser>,
        >,
        /// The Compute Instance Authorization type. Possible values include: `personal`. Changing this forces a new Machine Learning Compute Instance to be created.
        #[builder(into, default)]
        pub authorization_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The description of the Machine Learning Compute Instance. Changing this forces a new Machine Learning Compute Instance to be created.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below. Changing this forces a new Machine Learning Compute Instance to be created.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::machinelearning::ComputeInstanceIdentity>,
        >,
        /// Whether local authentication methods is enabled. Defaults to `true`. Changing this forces a new Machine Learning Compute Instance to be created.
        #[builder(into, default)]
        pub local_auth_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Machine Learning Workspace. Changing this forces a new Machine Learning Compute Instance to be created.
        #[builder(into)]
        pub machine_learning_workspace_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this Machine Learning Compute Instance. Changing this forces a new Machine Learning Compute Instance to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether the compute instance will have a public ip. To set this to false a `subnet_resource_id` needs to be set. Defaults to `true`. Changing this forces a new Machine Learning Compute Cluster to be created.
        #[builder(into, default)]
        pub node_public_ip_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A `ssh` block as defined below. Specifies policy and settings for SSH access. Changing this forces a new Machine Learning Compute Instance to be created.
        #[builder(into, default)]
        pub ssh: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::machinelearning::ComputeInstanceSsh>,
        >,
        /// Virtual network subnet resource ID the compute nodes belong to. Changing this forces a new Machine Learning Compute Instance to be created.
        #[builder(into, default)]
        pub subnet_resource_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Machine Learning Compute Instance. Changing this forces a new Machine Learning Compute Instance to be created.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Virtual Machine Size. Changing this forces a new Machine Learning Compute Instance to be created.
        #[builder(into)]
        pub virtual_machine_size: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ComputeInstanceResult {
        /// A `assign_to_user` block as defined below. A user explicitly assigned to a personal compute instance. Changing this forces a new Machine Learning Compute Instance to be created.
        pub assign_to_user: pulumi_wasm_rust::Output<
            Option<super::super::types::machinelearning::ComputeInstanceAssignToUser>,
        >,
        /// The Compute Instance Authorization type. Possible values include: `personal`. Changing this forces a new Machine Learning Compute Instance to be created.
        pub authorization_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of the Machine Learning Compute Instance. Changing this forces a new Machine Learning Compute Instance to be created.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below. Changing this forces a new Machine Learning Compute Instance to be created.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::machinelearning::ComputeInstanceIdentity>,
        >,
        /// Whether local authentication methods is enabled. Defaults to `true`. Changing this forces a new Machine Learning Compute Instance to be created.
        pub local_auth_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Machine Learning Workspace. Changing this forces a new Machine Learning Compute Instance to be created.
        pub machine_learning_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Machine Learning Compute Instance. Changing this forces a new Machine Learning Compute Instance to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether the compute instance will have a public ip. To set this to false a `subnet_resource_id` needs to be set. Defaults to `true`. Changing this forces a new Machine Learning Compute Cluster to be created.
        pub node_public_ip_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `ssh` block as defined below. Specifies policy and settings for SSH access. Changing this forces a new Machine Learning Compute Instance to be created.
        pub ssh: pulumi_wasm_rust::Output<
            Option<super::super::types::machinelearning::ComputeInstanceSsh>,
        >,
        /// Virtual network subnet resource ID the compute nodes belong to. Changing this forces a new Machine Learning Compute Instance to be created.
        pub subnet_resource_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Machine Learning Compute Instance. Changing this forces a new Machine Learning Compute Instance to be created.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Virtual Machine Size. Changing this forces a new Machine Learning Compute Instance to be created.
        pub virtual_machine_size: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ComputeInstanceArgs,
    ) -> ComputeInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let assign_to_user_binding = args.assign_to_user.get_output(context).get_inner();
        let authorization_type_binding = args
            .authorization_type
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let local_auth_enabled_binding = args
            .local_auth_enabled
            .get_output(context)
            .get_inner();
        let machine_learning_workspace_id_binding = args
            .machine_learning_workspace_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let node_public_ip_enabled_binding = args
            .node_public_ip_enabled
            .get_output(context)
            .get_inner();
        let ssh_binding = args.ssh.get_output(context).get_inner();
        let subnet_resource_id_binding = args
            .subnet_resource_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let virtual_machine_size_binding = args
            .virtual_machine_size
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:machinelearning/computeInstance:ComputeInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "assignToUser".into(),
                    value: &assign_to_user_binding,
                },
                register_interface::ObjectField {
                    name: "authorizationType".into(),
                    value: &authorization_type_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "localAuthEnabled".into(),
                    value: &local_auth_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "machineLearningWorkspaceId".into(),
                    value: &machine_learning_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nodePublicIpEnabled".into(),
                    value: &node_public_ip_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "ssh".into(),
                    value: &ssh_binding,
                },
                register_interface::ObjectField {
                    name: "subnetResourceId".into(),
                    value: &subnet_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "virtualMachineSize".into(),
                    value: &virtual_machine_size_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "assignToUser".into(),
                },
                register_interface::ResultField {
                    name: "authorizationType".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "localAuthEnabled".into(),
                },
                register_interface::ResultField {
                    name: "machineLearningWorkspaceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nodePublicIpEnabled".into(),
                },
                register_interface::ResultField {
                    name: "ssh".into(),
                },
                register_interface::ResultField {
                    name: "subnetResourceId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "virtualMachineSize".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ComputeInstanceResult {
            assign_to_user: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assignToUser").unwrap(),
            ),
            authorization_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizationType").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            local_auth_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localAuthEnabled").unwrap(),
            ),
            machine_learning_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("machineLearningWorkspaceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            node_public_ip_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodePublicIpEnabled").unwrap(),
            ),
            ssh: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ssh").unwrap(),
            ),
            subnet_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetResourceId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            virtual_machine_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualMachineSize").unwrap(),
            ),
        }
    }
}
