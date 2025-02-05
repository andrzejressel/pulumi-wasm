/// Manages a Virtual Machine Run Command.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: internal
///       resourceGroupName: ${exampleResourceGroup.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleNetworkInterface:
///     type: azure:network:NetworkInterface
///     name: example
///     properties:
///       name: example-nic
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       ipConfigurations:
///         - name: internal
///           subnetId: ${exampleSubnet.id}
///           privateIpAddressAllocation: Dynamic
///   exampleUserAssignedIdentity:
///     type: azure:authorization:UserAssignedIdentity
///     name: example
///     properties:
///       name: example-uai
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///   exampleLinuxVirtualMachine:
///     type: azure:compute:LinuxVirtualMachine
///     name: example
///     properties:
///       name: example-VM
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       size: Standard_B2s
///       adminUsername: adminuser
///       adminPassword: P@$$w0rd1234!
///       disablePasswordAuthentication: false
///       networkInterfaceIds:
///         - ${exampleNetworkInterface.id}
///       osDisk:
///         caching: ReadWrite
///         storageAccountType: Premium_LRS
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///       identity:
///         type: SystemAssigned, UserAssigned
///         identityIds:
///           - ${exampleUserAssignedIdentity.id}
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${exampleAccount.id}
///       roleDefinitionName: Storage Blob Data Contributor
///       principalId: ${exampleUserAssignedIdentity.principalId}
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: exampleaccount
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: example-sc
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: blob
///   example1:
///     type: azure:storage:Blob
///     properties:
///       name: script1
///       storageAccountName: ${exampleAccount.name}
///       storageContainerName: ${exampleContainer.name}
///       type: Block
///       sourceContent: echo 'hello world'
///   example2:
///     type: azure:storage:Blob
///     properties:
///       name: output
///       storageAccountName: ${exampleAccount.name}
///       storageContainerName: ${exampleContainer.name}
///       type: Append
///   example3:
///     type: azure:storage:Blob
///     properties:
///       name: error
///       storageAccountName: ${exampleAccount.name}
///       storageContainerName: ${exampleContainer.name}
///       type: Append
///   # basic example
///   exampleRunCommand:
///     type: azure:compute:RunCommand
///     name: example
///     properties:
///       name: example-vmrc
///       location: ${exampleResourceGroup.location}
///       virtualMachineId: ${exampleLinuxVirtualMachine.id}
///       source:
///         script: echo 'hello world'
///   # authorize to storage blob using user assigned identity
///   example2RunCommand:
///     type: azure:compute:RunCommand
///     name: example2
///     properties:
///       location: ${exampleResourceGroup.location}
///       name: example2-vmrc
///       virtualMachineId: ${exampleLinuxVirtualMachine.id}
///       outputBlobUri: ${example2.id}
///       errorBlobUri: ${example3.id}
///       runAsPassword: P@$$w0rd1234!
///       runAsUser: adminuser
///       source:
///         scriptUri: ${example1.id}
///         scriptUriManagedIdentity:
///           clientId: ${exampleUserAssignedIdentity.clientId}
///       errorBlobManagedIdentity:
///         clientId: ${exampleUserAssignedIdentity.clientId}
///       outputBlobManagedIdentity:
///         clientId: ${exampleUserAssignedIdentity.clientId}
///       parameters:
///         - name: examplev1
///           value: val1
///       protectedParameters:
///         - name: examplev2
///           value: val2
///       tags:
///         environment: terraform-examples
///         some_key: some-value
///     options:
///       dependsOn:
///         - ${exampleAssignment}
///   # authorize to storage blob using SAS token
///   example3RunCommand:
///     type: azure:compute:RunCommand
///     name: example3
///     properties:
///       location: ${exampleResourceGroup.location}
///       name: example3-vmrc
///       virtualMachineId: ${exampleLinuxVirtualMachine.id}
///       runAsPassword: P@$$w0rd1234!
///       runAsUser: adminuser
///       errorBlobUri: ${example3.id}${example.sas}
///       outputBlobUri: ${example2.id}${example.sas}
///       source:
///         scriptUri: ${example1.id}${example.sas}
///       parameters:
///         - name: example-vm1
///           value: val1
///       tags:
///         environment: terraform-example-s
///         some_key: some-value
/// variables:
///   example:
///     fn::invoke:
///       function: azure:storage:getAccountSAS
///       arguments:
///         connectionString: ${exampleAccount.primaryConnectionString}
///         httpsOnly: true
///         signedVersion: 2019-10-10
///         start: 2023-04-01T00:00:00Z
///         expiry: 2024-04-01T00:00:00Z
///         resourceTypes:
///           service: false
///           container: false
///           object: true
///         services:
///           blob: true
///           queue: false
///           table: false
///           file: false
///         permissions:
///           read: true
///           write: true
///           delete: false
///           list: false
///           add: true
///           create: true
///           update: false
///           process: false
///           tag: false
///           filter: false
/// ```
///
/// ## Import
///
/// An existing Virtual Machine Run Command can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/runCommand:RunCommand example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/virtualMachines/vm1/runCommands/rc1
/// ```
///
pub mod run_command {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RunCommandArgs {
        /// An `error_blob_managed_identity` block as defined below. User-assigned managed Identity that has access to errorBlobUri storage blob.
        #[builder(into, default)]
        pub error_blob_managed_identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::RunCommandErrorBlobManagedIdentity>,
        >,
        /// Specifies the Azure storage blob where script error stream will be uploaded.
        #[builder(into, default)]
        pub error_blob_uri: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Azure Region where the Virtual Machine Run Command should exist. Changing this forces a new Virtual Machine Run Command to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of this Virtual Machine Run Command. Changing this forces a new Virtual Machine Run Command to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// An `output_blob_managed_identity` block as defined below. User-assigned managed Identity that has access to outputBlobUri storage blob.
        #[builder(into, default)]
        pub output_blob_managed_identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::RunCommandOutputBlobManagedIdentity>,
        >,
        /// Specifies the Azure storage blob where script output stream will be uploaded. It can be basic blob URI with SAS token.
        #[builder(into, default)]
        pub output_blob_uri: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of `parameter` blocks as defined below. The parameters used by the script.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::RunCommandParameter>>,
        >,
        /// A list of `protected_parameter` blocks as defined below. The protected parameters used by the script.
        #[builder(into, default)]
        pub protected_parameters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::RunCommandProtectedParameter>>,
        >,
        /// Specifies the user account password on the VM when executing the Virtual Machine Run Command.
        #[builder(into, default)]
        pub run_as_password: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the user account on the VM when executing the Virtual Machine Run Command.
        #[builder(into, default)]
        pub run_as_user: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `source` block as defined below. The source of the run command script.
        #[builder(into)]
        pub source: pulumi_wasm_rust::InputOrOutput<
            super::super::types::compute::RunCommandSource,
        >,
        /// A mapping of tags which should be assigned to the Virtual Machine Run Command.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Virtual Machine ID within which this Virtual Machine Run Command should exist. Changing this forces a new Virtual Machine Run Command to be created.
        #[builder(into)]
        pub virtual_machine_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RunCommandResult {
        /// An `error_blob_managed_identity` block as defined below. User-assigned managed Identity that has access to errorBlobUri storage blob.
        pub error_blob_managed_identity: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::RunCommandErrorBlobManagedIdentity>,
        >,
        /// Specifies the Azure storage blob where script error stream will be uploaded.
        pub error_blob_uri: pulumi_wasm_rust::Output<Option<String>>,
        pub instance_views: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::RunCommandInstanceView>,
        >,
        /// The Azure Region where the Virtual Machine Run Command should exist. Changing this forces a new Virtual Machine Run Command to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Virtual Machine Run Command. Changing this forces a new Virtual Machine Run Command to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// An `output_blob_managed_identity` block as defined below. User-assigned managed Identity that has access to outputBlobUri storage blob.
        pub output_blob_managed_identity: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::RunCommandOutputBlobManagedIdentity>,
        >,
        /// Specifies the Azure storage blob where script output stream will be uploaded. It can be basic blob URI with SAS token.
        pub output_blob_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of `parameter` blocks as defined below. The parameters used by the script.
        pub parameters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::RunCommandParameter>>,
        >,
        /// A list of `protected_parameter` blocks as defined below. The protected parameters used by the script.
        pub protected_parameters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::RunCommandProtectedParameter>>,
        >,
        /// Specifies the user account password on the VM when executing the Virtual Machine Run Command.
        pub run_as_password: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the user account on the VM when executing the Virtual Machine Run Command.
        pub run_as_user: pulumi_wasm_rust::Output<Option<String>>,
        /// A `source` block as defined below. The source of the run command script.
        pub source: pulumi_wasm_rust::Output<
            super::super::types::compute::RunCommandSource,
        >,
        /// A mapping of tags which should be assigned to the Virtual Machine Run Command.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Virtual Machine ID within which this Virtual Machine Run Command should exist. Changing this forces a new Virtual Machine Run Command to be created.
        pub virtual_machine_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RunCommandArgs,
    ) -> RunCommandResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let error_blob_managed_identity_binding = args
            .error_blob_managed_identity
            .get_output(context)
            .get_inner();
        let error_blob_uri_binding = args.error_blob_uri.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let output_blob_managed_identity_binding = args
            .output_blob_managed_identity
            .get_output(context)
            .get_inner();
        let output_blob_uri_binding = args
            .output_blob_uri
            .get_output(context)
            .get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let protected_parameters_binding = args
            .protected_parameters
            .get_output(context)
            .get_inner();
        let run_as_password_binding = args
            .run_as_password
            .get_output(context)
            .get_inner();
        let run_as_user_binding = args.run_as_user.get_output(context).get_inner();
        let source_binding = args.source.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let virtual_machine_id_binding = args
            .virtual_machine_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/runCommand:RunCommand".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "errorBlobManagedIdentity".into(),
                    value: &error_blob_managed_identity_binding,
                },
                register_interface::ObjectField {
                    name: "errorBlobUri".into(),
                    value: &error_blob_uri_binding,
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
                    name: "outputBlobManagedIdentity".into(),
                    value: &output_blob_managed_identity_binding,
                },
                register_interface::ObjectField {
                    name: "outputBlobUri".into(),
                    value: &output_blob_uri_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "protectedParameters".into(),
                    value: &protected_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "runAsPassword".into(),
                    value: &run_as_password_binding,
                },
                register_interface::ObjectField {
                    name: "runAsUser".into(),
                    value: &run_as_user_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "virtualMachineId".into(),
                    value: &virtual_machine_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RunCommandResult {
            error_blob_managed_identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("errorBlobManagedIdentity"),
            ),
            error_blob_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("errorBlobUri"),
            ),
            instance_views: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceViews"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            output_blob_managed_identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outputBlobManagedIdentity"),
            ),
            output_blob_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outputBlobUri"),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            protected_parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("protectedParameters"),
            ),
            run_as_password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("runAsPassword"),
            ),
            run_as_user: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("runAsUser"),
            ),
            source: pulumi_wasm_rust::__private::into_domain(o.extract_field("source")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            virtual_machine_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualMachineId"),
            ),
        }
    }
}
