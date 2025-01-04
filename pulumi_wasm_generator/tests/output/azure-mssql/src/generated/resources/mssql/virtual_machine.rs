/// Manages a Microsoft SQL Virtual Machine
///
/// ## Example Usage
///
/// This example provisions a brief Managed Microsoft SQL Virtual Machine.
///
/// ```yaml
/// resources:
///   exampleVirtualMachine:
///     type: azure:mssql:VirtualMachine
///     name: example
///     properties:
///       virtualMachineId: ${example.id}
///       sqlLicenseType: PAYG
///       rServicesEnabled: true
///       sqlConnectivityPort: 1433
///       sqlConnectivityType: PRIVATE
///       sqlConnectivityUpdatePassword: Password1234!
///       sqlConnectivityUpdateUsername: sqllogin
///       autoPatching:
///         dayOfWeek: Sunday
///         maintenanceWindowDurationInMinutes: 60
///         maintenanceWindowStartingHour: 2
/// variables:
///   example:
///     fn::invoke:
///       function: azure:compute:getVirtualMachine
///       arguments:
///         name: example-vm
///         resourceGroupName: example-resources
/// ```
///
/// ## Import
///
/// Microsoft SQL Virtual Machines can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/virtualMachine:VirtualMachine example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachines/example1
/// ```
///
pub mod virtual_machine {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineArgs {
        /// An `assessment` block as defined below.
        #[builder(into, default)]
        pub assessment: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::VirtualMachineAssessment>,
        >,
        /// An `auto_backup` block as defined below. This block can be added to an existing resource, but removing this block forces a new resource to be created.
        #[builder(into, default)]
        pub auto_backup: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::VirtualMachineAutoBackup>,
        >,
        /// An `auto_patching` block as defined below.
        #[builder(into, default)]
        pub auto_patching: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::VirtualMachineAutoPatching>,
        >,
        /// An `key_vault_credential` block as defined below.
        #[builder(into, default)]
        pub key_vault_credential: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::VirtualMachineKeyVaultCredential>,
        >,
        /// Should R Services be enabled?
        #[builder(into, default)]
        pub r_services_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The SQL Server port. Defaults to `1433`.
        #[builder(into, default)]
        pub sql_connectivity_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The connectivity type used for this SQL Server. Possible values are `LOCAL`, `PRIVATE` and `PUBLIC`. Defaults to `PRIVATE`.
        #[builder(into, default)]
        pub sql_connectivity_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The SQL Server sysadmin login password.
        #[builder(into, default)]
        pub sql_connectivity_update_password: pulumi_wasm_rust::Output<Option<String>>,
        /// The SQL Server sysadmin login to create.
        #[builder(into, default)]
        pub sql_connectivity_update_username: pulumi_wasm_rust::Output<Option<String>>,
        /// A `sql_instance` block as defined below.
        #[builder(into, default)]
        pub sql_instance: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::VirtualMachineSqlInstance>,
        >,
        /// The SQL Server license type. Possible values are `AHUB` (Azure Hybrid Benefit), `DR` (Disaster Recovery), and `PAYG` (Pay-As-You-Go). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub sql_license_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the SQL Virtual Machine Group that the SQL Virtual Machine belongs to.
        #[builder(into, default)]
        pub sql_virtual_machine_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// An `storage_configuration` block as defined below.
        #[builder(into, default)]
        pub storage_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::VirtualMachineStorageConfiguration>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_machine_id: pulumi_wasm_rust::Output<String>,
        /// A `wsfc_domain_credential` block as defined below
        #[builder(into, default)]
        pub wsfc_domain_credential: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::VirtualMachineWsfcDomainCredential>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineResult {
        /// An `assessment` block as defined below.
        pub assessment: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::VirtualMachineAssessment>,
        >,
        /// An `auto_backup` block as defined below. This block can be added to an existing resource, but removing this block forces a new resource to be created.
        pub auto_backup: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::VirtualMachineAutoBackup>,
        >,
        /// An `auto_patching` block as defined below.
        pub auto_patching: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::VirtualMachineAutoPatching>,
        >,
        /// An `key_vault_credential` block as defined below.
        pub key_vault_credential: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::VirtualMachineKeyVaultCredential>,
        >,
        /// Should R Services be enabled?
        pub r_services_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The SQL Server port. Defaults to `1433`.
        pub sql_connectivity_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The connectivity type used for this SQL Server. Possible values are `LOCAL`, `PRIVATE` and `PUBLIC`. Defaults to `PRIVATE`.
        pub sql_connectivity_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The SQL Server sysadmin login password.
        pub sql_connectivity_update_password: pulumi_wasm_rust::Output<Option<String>>,
        /// The SQL Server sysadmin login to create.
        pub sql_connectivity_update_username: pulumi_wasm_rust::Output<Option<String>>,
        /// A `sql_instance` block as defined below.
        pub sql_instance: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::VirtualMachineSqlInstance>,
        >,
        /// The SQL Server license type. Possible values are `AHUB` (Azure Hybrid Benefit), `DR` (Disaster Recovery), and `PAYG` (Pay-As-You-Go). Changing this forces a new resource to be created.
        pub sql_license_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the SQL Virtual Machine Group that the SQL Virtual Machine belongs to.
        pub sql_virtual_machine_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// An `storage_configuration` block as defined below.
        pub storage_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::VirtualMachineStorageConfiguration>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Machine. Changing this forces a new resource to be created.
        pub virtual_machine_id: pulumi_wasm_rust::Output<String>,
        /// A `wsfc_domain_credential` block as defined below
        pub wsfc_domain_credential: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::VirtualMachineWsfcDomainCredential>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VirtualMachineArgs) -> VirtualMachineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let assessment_binding = args.assessment.get_inner();
        let auto_backup_binding = args.auto_backup.get_inner();
        let auto_patching_binding = args.auto_patching.get_inner();
        let key_vault_credential_binding = args.key_vault_credential.get_inner();
        let r_services_enabled_binding = args.r_services_enabled.get_inner();
        let sql_connectivity_port_binding = args.sql_connectivity_port.get_inner();
        let sql_connectivity_type_binding = args.sql_connectivity_type.get_inner();
        let sql_connectivity_update_password_binding = args
            .sql_connectivity_update_password
            .get_inner();
        let sql_connectivity_update_username_binding = args
            .sql_connectivity_update_username
            .get_inner();
        let sql_instance_binding = args.sql_instance.get_inner();
        let sql_license_type_binding = args.sql_license_type.get_inner();
        let sql_virtual_machine_group_id_binding = args
            .sql_virtual_machine_group_id
            .get_inner();
        let storage_configuration_binding = args.storage_configuration.get_inner();
        let tags_binding = args.tags.get_inner();
        let virtual_machine_id_binding = args.virtual_machine_id.get_inner();
        let wsfc_domain_credential_binding = args.wsfc_domain_credential.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/virtualMachine:VirtualMachine".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "assessment".into(),
                    value: &assessment_binding,
                },
                register_interface::ObjectField {
                    name: "autoBackup".into(),
                    value: &auto_backup_binding,
                },
                register_interface::ObjectField {
                    name: "autoPatching".into(),
                    value: &auto_patching_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultCredential".into(),
                    value: &key_vault_credential_binding,
                },
                register_interface::ObjectField {
                    name: "rServicesEnabled".into(),
                    value: &r_services_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "sqlConnectivityPort".into(),
                    value: &sql_connectivity_port_binding,
                },
                register_interface::ObjectField {
                    name: "sqlConnectivityType".into(),
                    value: &sql_connectivity_type_binding,
                },
                register_interface::ObjectField {
                    name: "sqlConnectivityUpdatePassword".into(),
                    value: &sql_connectivity_update_password_binding,
                },
                register_interface::ObjectField {
                    name: "sqlConnectivityUpdateUsername".into(),
                    value: &sql_connectivity_update_username_binding,
                },
                register_interface::ObjectField {
                    name: "sqlInstance".into(),
                    value: &sql_instance_binding,
                },
                register_interface::ObjectField {
                    name: "sqlLicenseType".into(),
                    value: &sql_license_type_binding,
                },
                register_interface::ObjectField {
                    name: "sqlVirtualMachineGroupId".into(),
                    value: &sql_virtual_machine_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageConfiguration".into(),
                    value: &storage_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "virtualMachineId".into(),
                    value: &virtual_machine_id_binding,
                },
                register_interface::ObjectField {
                    name: "wsfcDomainCredential".into(),
                    value: &wsfc_domain_credential_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "assessment".into(),
                },
                register_interface::ResultField {
                    name: "autoBackup".into(),
                },
                register_interface::ResultField {
                    name: "autoPatching".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultCredential".into(),
                },
                register_interface::ResultField {
                    name: "rServicesEnabled".into(),
                },
                register_interface::ResultField {
                    name: "sqlConnectivityPort".into(),
                },
                register_interface::ResultField {
                    name: "sqlConnectivityType".into(),
                },
                register_interface::ResultField {
                    name: "sqlConnectivityUpdatePassword".into(),
                },
                register_interface::ResultField {
                    name: "sqlConnectivityUpdateUsername".into(),
                },
                register_interface::ResultField {
                    name: "sqlInstance".into(),
                },
                register_interface::ResultField {
                    name: "sqlLicenseType".into(),
                },
                register_interface::ResultField {
                    name: "sqlVirtualMachineGroupId".into(),
                },
                register_interface::ResultField {
                    name: "storageConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "virtualMachineId".into(),
                },
                register_interface::ResultField {
                    name: "wsfcDomainCredential".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualMachineResult {
            assessment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assessment").unwrap(),
            ),
            auto_backup: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoBackup").unwrap(),
            ),
            auto_patching: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoPatching").unwrap(),
            ),
            key_vault_credential: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultCredential").unwrap(),
            ),
            r_services_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rServicesEnabled").unwrap(),
            ),
            sql_connectivity_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqlConnectivityPort").unwrap(),
            ),
            sql_connectivity_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqlConnectivityType").unwrap(),
            ),
            sql_connectivity_update_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqlConnectivityUpdatePassword").unwrap(),
            ),
            sql_connectivity_update_username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqlConnectivityUpdateUsername").unwrap(),
            ),
            sql_instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqlInstance").unwrap(),
            ),
            sql_license_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqlLicenseType").unwrap(),
            ),
            sql_virtual_machine_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqlVirtualMachineGroupId").unwrap(),
            ),
            storage_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageConfiguration").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            virtual_machine_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualMachineId").unwrap(),
            ),
            wsfc_domain_credential: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("wsfcDomainCredential").unwrap(),
            ),
        }
    }
}
