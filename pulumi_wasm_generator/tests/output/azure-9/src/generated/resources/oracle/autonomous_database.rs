/// Manages an Autonomous Database.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:oracle:AutonomousDatabase
///     properties:
///       name: example
///       resourceGroupName: example
///       location: West Europe
///       subnetId: example
///       displayName: example
///       dbWorkload: example
///       mtlsConnectionRequired: false
///       backupRetentionPeriodInDays: 42
///       computeModel: example
///       dataStorageSizeInGbs: 42
///       autoScalingForStorageEnabled: false
///       virtualNetworkId: example
///       adminPassword: example
///       autoScalingEnabled: example
///       characterSet: example
///       computeCount: 1.23456
///       nationalCharacterSet: example
///       licenseModel: false
///       dbVersion: example
/// ```
///
/// ## Import
///
/// Autonomous Databases can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:oracle/autonomousDatabase:AutonomousDatabase example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup/providers/Oracle.Database/autonomousDatabases/autonomousDatabases1
/// ```
///
pub mod autonomous_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutonomousDatabaseArgs {
        /// The password must be between `12` and `30 `characters long, and must contain at least 1 uppercase, 1 lowercase, and 1 numeric character. It cannot contain the double quote symbol (") or the username "admin", regardless of casing. Changing this forces a new Autonomous Database to be created.
        #[builder(into)]
        pub admin_password: pulumi_wasm_rust::InputOrOutput<String>,
        /// Indicates if auto scaling is enabled for the Autonomous Database CPU core count. The default value is `true`.
        #[builder(into)]
        pub auto_scaling_enabled: pulumi_wasm_rust::InputOrOutput<bool>,
        /// Indicates if auto scaling is enabled for the Autonomous Database storage. The default value is `false`.
        #[builder(into)]
        pub auto_scaling_for_storage_enabled: pulumi_wasm_rust::InputOrOutput<bool>,
        /// (Updatable) Retention period, in days, for backups. Changing this forces a new Autonomous Database to be created.
        #[builder(into)]
        pub backup_retention_period_in_days: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The character set for the autonomous database.  The default is `AL32UTF8`. Allowed values are:  `AL32UTF8`, `AR8ADOS710`, `AR8ADOS720`, `AR8APTEC715`, `AR8ARABICMACS`, `AR8ASMO8X`, `AR8ISO8859P6`, `AR8MSWIN1256`, `AR8MUSSAD768`, `AR8NAFITHA711`, `AR8NAFITHA721`, `AR8SAKHR706`, `AR8SAKHR707`, `AZ8ISO8859P9E`, `BG8MSWIN`, `BG8PC437S`, `BLT8CP921`, `BLT8ISO8859P13`, `BLT8MSWIN1257`, `BLT8PC775`, `BN8BSCII`, `CDN8PC863`, `CEL8ISO8859P14`, `CL8ISO8859P5`, `CL8ISOIR111`, `CL8KOI8R`, `CL8KOI8U`, `CL8MACCYRILLICS`, `CL8MSWIN1251`, `EE8ISO8859P2`, `EE8MACCES`, `EE8MACCROATIANS`, `EE8MSWIN1250`, `EE8PC852`, `EL8DEC`, `EL8ISO8859P7`, `EL8MACGREEKS`, `EL8MSWIN1253`, `EL8PC437S`, `EL8PC851`, `EL8PC869`, `ET8MSWIN923`, `HU8ABMOD`, `HU8CWI2`, `IN8ISCII`, `IS8PC861`, `IW8ISO8859P8`, `IW8MACHEBREWS`, `IW8MSWIN1255`, `IW8PC1507`, `JA16EUC`, `JA16EUCTILDE`, `JA16SJIS`, `JA16SJISTILDE`, `JA16VMS`, `KO16KSC5601`, `KO16KSCCS`, `KO16MSWIN949`, `LA8ISO6937`, `LA8PASSPORT`, `LT8MSWIN921`, `LT8PC772`, `LT8PC774`, `LV8PC1117`, `LV8PC8LR`, `LV8RST104090`, `N8PC865`, `NE8ISO8859P10`, `NEE8ISO8859P4`, `RU8BESTA`, `RU8PC855`, `RU8PC866`, `SE8ISO8859P3`, `TH8MACTHAIS`, `TH8TISASCII`, `TR8DEC`, `TR8MACTURKISHS`, `TR8MSWIN1254`, `TR8PC857`, `US7ASCII`, `US8PC437`, `UTF8`, `VN8MSWIN1258`, `VN8VN3`, `WE8DEC`, `WE8DG`, `WE8ISO8859P1`, `WE8ISO8859P15`, `WE8ISO8859P9`, `WE8MACROMAN8S`, `WE8MSWIN1252`, `WE8NCR4970`, `WE8NEXTSTEP`, `WE8PC850`, `WE8PC858`, `WE8PC860`, `WE8ROMAN8`, `ZHS16CGB231280`, `ZHS16GBK`, `ZHT16BIG5`, `ZHT16CCDC`, `ZHT16DBT`, `ZHT16HKSCS`, `ZHT16MSWIN950`, `ZHT32EUC`, `ZHT32SOPS`, `ZHT32TRIS`. Changing this forces a new Autonomous Database to be created
        #[builder(into)]
        pub character_set: pulumi_wasm_rust::InputOrOutput<String>,
        /// The compute amount (CPUs) available to the database. Minimum and maximum values depend on the compute model and whether the database is an Autonomous Database Serverless instance or an Autonomous Database on Dedicated Exadata Infrastructure.  For an Autonomous Database Serverless instance, the `ECPU` compute model requires a minimum value of one, for databases in the elastic resource pool and minimum value of two, otherwise. Required when using the `computeModel` parameter. When using `cpuCoreCount` parameter, it is an error to specify computeCount to a non-null value. Providing `computeModel` and `computeCount` is the preferred method for both OCPU and ECPU.
        #[builder(into)]
        pub compute_count: pulumi_wasm_rust::InputOrOutput<f64>,
        /// The compute model of the Autonomous Database. This is required if using the `computeCount` parameter. If using `cpuCoreCount` then it is an error to specify `computeModel` to a non-null value. ECPU compute model is the recommended model and OCPU compute model is legacy. Changing this forces a new Autonomous Database to be created.
        #[builder(into)]
        pub compute_model: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies a list of customer contacts as email addresses. Changing this forces a new Autonomous Database to be created.
        #[builder(into, default)]
        pub customer_contacts: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The maximum storage that can be allocated for the database, in terabytes.
        #[builder(into)]
        pub data_storage_size_in_tbs: pulumi_wasm_rust::InputOrOutput<i32>,
        /// A valid Oracle Database version for Autonomous Database. Changing this forces a new Autonomous Database to be created.
        #[builder(into)]
        pub db_version: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Autonomous Database workload type. Changing this forces a new Autonomous Database to be created. The following values are valid:
        /// * OLTP - indicates an Autonomous Transaction Processing database
        /// * DW - indicates an Autonomous Data Warehouse database
        /// * AJD - indicates an Autonomous JSON Database
        /// * APEX - indicates an Autonomous Database with the Oracle APEX Application Development workload type.
        #[builder(into)]
        pub db_workload: pulumi_wasm_rust::InputOrOutput<String>,
        /// The user-friendly name for the Autonomous Database. The name does not have to be unique. Changing this forces a new Autonomous Database to be created.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Oracle license model that applies to the Oracle Autonomous Database. Changing this forces a new Autonomous Database to be created. Bring your own license (BYOL) allows you to apply your current on-premises Oracle software licenses to equivalent, highly automated Oracle services in the cloud. License Included allows you to subscribe to new Oracle Database software licenses and the Oracle Database service. Note that when provisioning an [Autonomous Database on dedicated Exadata infrastructure](https://docs.oracle.com/en/cloud/paas/autonomous-database/index.html), this attribute must be null. It is already set at the Autonomous Exadata Infrastructure level. When provisioning an [Autonomous Database Serverless] (https://docs.oracle.com/en/cloud/paas/autonomous-database/index.html) database, if a value is not specified, the system defaults the value to `BRING_YOUR_OWN_LICENSE`. Bring your own license (BYOL) also allows you to select the DB edition using the optional parameter.
        #[builder(into)]
        pub license_model: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Azure Region where the Autonomous Database should exist. Changing this forces a new Autonomous Database to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies if the Autonomous Database requires mTLS connections. Changing this forces a new Autonomous Database to be created.
        #[builder(into)]
        pub mtls_connection_required: pulumi_wasm_rust::InputOrOutput<bool>,
        /// The name which should be used for this Autonomous Database. Changing this forces a new Autonomous Database to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The national character set for the autonomous database. Changing this forces a new Autonomous Database to be created. The default is AL16UTF16. Allowed values are: AL16UTF16 or UTF8.
        #[builder(into)]
        pub national_character_set: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Autonomous Database should exist. Changing this forces a new Autonomous Database to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the subnet the resource is associated with. Changing this forces a new Autonomous Database to be created.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Autonomous Database.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the vnet associated with the cloud VM cluster. Changing this forces a new Autonomous Database to be created.
        #[builder(into)]
        pub virtual_network_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AutonomousDatabaseResult {
        /// The password must be between `12` and `30 `characters long, and must contain at least 1 uppercase, 1 lowercase, and 1 numeric character. It cannot contain the double quote symbol (") or the username "admin", regardless of casing. Changing this forces a new Autonomous Database to be created.
        pub admin_password: pulumi_wasm_rust::Output<String>,
        /// Indicates if auto scaling is enabled for the Autonomous Database CPU core count. The default value is `true`.
        pub auto_scaling_enabled: pulumi_wasm_rust::Output<bool>,
        /// Indicates if auto scaling is enabled for the Autonomous Database storage. The default value is `false`.
        pub auto_scaling_for_storage_enabled: pulumi_wasm_rust::Output<bool>,
        /// (Updatable) Retention period, in days, for backups. Changing this forces a new Autonomous Database to be created.
        pub backup_retention_period_in_days: pulumi_wasm_rust::Output<i32>,
        /// The character set for the autonomous database.  The default is `AL32UTF8`. Allowed values are:  `AL32UTF8`, `AR8ADOS710`, `AR8ADOS720`, `AR8APTEC715`, `AR8ARABICMACS`, `AR8ASMO8X`, `AR8ISO8859P6`, `AR8MSWIN1256`, `AR8MUSSAD768`, `AR8NAFITHA711`, `AR8NAFITHA721`, `AR8SAKHR706`, `AR8SAKHR707`, `AZ8ISO8859P9E`, `BG8MSWIN`, `BG8PC437S`, `BLT8CP921`, `BLT8ISO8859P13`, `BLT8MSWIN1257`, `BLT8PC775`, `BN8BSCII`, `CDN8PC863`, `CEL8ISO8859P14`, `CL8ISO8859P5`, `CL8ISOIR111`, `CL8KOI8R`, `CL8KOI8U`, `CL8MACCYRILLICS`, `CL8MSWIN1251`, `EE8ISO8859P2`, `EE8MACCES`, `EE8MACCROATIANS`, `EE8MSWIN1250`, `EE8PC852`, `EL8DEC`, `EL8ISO8859P7`, `EL8MACGREEKS`, `EL8MSWIN1253`, `EL8PC437S`, `EL8PC851`, `EL8PC869`, `ET8MSWIN923`, `HU8ABMOD`, `HU8CWI2`, `IN8ISCII`, `IS8PC861`, `IW8ISO8859P8`, `IW8MACHEBREWS`, `IW8MSWIN1255`, `IW8PC1507`, `JA16EUC`, `JA16EUCTILDE`, `JA16SJIS`, `JA16SJISTILDE`, `JA16VMS`, `KO16KSC5601`, `KO16KSCCS`, `KO16MSWIN949`, `LA8ISO6937`, `LA8PASSPORT`, `LT8MSWIN921`, `LT8PC772`, `LT8PC774`, `LV8PC1117`, `LV8PC8LR`, `LV8RST104090`, `N8PC865`, `NE8ISO8859P10`, `NEE8ISO8859P4`, `RU8BESTA`, `RU8PC855`, `RU8PC866`, `SE8ISO8859P3`, `TH8MACTHAIS`, `TH8TISASCII`, `TR8DEC`, `TR8MACTURKISHS`, `TR8MSWIN1254`, `TR8PC857`, `US7ASCII`, `US8PC437`, `UTF8`, `VN8MSWIN1258`, `VN8VN3`, `WE8DEC`, `WE8DG`, `WE8ISO8859P1`, `WE8ISO8859P15`, `WE8ISO8859P9`, `WE8MACROMAN8S`, `WE8MSWIN1252`, `WE8NCR4970`, `WE8NEXTSTEP`, `WE8PC850`, `WE8PC858`, `WE8PC860`, `WE8ROMAN8`, `ZHS16CGB231280`, `ZHS16GBK`, `ZHT16BIG5`, `ZHT16CCDC`, `ZHT16DBT`, `ZHT16HKSCS`, `ZHT16MSWIN950`, `ZHT32EUC`, `ZHT32SOPS`, `ZHT32TRIS`. Changing this forces a new Autonomous Database to be created
        pub character_set: pulumi_wasm_rust::Output<String>,
        /// The compute amount (CPUs) available to the database. Minimum and maximum values depend on the compute model and whether the database is an Autonomous Database Serverless instance or an Autonomous Database on Dedicated Exadata Infrastructure.  For an Autonomous Database Serverless instance, the `ECPU` compute model requires a minimum value of one, for databases in the elastic resource pool and minimum value of two, otherwise. Required when using the `computeModel` parameter. When using `cpuCoreCount` parameter, it is an error to specify computeCount to a non-null value. Providing `computeModel` and `computeCount` is the preferred method for both OCPU and ECPU.
        pub compute_count: pulumi_wasm_rust::Output<f64>,
        /// The compute model of the Autonomous Database. This is required if using the `computeCount` parameter. If using `cpuCoreCount` then it is an error to specify `computeModel` to a non-null value. ECPU compute model is the recommended model and OCPU compute model is legacy. Changing this forces a new Autonomous Database to be created.
        pub compute_model: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of customer contacts as email addresses. Changing this forces a new Autonomous Database to be created.
        pub customer_contacts: pulumi_wasm_rust::Output<Vec<String>>,
        /// The maximum storage that can be allocated for the database, in terabytes.
        pub data_storage_size_in_tbs: pulumi_wasm_rust::Output<i32>,
        /// A valid Oracle Database version for Autonomous Database. Changing this forces a new Autonomous Database to be created.
        pub db_version: pulumi_wasm_rust::Output<String>,
        /// The Autonomous Database workload type. Changing this forces a new Autonomous Database to be created. The following values are valid:
        /// * OLTP - indicates an Autonomous Transaction Processing database
        /// * DW - indicates an Autonomous Data Warehouse database
        /// * AJD - indicates an Autonomous JSON Database
        /// * APEX - indicates an Autonomous Database with the Oracle APEX Application Development workload type.
        pub db_workload: pulumi_wasm_rust::Output<String>,
        /// The user-friendly name for the Autonomous Database. The name does not have to be unique. Changing this forces a new Autonomous Database to be created.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The Oracle license model that applies to the Oracle Autonomous Database. Changing this forces a new Autonomous Database to be created. Bring your own license (BYOL) allows you to apply your current on-premises Oracle software licenses to equivalent, highly automated Oracle services in the cloud. License Included allows you to subscribe to new Oracle Database software licenses and the Oracle Database service. Note that when provisioning an [Autonomous Database on dedicated Exadata infrastructure](https://docs.oracle.com/en/cloud/paas/autonomous-database/index.html), this attribute must be null. It is already set at the Autonomous Exadata Infrastructure level. When provisioning an [Autonomous Database Serverless] (https://docs.oracle.com/en/cloud/paas/autonomous-database/index.html) database, if a value is not specified, the system defaults the value to `BRING_YOUR_OWN_LICENSE`. Bring your own license (BYOL) also allows you to select the DB edition using the optional parameter.
        pub license_model: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Autonomous Database should exist. Changing this forces a new Autonomous Database to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies if the Autonomous Database requires mTLS connections. Changing this forces a new Autonomous Database to be created.
        pub mtls_connection_required: pulumi_wasm_rust::Output<bool>,
        /// The name which should be used for this Autonomous Database. Changing this forces a new Autonomous Database to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The national character set for the autonomous database. Changing this forces a new Autonomous Database to be created. The default is AL16UTF16. Allowed values are: AL16UTF16 or UTF8.
        pub national_character_set: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Autonomous Database should exist. Changing this forces a new Autonomous Database to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the subnet the resource is associated with. Changing this forces a new Autonomous Database to be created.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Autonomous Database.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the vnet associated with the cloud VM cluster. Changing this forces a new Autonomous Database to be created.
        pub virtual_network_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AutonomousDatabaseArgs,
    ) -> AutonomousDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admin_password_binding = args.admin_password.get_output(context).get_inner();
        let auto_scaling_enabled_binding = args
            .auto_scaling_enabled
            .get_output(context)
            .get_inner();
        let auto_scaling_for_storage_enabled_binding = args
            .auto_scaling_for_storage_enabled
            .get_output(context)
            .get_inner();
        let backup_retention_period_in_days_binding = args
            .backup_retention_period_in_days
            .get_output(context)
            .get_inner();
        let character_set_binding = args.character_set.get_output(context).get_inner();
        let compute_count_binding = args.compute_count.get_output(context).get_inner();
        let compute_model_binding = args.compute_model.get_output(context).get_inner();
        let customer_contacts_binding = args
            .customer_contacts
            .get_output(context)
            .get_inner();
        let data_storage_size_in_tbs_binding = args
            .data_storage_size_in_tbs
            .get_output(context)
            .get_inner();
        let db_version_binding = args.db_version.get_output(context).get_inner();
        let db_workload_binding = args.db_workload.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let license_model_binding = args.license_model.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let mtls_connection_required_binding = args
            .mtls_connection_required
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let national_character_set_binding = args
            .national_character_set
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let virtual_network_id_binding = args
            .virtual_network_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:oracle/autonomousDatabase:AutonomousDatabase".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "adminPassword".into(),
                    value: &admin_password_binding,
                },
                register_interface::ObjectField {
                    name: "autoScalingEnabled".into(),
                    value: &auto_scaling_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "autoScalingForStorageEnabled".into(),
                    value: &auto_scaling_for_storage_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "backupRetentionPeriodInDays".into(),
                    value: &backup_retention_period_in_days_binding,
                },
                register_interface::ObjectField {
                    name: "characterSet".into(),
                    value: &character_set_binding,
                },
                register_interface::ObjectField {
                    name: "computeCount".into(),
                    value: &compute_count_binding,
                },
                register_interface::ObjectField {
                    name: "computeModel".into(),
                    value: &compute_model_binding,
                },
                register_interface::ObjectField {
                    name: "customerContacts".into(),
                    value: &customer_contacts_binding,
                },
                register_interface::ObjectField {
                    name: "dataStorageSizeInTbs".into(),
                    value: &data_storage_size_in_tbs_binding,
                },
                register_interface::ObjectField {
                    name: "dbVersion".into(),
                    value: &db_version_binding,
                },
                register_interface::ObjectField {
                    name: "dbWorkload".into(),
                    value: &db_workload_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "licenseModel".into(),
                    value: &license_model_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "mtlsConnectionRequired".into(),
                    value: &mtls_connection_required_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nationalCharacterSet".into(),
                    value: &national_character_set_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkId".into(),
                    value: &virtual_network_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AutonomousDatabaseResult {
            admin_password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminPassword"),
            ),
            auto_scaling_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoScalingEnabled"),
            ),
            auto_scaling_for_storage_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoScalingForStorageEnabled"),
            ),
            backup_retention_period_in_days: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupRetentionPeriodInDays"),
            ),
            character_set: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("characterSet"),
            ),
            compute_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("computeCount"),
            ),
            compute_model: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("computeModel"),
            ),
            customer_contacts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerContacts"),
            ),
            data_storage_size_in_tbs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataStorageSizeInTbs"),
            ),
            db_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dbVersion"),
            ),
            db_workload: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dbWorkload"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            license_model: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("licenseModel"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            mtls_connection_required: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mtlsConnectionRequired"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            national_character_set: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nationalCharacterSet"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualNetworkId"),
            ),
        }
    }
}
