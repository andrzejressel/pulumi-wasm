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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_machine {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineArgs {
        /// An `assessment` block as defined below.
        #[builder(into, default)]
        pub assessment: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mssql::VirtualMachineAssessment>,
        >,
        /// An `auto_backup` block as defined below. This block can be added to an existing resource, but removing this block forces a new resource to be created.
        #[builder(into, default)]
        pub auto_backup: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mssql::VirtualMachineAutoBackup>,
        >,
        /// An `auto_patching` block as defined below.
        #[builder(into, default)]
        pub auto_patching: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mssql::VirtualMachineAutoPatching>,
        >,
        /// An `key_vault_credential` block as defined below.
        #[builder(into, default)]
        pub key_vault_credential: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mssql::VirtualMachineKeyVaultCredential>,
        >,
        /// Should R Services be enabled?
        #[builder(into, default)]
        pub r_services_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The SQL Server port. Defaults to `1433`.
        #[builder(into, default)]
        pub sql_connectivity_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The connectivity type used for this SQL Server. Possible values are `LOCAL`, `PRIVATE` and `PUBLIC`. Defaults to `PRIVATE`.
        #[builder(into, default)]
        pub sql_connectivity_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The SQL Server sysadmin login password.
        #[builder(into, default)]
        pub sql_connectivity_update_password: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The SQL Server sysadmin login to create.
        #[builder(into, default)]
        pub sql_connectivity_update_username: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A `sql_instance` block as defined below.
        #[builder(into, default)]
        pub sql_instance: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mssql::VirtualMachineSqlInstance>,
        >,
        /// The SQL Server license type. Possible values are `AHUB` (Azure Hybrid Benefit), `DR` (Disaster Recovery), and `PAYG` (Pay-As-You-Go). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub sql_license_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the SQL Virtual Machine Group that the SQL Virtual Machine belongs to.
        #[builder(into, default)]
        pub sql_virtual_machine_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// An `storage_configuration` block as defined below.
        #[builder(into, default)]
        pub storage_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mssql::VirtualMachineStorageConfiguration>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_machine_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `wsfc_domain_credential` block as defined below
        #[builder(into, default)]
        pub wsfc_domain_credential: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mssql::VirtualMachineWsfcDomainCredential>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineResult {
        /// An `assessment` block as defined below.
        pub assessment: pulumi_gestalt_rust::Output<
            Option<super::super::types::mssql::VirtualMachineAssessment>,
        >,
        /// An `auto_backup` block as defined below. This block can be added to an existing resource, but removing this block forces a new resource to be created.
        pub auto_backup: pulumi_gestalt_rust::Output<
            Option<super::super::types::mssql::VirtualMachineAutoBackup>,
        >,
        /// An `auto_patching` block as defined below.
        pub auto_patching: pulumi_gestalt_rust::Output<
            Option<super::super::types::mssql::VirtualMachineAutoPatching>,
        >,
        /// An `key_vault_credential` block as defined below.
        pub key_vault_credential: pulumi_gestalt_rust::Output<
            Option<super::super::types::mssql::VirtualMachineKeyVaultCredential>,
        >,
        /// Should R Services be enabled?
        pub r_services_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The SQL Server port. Defaults to `1433`.
        pub sql_connectivity_port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The connectivity type used for this SQL Server. Possible values are `LOCAL`, `PRIVATE` and `PUBLIC`. Defaults to `PRIVATE`.
        pub sql_connectivity_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The SQL Server sysadmin login password.
        pub sql_connectivity_update_password: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The SQL Server sysadmin login to create.
        pub sql_connectivity_update_username: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// A `sql_instance` block as defined below.
        pub sql_instance: pulumi_gestalt_rust::Output<
            Option<super::super::types::mssql::VirtualMachineSqlInstance>,
        >,
        /// The SQL Server license type. Possible values are `AHUB` (Azure Hybrid Benefit), `DR` (Disaster Recovery), and `PAYG` (Pay-As-You-Go). Changing this forces a new resource to be created.
        pub sql_license_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the SQL Virtual Machine Group that the SQL Virtual Machine belongs to.
        pub sql_virtual_machine_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `storage_configuration` block as defined below.
        pub storage_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::mssql::VirtualMachineStorageConfiguration>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Machine. Changing this forces a new resource to be created.
        pub virtual_machine_id: pulumi_gestalt_rust::Output<String>,
        /// A `wsfc_domain_credential` block as defined below
        pub wsfc_domain_credential: pulumi_gestalt_rust::Output<
            Option<super::super::types::mssql::VirtualMachineWsfcDomainCredential>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualMachineArgs,
    ) -> VirtualMachineResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let assessment_binding = args.assessment.get_output(context);
        let auto_backup_binding = args.auto_backup.get_output(context);
        let auto_patching_binding = args.auto_patching.get_output(context);
        let key_vault_credential_binding = args.key_vault_credential.get_output(context);
        let r_services_enabled_binding = args.r_services_enabled.get_output(context);
        let sql_connectivity_port_binding = args
            .sql_connectivity_port
            .get_output(context);
        let sql_connectivity_type_binding = args
            .sql_connectivity_type
            .get_output(context);
        let sql_connectivity_update_password_binding = args
            .sql_connectivity_update_password
            .get_output(context);
        let sql_connectivity_update_username_binding = args
            .sql_connectivity_update_username
            .get_output(context);
        let sql_instance_binding = args.sql_instance.get_output(context);
        let sql_license_type_binding = args.sql_license_type.get_output(context);
        let sql_virtual_machine_group_id_binding = args
            .sql_virtual_machine_group_id
            .get_output(context);
        let storage_configuration_binding = args
            .storage_configuration
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let virtual_machine_id_binding = args.virtual_machine_id.get_output(context);
        let wsfc_domain_credential_binding = args
            .wsfc_domain_credential
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mssql/virtualMachine:VirtualMachine".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assessment".into(),
                    value: assessment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoBackup".into(),
                    value: auto_backup_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoPatching".into(),
                    value: auto_patching_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultCredential".into(),
                    value: key_vault_credential_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rServicesEnabled".into(),
                    value: r_services_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlConnectivityPort".into(),
                    value: sql_connectivity_port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlConnectivityType".into(),
                    value: sql_connectivity_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlConnectivityUpdatePassword".into(),
                    value: sql_connectivity_update_password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlConnectivityUpdateUsername".into(),
                    value: sql_connectivity_update_username_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlInstance".into(),
                    value: sql_instance_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlLicenseType".into(),
                    value: sql_license_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlVirtualMachineGroupId".into(),
                    value: sql_virtual_machine_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageConfiguration".into(),
                    value: storage_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualMachineId".into(),
                    value: virtual_machine_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "wsfcDomainCredential".into(),
                    value: wsfc_domain_credential_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualMachineResult {
            assessment: o.get_field("assessment"),
            auto_backup: o.get_field("autoBackup"),
            auto_patching: o.get_field("autoPatching"),
            key_vault_credential: o.get_field("keyVaultCredential"),
            r_services_enabled: o.get_field("rServicesEnabled"),
            sql_connectivity_port: o.get_field("sqlConnectivityPort"),
            sql_connectivity_type: o.get_field("sqlConnectivityType"),
            sql_connectivity_update_password: o
                .get_field("sqlConnectivityUpdatePassword"),
            sql_connectivity_update_username: o
                .get_field("sqlConnectivityUpdateUsername"),
            sql_instance: o.get_field("sqlInstance"),
            sql_license_type: o.get_field("sqlLicenseType"),
            sql_virtual_machine_group_id: o.get_field("sqlVirtualMachineGroupId"),
            storage_configuration: o.get_field("storageConfiguration"),
            tags: o.get_field("tags"),
            virtual_machine_id: o.get_field("virtualMachineId"),
            wsfc_domain_credential: o.get_field("wsfcDomainCredential"),
        }
    }
}
