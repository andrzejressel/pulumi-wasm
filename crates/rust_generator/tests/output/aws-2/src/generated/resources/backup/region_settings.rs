/// Provides an AWS Backup Region Settings resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:backup:RegionSettings
///     properties:
///       resourceTypeOptInPreference:
///         Aurora: true
///         DocumentDB: true
///         DynamoDB: true
///         EBS: true
///         EC2: true
///         EFS: true
///         FSx: true
///         Neptune: true
///         RDS: true
///         Storage Gateway: true
///         VirtualMachine: true
///         CloudFormation: true
///         Redshift: true
///         S3: true
///         SAP HANA on Amazon EC2: true
///       resourceTypeManagementPreference:
///         DynamoDB: true
///         EFS: true
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup Region Settings using the `region`. For example:
///
/// ```sh
/// $ pulumi import aws:backup/regionSettings:RegionSettings test us-west-2
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod region_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionSettingsArgs {
        /// A map of services along with the management preferences for the Region. For more information, see the [AWS Documentation](https://docs.aws.amazon.com/aws-backup/latest/devguide/API_UpdateRegionSettings.html#API_UpdateRegionSettings_RequestSyntax).
        #[builder(into, default)]
        pub resource_type_management_preference: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, bool>>,
        >,
        /// A map of services along with the opt-in preferences for the Region.
        #[builder(into)]
        pub resource_type_opt_in_preference: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct RegionSettingsResult {
        /// A map of services along with the management preferences for the Region. For more information, see the [AWS Documentation](https://docs.aws.amazon.com/aws-backup/latest/devguide/API_UpdateRegionSettings.html#API_UpdateRegionSettings_RequestSyntax).
        pub resource_type_management_preference: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, bool>,
        >,
        /// A map of services along with the opt-in preferences for the Region.
        pub resource_type_opt_in_preference: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, bool>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegionSettingsArgs,
    ) -> RegionSettingsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let resource_type_management_preference_binding = args
            .resource_type_management_preference
            .get_output(context);
        let resource_type_opt_in_preference_binding = args
            .resource_type_opt_in_preference
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:backup/regionSettings:RegionSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceTypeManagementPreference".into(),
                    value: &resource_type_management_preference_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceTypeOptInPreference".into(),
                    value: &resource_type_opt_in_preference_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegionSettingsResult {
            resource_type_management_preference: o
                .get_field("resourceTypeManagementPreference"),
            resource_type_opt_in_preference: o.get_field("resourceTypeOptInPreference"),
        }
    }
}
