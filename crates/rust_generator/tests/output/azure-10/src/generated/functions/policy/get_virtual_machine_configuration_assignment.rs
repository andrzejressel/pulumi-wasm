#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_virtual_machine_configuration_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualMachineConfigurationAssignmentArgs {
        /// Specifies the name of the Guest Configuration Assignment.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Name of the Resource Group where the Guest Configuration Assignment exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Only retrieve Policy Set Definitions from this Management Group.
        #[builder(into)]
        pub virtual_machine_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualMachineConfigurationAssignmentResult {
        /// Combined hash of the configuration package and parameters.
        pub assignment_hash: pulumi_gestalt_rust::Output<String>,
        /// A value indicating compliance status of the machine for the assigned guest configuration. Possible return values are `Compliant`, `NonCompliant` and `Pending`.
        pub compliance_status: pulumi_gestalt_rust::Output<String>,
        /// The content hash for the Guest Configuration package.
        pub content_hash: pulumi_gestalt_rust::Output<String>,
        /// The content URI where the Guest Configuration package is stored.
        pub content_uri: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Date and time, in RFC3339 format, when the machines compliance status was last checked.
        pub last_compliance_status_checked: pulumi_gestalt_rust::Output<String>,
        /// The ID of the latest report for the guest configuration assignment.
        pub latest_report_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub virtual_machine_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVirtualMachineConfigurationAssignmentArgs,
    ) -> GetVirtualMachineConfigurationAssignmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let virtual_machine_name_binding = args.virtual_machine_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:policy/getVirtualMachineConfigurationAssignment:getVirtualMachineConfigurationAssignment"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualMachineName".into(),
                    value: virtual_machine_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVirtualMachineConfigurationAssignmentResult {
            assignment_hash: o.get_field("assignmentHash"),
            compliance_status: o.get_field("complianceStatus"),
            content_hash: o.get_field("contentHash"),
            content_uri: o.get_field("contentUri"),
            id: o.get_field("id"),
            last_compliance_status_checked: o.get_field("lastComplianceStatusChecked"),
            latest_report_id: o.get_field("latestReportId"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            virtual_machine_name: o.get_field("virtualMachineName"),
        }
    }
}
