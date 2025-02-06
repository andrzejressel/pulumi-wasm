/// Manages an SAP Discovery Virtual Instance.
///
/// > **Note:** Before using this resource, it's required to submit the request of registering the Resource Provider with Azure CLI `az provider register --namespace "Microsoft.Workloads"`. The Resource Provider can take a while to register, you can check the status by running `az provider show --namespace "Microsoft.Workloads" --query "registrationState"`. Once this outputs "Registered" the Resource Provider is available for use.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-sapvis
///       location: West Europe
///   exampleDiscoveryVirtualInstance:
///     type: azure:workloadssap:DiscoveryVirtualInstance
///     name: example
///     properties:
///       name: X01
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       environment: NonProd
///       sapProduct: S4HANA
///       centralServerVirtualMachineId: /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/exampleRG/providers/Microsoft.Compute/virtualMachines/csvm1
///       managedStorageAccountName: managedsa
///       identity:
///         type: UserAssigned
///         identityIds:
///           - /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/exampleRG/providers/Microsoft.ManagedIdentity/userAssignedIdentities/uai1
/// ```
///
/// ## Import
///
/// SAP Discovery Virtual Instances can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:workloadssap/discoveryVirtualInstance:DiscoveryVirtualInstance example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Workloads/sapVirtualInstances/vis1
/// ```
///
pub mod discovery_virtual_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DiscoveryVirtualInstanceArgs {
        /// The ID of the Virtual Machine of the Central Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub central_server_virtual_machine_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// The environment type for the SAP Discovery Virtual Instance. Possible values are `NonProd` and `Prod`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub environment: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::workloadssap::DiscoveryVirtualInstanceIdentity>,
        >,
        /// The Azure Region where the SAP Discovery Virtual Instance should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the managed Resource Group for the SAP Discovery Virtual Instance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub managed_resource_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the custom Storage Account created by the service in the managed Resource Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub managed_storage_account_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the SAP Discovery Virtual Instance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the SAP Discovery Virtual Instance should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SAP Product type for the SAP Discovery Virtual Instance. Possible values are `ECC`, `Other` and `S4HANA`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sap_product: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the SAP Discovery Virtual Instance.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DiscoveryVirtualInstanceResult {
        /// The ID of the Virtual Machine of the Central Server. Changing this forces a new resource to be created.
        pub central_server_virtual_machine_id: pulumi_gestalt_rust::Output<String>,
        /// The environment type for the SAP Discovery Virtual Instance. Possible values are `NonProd` and `Prod`. Changing this forces a new resource to be created.
        pub environment: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::workloadssap::DiscoveryVirtualInstanceIdentity>,
        >,
        /// The Azure Region where the SAP Discovery Virtual Instance should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the managed Resource Group for the SAP Discovery Virtual Instance. Changing this forces a new resource to be created.
        pub managed_resource_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the custom Storage Account created by the service in the managed Resource Group. Changing this forces a new resource to be created.
        pub managed_storage_account_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the SAP Discovery Virtual Instance. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the SAP Discovery Virtual Instance should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SAP Product type for the SAP Discovery Virtual Instance. Possible values are `ECC`, `Other` and `S4HANA`. Changing this forces a new resource to be created.
        pub sap_product: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the SAP Discovery Virtual Instance.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DiscoveryVirtualInstanceArgs,
    ) -> DiscoveryVirtualInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let central_server_virtual_machine_id_binding = args
            .central_server_virtual_machine_id
            .get_output(context)
            .get_inner();
        let environment_binding = args.environment.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let managed_resource_group_name_binding = args
            .managed_resource_group_name
            .get_output(context)
            .get_inner();
        let managed_storage_account_name_binding = args
            .managed_storage_account_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sap_product_binding = args.sap_product.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:workloadssap/discoveryVirtualInstance:DiscoveryVirtualInstance"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "centralServerVirtualMachineId".into(),
                    value: &central_server_virtual_machine_id_binding,
                },
                register_interface::ObjectField {
                    name: "environment".into(),
                    value: &environment_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managedResourceGroupName".into(),
                    value: &managed_resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "managedStorageAccountName".into(),
                    value: &managed_storage_account_name_binding,
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
                    name: "sapProduct".into(),
                    value: &sap_product_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DiscoveryVirtualInstanceResult {
            central_server_virtual_machine_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("centralServerVirtualMachineId"),
            ),
            environment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environment"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            managed_resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedResourceGroupName"),
            ),
            managed_storage_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedStorageAccountName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sap_product: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sapProduct"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
