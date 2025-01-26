/// Manages a Stack HCI Deployment Setting.
///
/// > Note: Completion of the prerequisites of deploying the Azure Stack HCI in your environment is outside the scope of this document. For more details refer to the [Azure Stack HCI deployment sequence](https://learn.microsoft.com/en-us/azure-stack/hci/deploy/deployment-introduction#deployment-sequence). If you encounter issues completing the prerequisites, we'd recommend opening a ticket with Microsoft Support.
///
/// > Note: During the deployment process, the service will generate additional resources, including a new Arc Bridge Appliance and a Custom Location containing several Stack HCI Storage Paths. The provider will attempt to remove these resources on the deletion or recreation of `azure.stack.HciDeploymentSetting`.
///
/// ## Import
///
/// Stack HCI Deployment Settings can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:stack/hciDeploymentSetting:HciDeploymentSetting example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.AzureStackHCI/clusters/clus1/deploymentSettings/default
/// ```
///
pub mod hci_deployment_setting {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HciDeploymentSettingArgs {
        /// Specifies a list of IDs of Azure ARC machine resource to be part of cluster. Changing this forces a new Stack HCI Deployment Setting to be created.
        #[builder(into)]
        pub arc_resource_ids: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// One or more `scale_unit` blocks as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
        #[builder(into)]
        pub scale_units: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::stack::HciDeploymentSettingScaleUnit>,
        >,
        /// The ID of the Azure Stack HCI cluster. Changing this forces a new Stack HCI Deployment Setting to be created.
        #[builder(into)]
        pub stack_hci_cluster_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The deployment template version. The format must be a set of numbers separated by dots such as `10.0.0.0`. Changing this forces a new Stack HCI Deployment Setting to be created.
        #[builder(into)]
        pub version: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct HciDeploymentSettingResult {
        /// Specifies a list of IDs of Azure ARC machine resource to be part of cluster. Changing this forces a new Stack HCI Deployment Setting to be created.
        pub arc_resource_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// One or more `scale_unit` blocks as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
        pub scale_units: pulumi_wasm_rust::Output<
            Vec<super::super::types::stack::HciDeploymentSettingScaleUnit>,
        >,
        /// The ID of the Azure Stack HCI cluster. Changing this forces a new Stack HCI Deployment Setting to be created.
        pub stack_hci_cluster_id: pulumi_wasm_rust::Output<String>,
        /// The deployment template version. The format must be a set of numbers separated by dots such as `10.0.0.0`. Changing this forces a new Stack HCI Deployment Setting to be created.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: HciDeploymentSettingArgs,
    ) -> HciDeploymentSettingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arc_resource_ids_binding = args
            .arc_resource_ids
            .get_output(context)
            .get_inner();
        let scale_units_binding = args.scale_units.get_output(context).get_inner();
        let stack_hci_cluster_id_binding = args
            .stack_hci_cluster_id
            .get_output(context)
            .get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:stack/hciDeploymentSetting:HciDeploymentSetting".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arcResourceIds".into(),
                    value: &arc_resource_ids_binding,
                },
                register_interface::ObjectField {
                    name: "scaleUnits".into(),
                    value: &scale_units_binding,
                },
                register_interface::ObjectField {
                    name: "stackHciClusterId".into(),
                    value: &stack_hci_cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arcResourceIds".into(),
                },
                register_interface::ResultField {
                    name: "scaleUnits".into(),
                },
                register_interface::ResultField {
                    name: "stackHciClusterId".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HciDeploymentSettingResult {
            arc_resource_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arcResourceIds").unwrap(),
            ),
            scale_units: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scaleUnits").unwrap(),
            ),
            stack_hci_cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackHciClusterId").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
