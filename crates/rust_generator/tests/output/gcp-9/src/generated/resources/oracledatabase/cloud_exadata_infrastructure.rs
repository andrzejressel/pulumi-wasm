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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cloud_exadata_infrastructure {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
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
        pub cloud_exadata_infrastructure_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// User friendly name for this resource.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// GCP location where Oracle Exadata is hosted.
        #[builder(into, default)]
        pub gcp_oracle_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Labels or tags associated with the resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. See documentation for resource type `oracledatabase.googleapis.com/DbServer`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Various properties of Exadata Infrastructure.
        /// Structure is documented below.
        #[builder(into, default)]
        pub properties: pulumi_gestalt_rust::InputOrOutput<
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
        pub cloud_exadata_infrastructure_id: pulumi_gestalt_rust::Output<String>,
        /// The date and time that the Exadata Infrastructure was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// User friendly name for this resource.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Entitlement ID of the private offer against which this infrastructure
        /// resource is provisioned.
        pub entitlement_id: pulumi_gestalt_rust::Output<String>,
        /// GCP location where Oracle Exadata is hosted.
        pub gcp_oracle_zone: pulumi_gestalt_rust::Output<String>,
        /// Labels or tags associated with the resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. See documentation for resource type `oracledatabase.googleapis.com/DbServer`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Identifier. The name of the Exadata Infrastructure resource with the following format:
        /// projects/{project}/locations/{region}/cloudExadataInfrastructures/{cloud_exadata_infrastructure}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Various properties of Exadata Infrastructure.
        /// Structure is documented below.
        pub properties: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::oracledatabase::CloudExadataInfrastructureProperties,
            >,
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
        args: CloudExadataInfrastructureArgs,
    ) -> CloudExadataInfrastructureResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cloud_exadata_infrastructure_id_binding = args
            .cloud_exadata_infrastructure_id
            .get_output(context);
        let deletion_protection_binding = args.deletion_protection.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let gcp_oracle_zone_binding = args.gcp_oracle_zone.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let properties_binding = args.properties.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:oracledatabase/cloudExadataInfrastructure:CloudExadataInfrastructure"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudExadataInfrastructureId".into(),
                    value: cloud_exadata_infrastructure_id_binding.get_id(),
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
                    name: "gcpOracleZone".into(),
                    value: gcp_oracle_zone_binding.get_id(),
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
        CloudExadataInfrastructureResult {
            cloud_exadata_infrastructure_id: o.get_field("cloudExadataInfrastructureId"),
            create_time: o.get_field("createTime"),
            deletion_protection: o.get_field("deletionProtection"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            entitlement_id: o.get_field("entitlementId"),
            gcp_oracle_zone: o.get_field("gcpOracleZone"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            properties: o.get_field("properties"),
            pulumi_labels: o.get_field("pulumiLabels"),
        }
    }
}
