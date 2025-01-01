/// An AutonomousDatabase resource.
///
///
/// To get more information about AutonomousDatabase, see:
///
/// * [API documentation](https://cloud.google.com/oracle/database/docs/reference/rest/v1/projects.locations.autonomousDatabases)
/// * How-to Guides
///     * [Create Autonomous databases](https://cloud.google.com/oracle/database/docs/create-databases)
///
/// ## Example Usage
///
/// ### Oracledatabase Autonomous Database Basic
///
///
/// ```yaml
/// resources:
///   myADB:
///     type: gcp:oracledatabase:AutonomousDatabase
///     properties:
///       autonomousDatabaseId: my-instance
///       location: us-east4
///       project: my-project
///       database: mydatabase
///       adminPassword: 123Abpassword
///       network: ${default.id}
///       cidr: 10.5.0.0/24
///       properties:
///         computeCount: '2'
///         dataStorageSizeTb: '1'
///         dbVersion: 19c
///         dbWorkload: OLTP
///         licenseType: LICENSE_INCLUDED
///       deletionProtection: 'true'
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:compute:getNetwork
///       arguments:
///         name: new
///         project: my-project
/// ```
/// ### Oracledatabase Autonomous Database Full
///
///
/// ```yaml
/// resources:
///   myADB:
///     type: gcp:oracledatabase:AutonomousDatabase
///     properties:
///       autonomousDatabaseId: my-instance
///       location: us-east4
///       project: my-project
///       displayName: autonomousDatabase displayname
///       database: mydatabase
///       adminPassword: 123Abpassword
///       network: ${default.id}
///       cidr: 10.5.0.0/24
///       labels:
///         label-one: value-one
///       properties:
///         computeCount: '2'
///         dataStorageSizeGb: '48'
///         dbVersion: 19c
///         dbEdition: STANDARD_EDITION
///         dbWorkload: OLTP
///         isAutoScalingEnabled: 'true'
///         licenseType: BRING_YOUR_OWN_LICENSE
///         backupRetentionPeriodDays: '60'
///         characterSet: AL32UTF8
///         isStorageAutoScalingEnabled: 'false'
///         maintenanceScheduleType: REGULAR
///         mtlsConnectionRequired: 'false'
///         nCharacterSet: AL16UTF16
///         operationsInsightsState: NOT_ENABLED
///         customerContacts:
///           - email: xyz@example.com
///         privateEndpointIp: 10.5.0.11
///         privateEndpointLabel: testhost
///       deletionProtection: 'true'
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:compute:getNetwork
///       arguments:
///         name: new
///         project: my-project
/// ```
///
/// ## Import
///
/// AutonomousDatabase can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/autonomousDatabases/{{autonomous_database_id}}`
///
/// * `{{project}}/{{location}}/{{autonomous_database_id}}`
///
/// * `{{location}}/{{autonomous_database_id}}`
///
/// When using the `pulumi import` command, AutonomousDatabase can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:oracledatabase/autonomousDatabase:AutonomousDatabase default projects/{{project}}/locations/{{location}}/autonomousDatabases/{{autonomous_database_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:oracledatabase/autonomousDatabase:AutonomousDatabase default {{project}}/{{location}}/{{autonomous_database_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:oracledatabase/autonomousDatabase:AutonomousDatabase default {{location}}/{{autonomous_database_id}}
/// ```
///
pub mod autonomous_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutonomousDatabaseArgs {
        /// The password for the default ADMIN user.
        #[builder(into, default)]
        pub admin_password: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Autonomous Database to create. This value is restricted
        /// to (^a-z?$) and must be a maximum of 63
        /// characters in length. The value must start with a letter and end with
        /// a letter or a number.
        #[builder(into)]
        pub autonomous_database_id: pulumi_wasm_rust::Output<String>,
        /// The subnet CIDR range for the Autonmous Database.
        #[builder(into)]
        pub cidr: pulumi_wasm_rust::Output<String>,
        /// The name of the Autonomous Database. The database name must be unique in
        /// the project. The name must begin with a letter and can
        /// contain a maximum of 30 alphanumeric characters.
        #[builder(into)]
        pub database: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// The display name for the Autonomous Database. The name does not have to be unique within your project.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The labels or tags associated with the Autonomous Database. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. See documentation for resource type `oracledatabase.googleapis.com/AutonomousDatabaseBackup`.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the VPC network used by the Autonomous Database.
        /// Format: projects/{project}/global/networks/{network}
        #[builder(into)]
        pub network: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The properties of an Autonomous Database.
        /// Structure is documented below.
        #[builder(into)]
        pub properties: pulumi_wasm_rust::Output<
            super::super::types::oracledatabase::AutonomousDatabaseProperties,
        >,
    }
    #[allow(dead_code)]
    pub struct AutonomousDatabaseResult {
        /// The password for the default ADMIN user.
        pub admin_password: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Autonomous Database to create. This value is restricted
        /// to (^a-z?$) and must be a maximum of 63
        /// characters in length. The value must start with a letter and end with
        /// a letter or a number.
        pub autonomous_database_id: pulumi_wasm_rust::Output<String>,
        /// The subnet CIDR range for the Autonmous Database.
        pub cidr: pulumi_wasm_rust::Output<String>,
        /// The date and time that the Autonomous Database was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The name of the Autonomous Database. The database name must be unique in
        /// the project. The name must begin with a letter and can
        /// contain a maximum of 30 alphanumeric characters.
        pub database: pulumi_wasm_rust::Output<String>,
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// The display name for the Autonomous Database. The name does not have to be unique within your project.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the subscription entitlement associated with the Autonomous
        /// Database.
        pub entitlement_id: pulumi_wasm_rust::Output<String>,
        /// The labels or tags associated with the Autonomous Database. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. See documentation for resource type `oracledatabase.googleapis.com/AutonomousDatabaseBackup`.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Identifier. The name of the Autonomous Database resource in the following format:
        /// projects/{project}/locations/{region}/autonomousDatabases/{autonomous_database}
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the VPC network used by the Autonomous Database.
        /// Format: projects/{project}/global/networks/{network}
        pub network: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The properties of an Autonomous Database.
        /// Structure is documented below.
        pub properties: pulumi_wasm_rust::Output<
            super::super::types::oracledatabase::AutonomousDatabaseProperties,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AutonomousDatabaseArgs) -> AutonomousDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admin_password_binding = args.admin_password.get_inner();
        let autonomous_database_id_binding = args.autonomous_database_id.get_inner();
        let cidr_binding = args.cidr.get_inner();
        let database_binding = args.database.get_inner();
        let deletion_protection_binding = args.deletion_protection.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let network_binding = args.network.get_inner();
        let project_binding = args.project.get_inner();
        let properties_binding = args.properties.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:oracledatabase/autonomousDatabase:AutonomousDatabase".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "adminPassword".into(),
                    value: &admin_password_binding,
                },
                register_interface::ObjectField {
                    name: "autonomousDatabaseId".into(),
                    value: &autonomous_database_id_binding,
                },
                register_interface::ObjectField {
                    name: "cidr".into(),
                    value: &cidr_binding,
                },
                register_interface::ObjectField {
                    name: "database".into(),
                    value: &database_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "properties".into(),
                    value: &properties_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "adminPassword".into(),
                },
                register_interface::ResultField {
                    name: "autonomousDatabaseId".into(),
                },
                register_interface::ResultField {
                    name: "cidr".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "database".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "entitlementId".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "properties".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AutonomousDatabaseResult {
            admin_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminPassword").unwrap(),
            ),
            autonomous_database_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autonomousDatabaseId").unwrap(),
            ),
            cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidr").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            database: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("database").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            entitlement_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entitlementId").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("properties").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
        }
    }
}
