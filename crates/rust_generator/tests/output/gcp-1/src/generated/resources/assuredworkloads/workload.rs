/// The AssuredWorkloads Workload resource
///
/// ## Example Usage
///
/// ### Basic_workload
/// A basic test of a assuredworkloads api
/// ```yaml
/// resources:
///   primary:
///     type: gcp:assuredworkloads:Workload
///     properties:
///       complianceRegime: FEDRAMP_MODERATE
///       displayName: '{{display}}'
///       location: us-west1
///       organization: '123456789'
///       billingAccount: billingAccounts/000000-0000000-0000000-000000
///       kmsSettings:
///         nextRotationTime: 9999-10-02T15:01:23Z
///         rotationPeriod: 10368000s
///       provisionedResourcesParent: folders/519620126891
///       resourceSettings:
///         - displayName: '{{name}}'
///           resourceType: CONSUMER_FOLDER
///         - resourceType: ENCRYPTION_KEYS_PROJECT
///         - resourceId: ring
///           resourceType: KEYRING
///       violationNotificationsEnabled: true
///       workloadOptions:
///         kajEnrollmentType: KEY_ACCESS_TRANSPARENCY_OFF
///       labels:
///         label-one: value-one
/// ```
/// ### Sovereign_controls_workload
/// A Sovereign Controls test of the assuredworkloads api
/// ```yaml
/// resources:
///   primary:
///     type: gcp:assuredworkloads:Workload
///     properties:
///       complianceRegime: EU_REGIONS_AND_SUPPORT
///       displayName: display
///       location: europe-west9
///       organization: '123456789'
///       billingAccount: billingAccounts/000000-0000000-0000000-000000
///       enableSovereignControls: true
///       kmsSettings:
///         nextRotationTime: 9999-10-02T15:01:23Z
///         rotationPeriod: 10368000s
///       resourceSettings:
///         - resourceType: CONSUMER_FOLDER
///         - resourceType: ENCRYPTION_KEYS_PROJECT
///         - resourceId: ring
///           resourceType: KEYRING
///       labels:
///         label-one: value-one
/// ```
/// ### Split_billing_partner_workload
/// A Split billing partner test of the assuredworkloads api
/// ```yaml
/// resources:
///   primary:
///     type: gcp:assuredworkloads:Workload
///     properties:
///       complianceRegime: ASSURED_WORKLOADS_FOR_PARTNERS
///       displayName: display
///       location: europe-west8
///       organization: '123456789'
///       billingAccount: billingAccounts/000000-0000000-0000000-000000
///       partner: SOVEREIGN_CONTROLS_BY_PSN
///       partnerPermissions:
///         assuredWorkloadsMonitoring: true
///         dataLogsViewer: true
///         serviceAccessApprover: true
///       partnerServicesBillingAccount: billingAccounts/01BF3F-2C6DE5-30C607
///       resourceSettings:
///         - resourceType: CONSUMER_FOLDER
///         - resourceType: ENCRYPTION_KEYS_PROJECT
///         - resourceId: ring
///           resourceType: KEYRING
///       violationNotificationsEnabled: true
///       labels:
///         label-one: value-one
/// ```
///
/// ## Import
///
/// Workload can be imported using any of these accepted formats:
///
/// * `organizations/{{organization}}/locations/{{location}}/workloads/{{name}}`
///
/// * `{{organization}}/{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Workload can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:assuredworkloads/workload:Workload default organizations/{{organization}}/locations/{{location}}/workloads/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:assuredworkloads/workload:Workload default {{organization}}/{{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workload {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkloadArgs {
        /// Optional. Input only. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`.
        #[builder(into, default)]
        pub billing_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. Immutable. Compliance Regime associated with this workload. Possible values: COMPLIANCE_REGIME_UNSPECIFIED, IL4, CJIS, FEDRAMP_HIGH, FEDRAMP_MODERATE, US_REGIONAL_ACCESS, HIPAA, HITRUST, EU_REGIONS_AND_SUPPORT, CA_REGIONS_AND_SUPPORT, ITAR, AU_REGIONS_AND_US_SUPPORT, ASSURED_WORKLOADS_FOR_PARTNERS, ISR_REGIONS, ISR_REGIONS_AND_SUPPORT, CA_PROTECTED_B, IL5, IL2, JP_REGIONS_AND_SUPPORT, KSA_REGIONS_AND_SUPPORT_WITH_SOVEREIGNTY_CONTROLS, REGIONAL_CONTROLS, HEALTHCARE_AND_LIFE_SCIENCES_CONTROLS, HEALTHCARE_AND_LIFE_SCIENCES_CONTROLS_WITH_US_SUPPORT, IRS_1075
        #[builder(into)]
        pub compliance_regime: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers.
        #[builder(into, default)]
        pub enable_sovereign_controls: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// **DEPRECATED** Input only. Settings used to create a CMEK crypto key. When set, a project with a KMS CMEK key is provisioned. This field is deprecated as of Feb 28, 2022. In order to create a Keyring, callers should specify, ENCRYPTION_KEYS_PROJECT or KEYRING in ResourceSettings.resource_type field.
        #[builder(into, default)]
        pub kms_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::assuredworkloads::WorkloadKmsSettings>,
        >,
        /// Optional. Labels applied to the workload.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The organization for the resource
        ///
        ///
        ///
        /// - - -
        #[builder(into)]
        pub organization: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. Partner regime associated with this workload. Possible values: PARTNER_UNSPECIFIED, LOCAL_CONTROLS_BY_S3NS, SOVEREIGN_CONTROLS_BY_T_SYSTEMS, SOVEREIGN_CONTROLS_BY_SIA_MINSAIT, SOVEREIGN_CONTROLS_BY_PSN, SOVEREIGN_CONTROLS_BY_CNTXT, SOVEREIGN_CONTROLS_BY_CNTXT_NO_EKM
        #[builder(into, default)]
        pub partner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Permissions granted to the AW Partner SA account for the customer workload
        #[builder(into, default)]
        pub partner_permissions: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::assuredworkloads::WorkloadPartnerPermissions>,
        >,
        /// Optional. Input only. Billing account necessary for purchasing services from Sovereign Partners. This field is required for creating SIA/PSN/CNTXT partner workloads. The caller should have 'billing.resourceAssociations.create' IAM permission on this billing-account. The format of this string is billingAccounts/AAAAAA-BBBBBB-CCCCCC.
        #[builder(into, default)]
        pub partner_services_billing_account: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Input only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id}
        #[builder(into, default)]
        pub provisioned_resources_parent: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Input only. Resource properties that are used to customize workload resources. These properties (such as custom project id) will be used to create workload resources if possible. This field is optional.
        #[builder(into, default)]
        pub resource_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::assuredworkloads::WorkloadResourceSetting>>,
        >,
        /// Optional. Indicates whether the e-mail notification for a violation is enabled for a workload. This value will be by default True, and if not present will be considered as true. This should only be updated via updateWorkload call. Any Changes to this field during the createWorkload call will not be honored. This will always be true while creating the workload.
        #[builder(into, default)]
        pub violation_notifications_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Optional. Used to specify certain options for a workload during workload creation - currently only supporting KAT Optionality for Regional Controls workloads.
        #[builder(into, default)]
        pub workload_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::assuredworkloads::WorkloadWorkloadOptions>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkloadResult {
        /// Optional. Input only. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`.
        pub billing_account: pulumi_gestalt_rust::Output<Option<String>>,
        /// Required. Immutable. Compliance Regime associated with this workload. Possible values: COMPLIANCE_REGIME_UNSPECIFIED, IL4, CJIS, FEDRAMP_HIGH, FEDRAMP_MODERATE, US_REGIONAL_ACCESS, HIPAA, HITRUST, EU_REGIONS_AND_SUPPORT, CA_REGIONS_AND_SUPPORT, ITAR, AU_REGIONS_AND_US_SUPPORT, ASSURED_WORKLOADS_FOR_PARTNERS, ISR_REGIONS, ISR_REGIONS_AND_SUPPORT, CA_PROTECTED_B, IL5, IL2, JP_REGIONS_AND_SUPPORT, KSA_REGIONS_AND_SUPPORT_WITH_SOVEREIGNTY_CONTROLS, REGIONAL_CONTROLS, HEALTHCARE_AND_LIFE_SCIENCES_CONTROLS, HEALTHCARE_AND_LIFE_SCIENCES_CONTROLS_WITH_US_SUPPORT, IRS_1075
        pub compliance_regime: pulumi_gestalt_rust::Output<String>,
        /// Output only. Count of active Violations in the Workload.
        pub compliance_statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::assuredworkloads::WorkloadComplianceStatus>,
        >,
        /// Output only. Urls for services which are compliant for this Assured Workload, but which are currently disallowed by the ResourceUsageRestriction org policy. Invoke workloads.restrictAllowedResources endpoint to allow your project developers to use these services in their environment.
        pub compliant_but_disallowed_services: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Output only. Immutable. The Workload creation timestamp.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Required. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Represents the Ekm Provisioning State of the given workload.
        pub ekm_provisioning_responses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::assuredworkloads::WorkloadEkmProvisioningResponse>,
        >,
        /// Optional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers.
        pub enable_sovereign_controls: pulumi_gestalt_rust::Output<bool>,
        /// Output only. Represents the KAJ enrollment state of the given workload. Possible values: KAJ_ENROLLMENT_STATE_UNSPECIFIED, KAJ_ENROLLMENT_STATE_PENDING, KAJ_ENROLLMENT_STATE_COMPLETE
        pub kaj_enrollment_state: pulumi_gestalt_rust::Output<String>,
        /// **DEPRECATED** Input only. Settings used to create a CMEK crypto key. When set, a project with a KMS CMEK key is provisioned. This field is deprecated as of Feb 28, 2022. In order to create a Keyring, callers should specify, ENCRYPTION_KEYS_PROJECT or KEYRING in ResourceSettings.resource_type field.
        pub kms_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::assuredworkloads::WorkloadKmsSettings>,
        >,
        /// Optional. Labels applied to the workload.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Output only. The resource name of the workload.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The organization for the resource
        ///
        ///
        ///
        /// - - -
        pub organization: pulumi_gestalt_rust::Output<String>,
        /// Optional. Partner regime associated with this workload. Possible values: PARTNER_UNSPECIFIED, LOCAL_CONTROLS_BY_S3NS, SOVEREIGN_CONTROLS_BY_T_SYSTEMS, SOVEREIGN_CONTROLS_BY_SIA_MINSAIT, SOVEREIGN_CONTROLS_BY_PSN, SOVEREIGN_CONTROLS_BY_CNTXT, SOVEREIGN_CONTROLS_BY_CNTXT_NO_EKM
        pub partner: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. Permissions granted to the AW Partner SA account for the customer workload
        pub partner_permissions: pulumi_gestalt_rust::Output<
            Option<super::super::types::assuredworkloads::WorkloadPartnerPermissions>,
        >,
        /// Optional. Input only. Billing account necessary for purchasing services from Sovereign Partners. This field is required for creating SIA/PSN/CNTXT partner workloads. The caller should have 'billing.resourceAssociations.create' IAM permission on this billing-account. The format of this string is billingAccounts/AAAAAA-BBBBBB-CCCCCC.
        pub partner_services_billing_account: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Input only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id}
        pub provisioned_resources_parent: pulumi_gestalt_rust::Output<Option<String>>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Input only. Resource properties that are used to customize workload resources. These properties (such as custom project id) will be used to create workload resources if possible. This field is optional.
        pub resource_settings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::assuredworkloads::WorkloadResourceSetting>>,
        >,
        /// Output only. The resources associated with this workload. These resources will be created when creating the workload. If any of the projects already exist, the workload creation will fail. Always read only.
        pub resources: pulumi_gestalt_rust::Output<
            Vec<super::super::types::assuredworkloads::WorkloadResource>,
        >,
        /// Output only. Represents the SAA enrollment response of the given workload. SAA enrollment response is queried during workloads.get call. In failure cases, user friendly error message is shown in SAA details page.
        pub saa_enrollment_responses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::assuredworkloads::WorkloadSaaEnrollmentResponse>,
        >,
        /// Optional. Indicates whether the e-mail notification for a violation is enabled for a workload. This value will be by default True, and if not present will be considered as true. This should only be updated via updateWorkload call. Any Changes to this field during the createWorkload call will not be honored. This will always be true while creating the workload.
        pub violation_notifications_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Optional. Used to specify certain options for a workload during workload creation - currently only supporting KAT Optionality for Regional Controls workloads.
        pub workload_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::assuredworkloads::WorkloadWorkloadOptions>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkloadArgs,
    ) -> WorkloadResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let billing_account_binding = args.billing_account.get_output(context);
        let compliance_regime_binding = args.compliance_regime.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enable_sovereign_controls_binding = args
            .enable_sovereign_controls
            .get_output(context);
        let kms_settings_binding = args.kms_settings.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let organization_binding = args.organization.get_output(context);
        let partner_binding = args.partner.get_output(context);
        let partner_permissions_binding = args.partner_permissions.get_output(context);
        let partner_services_billing_account_binding = args
            .partner_services_billing_account
            .get_output(context);
        let provisioned_resources_parent_binding = args
            .provisioned_resources_parent
            .get_output(context);
        let resource_settings_binding = args.resource_settings.get_output(context);
        let violation_notifications_enabled_binding = args
            .violation_notifications_enabled
            .get_output(context);
        let workload_options_binding = args.workload_options.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:assuredworkloads/workload:Workload".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "billingAccount".into(),
                    value: &billing_account_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "complianceRegime".into(),
                    value: &compliance_regime_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableSovereignControls".into(),
                    value: &enable_sovereign_controls_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsSettings".into(),
                    value: &kms_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organization".into(),
                    value: &organization_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partner".into(),
                    value: &partner_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partnerPermissions".into(),
                    value: &partner_permissions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partnerServicesBillingAccount".into(),
                    value: &partner_services_billing_account_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "provisionedResourcesParent".into(),
                    value: &provisioned_resources_parent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceSettings".into(),
                    value: &resource_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "violationNotificationsEnabled".into(),
                    value: &violation_notifications_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workloadOptions".into(),
                    value: &workload_options_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkloadResult {
            billing_account: o.get_field("billingAccount"),
            compliance_regime: o.get_field("complianceRegime"),
            compliance_statuses: o.get_field("complianceStatuses"),
            compliant_but_disallowed_services: o
                .get_field("compliantButDisallowedServices"),
            create_time: o.get_field("createTime"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            ekm_provisioning_responses: o.get_field("ekmProvisioningResponses"),
            enable_sovereign_controls: o.get_field("enableSovereignControls"),
            kaj_enrollment_state: o.get_field("kajEnrollmentState"),
            kms_settings: o.get_field("kmsSettings"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            organization: o.get_field("organization"),
            partner: o.get_field("partner"),
            partner_permissions: o.get_field("partnerPermissions"),
            partner_services_billing_account: o
                .get_field("partnerServicesBillingAccount"),
            provisioned_resources_parent: o.get_field("provisionedResourcesParent"),
            pulumi_labels: o.get_field("pulumiLabels"),
            resource_settings: o.get_field("resourceSettings"),
            resources: o.get_field("resources"),
            saa_enrollment_responses: o.get_field("saaEnrollmentResponses"),
            violation_notifications_enabled: o
                .get_field("violationNotificationsEnabled"),
            workload_options: o.get_field("workloadOptions"),
        }
    }
}
