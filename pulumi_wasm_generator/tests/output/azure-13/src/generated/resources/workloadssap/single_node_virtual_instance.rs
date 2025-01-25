/// Manages an SAP Single Node Virtual Instance with new SAP System.
///
/// > **Note:** Before using this resource, it's required to submit the request of registering the Resource Provider with Azure CLI `az provider register --namespace "Microsoft.Workloads"`. The Resource Provider can take a while to register, you can check the status by running `az provider show --namespace "Microsoft.Workloads" --query "registrationState"`. Once this outputs "Registered" the Resource Provider is available for use.
///
/// ## Import
///
/// SAP Single Node Virtual Instances with new SAP Systems can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:workloadssap/singleNodeVirtualInstance:SingleNodeVirtualInstance example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Workloads/sapVirtualInstances/vis1
/// ```
///
pub mod single_node_virtual_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SingleNodeVirtualInstanceArgs {
        /// The Geo-Location where the SAP system is to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub app_location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The environment type for the SAP Single Node Virtual Instance. Possible values are `NonProd` and `Prod`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub environment: pulumi_wasm_rust::InputOrOutput<String>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::workloadssap::SingleNodeVirtualInstanceIdentity>,
        >,
        /// The Azure Region where the SAP Single Node Virtual Instance should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the managed Resource Group for the SAP Single Node Virtual Instance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub managed_resource_group_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of this SAP Single Node Virtual Instance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the SAP Single Node Virtual Instance should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The fully qualified domain name for the SAP system. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sap_fqdn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The SAP Product type for the SAP Single Node Virtual Instance. Possible values are `ECC`, `Other` and `S4HANA`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sap_product: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `single_server_configuration` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub single_server_configuration: pulumi_wasm_rust::InputOrOutput<
            super::super::types::workloadssap::SingleNodeVirtualInstanceSingleServerConfiguration,
        >,
        /// A mapping of tags which should be assigned to the SAP Single Node Virtual Instance.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SingleNodeVirtualInstanceResult {
        /// The Geo-Location where the SAP system is to be created. Changing this forces a new resource to be created.
        pub app_location: pulumi_wasm_rust::Output<String>,
        /// The environment type for the SAP Single Node Virtual Instance. Possible values are `NonProd` and `Prod`. Changing this forces a new resource to be created.
        pub environment: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::workloadssap::SingleNodeVirtualInstanceIdentity>,
        >,
        /// The Azure Region where the SAP Single Node Virtual Instance should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the managed Resource Group for the SAP Single Node Virtual Instance. Changing this forces a new resource to be created.
        pub managed_resource_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of this SAP Single Node Virtual Instance. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the SAP Single Node Virtual Instance should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The fully qualified domain name for the SAP system. Changing this forces a new resource to be created.
        pub sap_fqdn: pulumi_wasm_rust::Output<String>,
        /// The SAP Product type for the SAP Single Node Virtual Instance. Possible values are `ECC`, `Other` and `S4HANA`. Changing this forces a new resource to be created.
        pub sap_product: pulumi_wasm_rust::Output<String>,
        /// A `single_server_configuration` block as defined below. Changing this forces a new resource to be created.
        pub single_server_configuration: pulumi_wasm_rust::Output<
            super::super::types::workloadssap::SingleNodeVirtualInstanceSingleServerConfiguration,
        >,
        /// A mapping of tags which should be assigned to the SAP Single Node Virtual Instance.
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
        args: SingleNodeVirtualInstanceArgs,
    ) -> SingleNodeVirtualInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_location_binding = args.app_location.get_output(context).get_inner();
        let environment_binding = args.environment.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let managed_resource_group_name_binding = args
            .managed_resource_group_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sap_fqdn_binding = args.sap_fqdn.get_output(context).get_inner();
        let sap_product_binding = args.sap_product.get_output(context).get_inner();
        let single_server_configuration_binding = args
            .single_server_configuration
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:workloadssap/singleNodeVirtualInstance:SingleNodeVirtualInstance"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appLocation".into(),
                    value: &app_location_binding,
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sapFqdn".into(),
                    value: &sap_fqdn_binding,
                },
                register_interface::ObjectField {
                    name: "sapProduct".into(),
                    value: &sap_product_binding,
                },
                register_interface::ObjectField {
                    name: "singleServerConfiguration".into(),
                    value: &single_server_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appLocation".into(),
                },
                register_interface::ResultField {
                    name: "environment".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managedResourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sapFqdn".into(),
                },
                register_interface::ResultField {
                    name: "sapProduct".into(),
                },
                register_interface::ResultField {
                    name: "singleServerConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SingleNodeVirtualInstanceResult {
            app_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appLocation").unwrap(),
            ),
            environment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environment").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            managed_resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedResourceGroupName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sap_fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sapFqdn").unwrap(),
            ),
            sap_product: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sapProduct").unwrap(),
            ),
            single_server_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("singleServerConfiguration").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
