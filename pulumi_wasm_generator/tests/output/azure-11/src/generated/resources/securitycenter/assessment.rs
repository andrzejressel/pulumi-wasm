/// Manages the Security Center Assessment for Azure Security Center.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-network
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       addressSpaces:
///         - 10.0.0.0/16
///   internal:
///     type: azure:network:Subnet
///     properties:
///       name: internal
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleLinuxVirtualMachineScaleSet:
///     type: azure:compute:LinuxVirtualMachineScaleSet
///     name: example
///     properties:
///       name: example-vmss
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: Standard_F2
///       instances: 1
///       adminUsername: adminuser
///       adminSshKeys:
///         - username: adminuser
///           publicKey:
///             fn::invoke:
///               function: std:file
///               arguments:
///                 input: ~/.ssh/id_rsa.pub
///               return: result
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///       osDisk:
///         storageAccountType: Standard_LRS
///         caching: ReadWrite
///       networkInterfaces:
///         - name: example
///           primary: true
///           ipConfigurations:
///             - name: internal
///               primary: true
///               subnetId: ${internal.id}
///   exampleAssessmentPolicy:
///     type: azure:securitycenter:AssessmentPolicy
///     name: example
///     properties:
///       displayName: Test Display Name
///       severity: Medium
///       description: Test Description
///   exampleAssessment:
///     type: azure:securitycenter:Assessment
///     name: example
///     properties:
///       assessmentPolicyId: ${exampleAssessmentPolicy.id}
///       targetResourceId: ${exampleLinuxVirtualMachineScaleSet.id}
///       status:
///         code: Healthy
/// ```
///
/// ## Import
///
/// Security Assessment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:securitycenter/assessment:Assessment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.Compute/virtualMachineScaleSets/vmss1/providers/Microsoft.Security/assessments/00000000-0000-0000-0000-000000000000
/// ```
///
pub mod assessment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssessmentArgs {
        /// A map of additional data to associate with the assessment.
        #[builder(into, default)]
        pub additional_data: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the security Assessment policy to apply to this resource. Changing this forces a new security Assessment to be created.
        #[builder(into)]
        pub assessment_policy_id: pulumi_wasm_rust::Output<String>,
        /// A `status` block as defined below.
        #[builder(into)]
        pub status: pulumi_wasm_rust::Output<
            super::super::types::securitycenter::AssessmentStatus,
        >,
        /// The ID of the target resource. Changing this forces a new security Assessment to be created.
        #[builder(into)]
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AssessmentResult {
        /// A map of additional data to associate with the assessment.
        pub additional_data: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the security Assessment policy to apply to this resource. Changing this forces a new security Assessment to be created.
        pub assessment_policy_id: pulumi_wasm_rust::Output<String>,
        /// A `status` block as defined below.
        pub status: pulumi_wasm_rust::Output<
            super::super::types::securitycenter::AssessmentStatus,
        >,
        /// The ID of the target resource. Changing this forces a new security Assessment to be created.
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AssessmentArgs) -> AssessmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_data_binding = args.additional_data.get_inner();
        let assessment_policy_id_binding = args.assessment_policy_id.get_inner();
        let status_binding = args.status.get_inner();
        let target_resource_id_binding = args.target_resource_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:securitycenter/assessment:Assessment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalData".into(),
                    value: &additional_data_binding,
                },
                register_interface::ObjectField {
                    name: "assessmentPolicyId".into(),
                    value: &assessment_policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalData".into(),
                },
                register_interface::ResultField {
                    name: "assessmentPolicyId".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "targetResourceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AssessmentResult {
            additional_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalData").unwrap(),
            ),
            assessment_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assessmentPolicyId").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            target_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetResourceId").unwrap(),
            ),
        }
    }
}
