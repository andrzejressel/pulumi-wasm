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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod autonomous_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutonomousDatabaseArgs {
        /// The password for the default ADMIN user.
        #[builder(into, default)]
        pub admin_password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Autonomous Database to create. This value is restricted
        /// to (^a-z?$) and must be a maximum of 63
        /// characters in length. The value must start with a letter and end with
        /// a letter or a number.
        #[builder(into)]
        pub autonomous_database_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The subnet CIDR range for the Autonmous Database.
        #[builder(into)]
        pub cidr: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Autonomous Database. The database name must be unique in
        /// the project. The name must begin with a letter and can
        /// contain a maximum of 30 alphanumeric characters.
        #[builder(into)]
        pub database: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The display name for the Autonomous Database. The name does not have to be unique within your project.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The labels or tags associated with the Autonomous Database. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. See documentation for resource type `oracledatabase.googleapis.com/AutonomousDatabaseBackup`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the VPC network used by the Autonomous Database.
        /// Format: projects/{project}/global/networks/{network}
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The properties of an Autonomous Database.
        /// Structure is documented below.
        #[builder(into)]
        pub properties: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::oracledatabase::AutonomousDatabaseProperties,
        >,
    }
    #[allow(dead_code)]
    pub struct AutonomousDatabaseResult {
        /// The password for the default ADMIN user.
        pub admin_password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Autonomous Database to create. This value is restricted
        /// to (^a-z?$) and must be a maximum of 63
        /// characters in length. The value must start with a letter and end with
        /// a letter or a number.
        pub autonomous_database_id: pulumi_gestalt_rust::Output<String>,
        /// The subnet CIDR range for the Autonmous Database.
        pub cidr: pulumi_gestalt_rust::Output<String>,
        /// The date and time that the Autonomous Database was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The name of the Autonomous Database. The database name must be unique in
        /// the project. The name must begin with a letter and can
        /// contain a maximum of 30 alphanumeric characters.
        pub database: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The display name for the Autonomous Database. The name does not have to be unique within your project.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the subscription entitlement associated with the Autonomous
        /// Database.
        pub entitlement_id: pulumi_gestalt_rust::Output<String>,
        /// The labels or tags associated with the Autonomous Database. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. See documentation for resource type `oracledatabase.googleapis.com/AutonomousDatabaseBackup`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Identifier. The name of the Autonomous Database resource in the following format:
        /// projects/{project}/locations/{region}/autonomousDatabases/{autonomous_database}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the VPC network used by the Autonomous Database.
        /// Format: projects/{project}/global/networks/{network}
        pub network: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The properties of an Autonomous Database.
        /// Structure is documented below.
        pub properties: pulumi_gestalt_rust::Output<
            super::super::types::oracledatabase::AutonomousDatabaseProperties,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AutonomousDatabaseArgs,
    ) -> AutonomousDatabaseResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let admin_password_binding = args.admin_password.get_output(context);
        let autonomous_database_id_binding = args
            .autonomous_database_id
            .get_output(context);
        let cidr_binding = args.cidr.get_output(context);
        let database_binding = args.database.get_output(context);
        let deletion_protection_binding = args.deletion_protection.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let network_binding = args.network.get_output(context);
        let project_binding = args.project.get_output(context);
        let properties_binding = args.properties.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:oracledatabase/autonomousDatabase:AutonomousDatabase".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminPassword".into(),
                    value: admin_password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autonomousDatabaseId".into(),
                    value: autonomous_database_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidr".into(),
                    value: cidr_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "database".into(),
                    value: database_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtection".into(),
                    value: deletion_protection_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: network_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "properties".into(),
                    value: properties_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AutonomousDatabaseResult {
            admin_password: o.get_field("adminPassword"),
            autonomous_database_id: o.get_field("autonomousDatabaseId"),
            cidr: o.get_field("cidr"),
            create_time: o.get_field("createTime"),
            database: o.get_field("database"),
            deletion_protection: o.get_field("deletionProtection"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            entitlement_id: o.get_field("entitlementId"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            project: o.get_field("project"),
            properties: o.get_field("properties"),
            pulumi_labels: o.get_field("pulumiLabels"),
        }
    }
}
