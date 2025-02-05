/// A CloudExadataInfrastructure resource.
///
///
/// To get more information about CloudExadataInfrastructure, see:
///
/// * [API documentation](https://cloud.google.com/oracle/database/docs/reference/rest/v1/projects.locations.cloudExadataInfrastructures)
/// * How-to Guides
///     * [Create Exadata Infrastructure instances](https://cloud.google.com/oracle/database/docs/create-instances)
///
/// ## Example Usage
///
/// ### Oracledatabase Cloud Exadata Infrastructure Basic
///
///
/// ```yaml
/// resources:
///   my-cloud-exadata:
///     type: gcp:oracledatabase:CloudExadataInfrastructure
///     properties:
///       cloudExadataInfrastructureId: my-instance
///       displayName: my-instance displayname
///       location: us-east4
///       project: my-project
///       properties:
///         shape: Exadata.X9M
///         computeCount: '2'
///         storageCount: '3'
///       deletionProtection: 'true'
/// ```
/// ### Oracledatabase Cloud Exadata Infrastructure Full
///
///
/// ```yaml
/// resources:
///   my-cloud-exadata:
///     type: gcp:oracledatabase:CloudExadataInfrastructure
///     properties:
///       cloudExadataInfrastructureId: my-instance
///       displayName: my-instance displayname
///       location: us-east4
///       project: my-project
///       gcpOracleZone: us-east4-b-r1
///       properties:
///         shape: Exadata.X9M
///         computeCount: '2'
///         storageCount: '3'
///         customerContacts:
///           - email: xyz@example.com
///         maintenanceWindow:
///           customActionTimeoutMins: '20'
///           daysOfWeeks:
///             - SUNDAY
///           hoursOfDays:
///             - 4
///           isCustomActionTimeoutEnabled: '0'
///           leadTimeWeek: '1'
///           months:
///             - JANUARY
///             - APRIL
///             - MAY
///             - OCTOBER
///           patchingMode: ROLLING
///           preference: CUSTOM_PREFERENCE
///           weeksOfMonths:
///             - 4
///         totalStorageSizeGb: '196608'
///       labels:
///         label-one: value-one
///       deletionProtection: 'true'
/// ```
///
/// ## Import
///
/// CloudExadataInfrastructure can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/cloudExadataInfrastructures/{{cloud_exadata_infrastructure_id}}`
///
/// * `{{project}}/{{location}}/{{cloud_exadata_infrastructure_id}}`
///
/// * `{{location}}/{{cloud_exadata_infrastructure_id}}`
///
/// When using the `pulumi import` command, CloudExadataInfrastructure can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:oracledatabase/cloudExadataInfrastructure:CloudExadataInfrastructure default projects/{{project}}/locations/{{location}}/cloudExadataInfrastructures/{{cloud_exadata_infrastructure_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:oracledatabase/cloudExadataInfrastructure:CloudExadataInfrastructure default {{project}}/{{location}}/{{cloud_exadata_infrastructure_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:oracledatabase/cloudExadataInfrastructure:CloudExadataInfrastructure default {{location}}/{{cloud_exadata_infrastructure_id}}
/// ```
///
pub mod cloud_exadata_infrastructure {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CloudExadataInfrastructureArgs {
        /// The ID of the Exadata Infrastructure to create. This value is restricted
        /// to (^a-z?$) and must be a maximum of 63
        /// characters in length. The value must start with a letter and end with
        /// a letter or a number.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub cloud_exadata_infrastructure_id: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub deletion_protection: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// User friendly name for this resource.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// GCP location where Oracle Exadata is hosted.
        #[builder(into, default)]
        pub gcp_oracle_zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Labels or tags associated with the resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. See documentation for resource type `oracledatabase.googleapis.com/DbServer`.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Various properties of Exadata Infrastructure.
        /// Structure is documented below.
        #[builder(into, default)]
        pub properties: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::oracledatabase::CloudExadataInfrastructureProperties,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct CloudExadataInfrastructureResult {
        /// The ID of the Exadata Infrastructure to create. This value is restricted
        /// to (^a-z?$) and must be a maximum of 63
        /// characters in length. The value must start with a letter and end with
        /// a letter or a number.
        ///
        ///
        /// - - -
        pub cloud_exadata_infrastructure_id: pulumi_wasm_rust::Output<String>,
        /// The date and time that the Exadata Infrastructure was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// User friendly name for this resource.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Entitlement ID of the private offer against which this infrastructure
        /// resource is provisioned.
        pub entitlement_id: pulumi_wasm_rust::Output<String>,
        /// GCP location where Oracle Exadata is hosted.
        pub gcp_oracle_zone: pulumi_wasm_rust::Output<String>,
        /// Labels or tags associated with the resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. See documentation for resource type `oracledatabase.googleapis.com/DbServer`.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Identifier. The name of the Exadata Infrastructure resource with the following format:
        /// projects/{project}/locations/{region}/cloudExadataInfrastructures/{cloud_exadata_infrastructure}
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Various properties of Exadata Infrastructure.
        /// Structure is documented below.
        pub properties: pulumi_wasm_rust::Output<
            Option<
                super::super::types::oracledatabase::CloudExadataInfrastructureProperties,
            >,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CloudExadataInfrastructureArgs,
    ) -> CloudExadataInfrastructureResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloud_exadata_infrastructure_id_binding = args
            .cloud_exadata_infrastructure_id
            .get_output(context)
            .get_inner();
        let deletion_protection_binding = args
            .deletion_protection
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let gcp_oracle_zone_binding = args
            .gcp_oracle_zone
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let properties_binding = args.properties.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:oracledatabase/cloudExadataInfrastructure:CloudExadataInfrastructure"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudExadataInfrastructureId".into(),
                    value: &cloud_exadata_infrastructure_id_binding,
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
                    name: "gcpOracleZone".into(),
                    value: &gcp_oracle_zone_binding,
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
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "properties".into(),
                    value: &properties_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CloudExadataInfrastructureResult {
            cloud_exadata_infrastructure_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudExadataInfrastructureId"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            entitlement_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("entitlementId"),
            ),
            gcp_oracle_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("gcpOracleZone"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            properties: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("properties"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
        }
    }
}
