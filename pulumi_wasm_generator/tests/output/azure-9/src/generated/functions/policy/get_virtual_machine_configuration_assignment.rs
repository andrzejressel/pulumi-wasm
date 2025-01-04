pub mod get_virtual_machine_configuration_assignment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualMachineConfigurationAssignmentArgs {
        /// Specifies the name of the Guest Configuration Assignment.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the Name of the Resource Group where the Guest Configuration Assignment exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Only retrieve Policy Set Definitions from this Management Group.
        #[builder(into)]
        pub virtual_machine_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualMachineConfigurationAssignmentResult {
        /// Combined hash of the configuration package and parameters.
        pub assignment_hash: pulumi_wasm_rust::Output<String>,
        /// A value indicating compliance status of the machine for the assigned guest configuration. Possible return values are `Compliant`, `NonCompliant` and `Pending`.
        pub compliance_status: pulumi_wasm_rust::Output<String>,
        /// The content hash for the Guest Configuration package.
        pub content_hash: pulumi_wasm_rust::Output<String>,
        /// The content URI where the Guest Configuration package is stored.
        pub content_uri: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Date and time, in RFC3339 format, when the machines compliance status was last checked.
        pub last_compliance_status_checked: pulumi_wasm_rust::Output<String>,
        /// The ID of the latest report for the guest configuration assignment.
        pub latest_report_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub virtual_machine_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetVirtualMachineConfigurationAssignmentArgs,
    ) -> GetVirtualMachineConfigurationAssignmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let virtual_machine_name_binding = args.virtual_machine_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:policy/getVirtualMachineConfigurationAssignment:getVirtualMachineConfigurationAssignment"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "virtualMachineName".into(),
                    value: &virtual_machine_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "assignmentHash".into(),
                },
                register_interface::ResultField {
                    name: "complianceStatus".into(),
                },
                register_interface::ResultField {
                    name: "contentHash".into(),
                },
                register_interface::ResultField {
                    name: "contentUri".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lastComplianceStatusChecked".into(),
                },
                register_interface::ResultField {
                    name: "latestReportId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "virtualMachineName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVirtualMachineConfigurationAssignmentResult {
            assignment_hash: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assignmentHash").unwrap(),
            ),
            compliance_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("complianceStatus").unwrap(),
            ),
            content_hash: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentHash").unwrap(),
            ),
            content_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentUri").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            last_compliance_status_checked: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastComplianceStatusChecked").unwrap(),
            ),
            latest_report_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestReportId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            virtual_machine_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualMachineName").unwrap(),
            ),
        }
    }
}
