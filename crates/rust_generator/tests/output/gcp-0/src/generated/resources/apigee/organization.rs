/// An `Organization` is the top-level container in Apigee.
///
///
/// To get more information about Organization, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations)
/// * How-to Guides
///     * [Creating an API organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org)
///
/// ## Example Usage
///
/// ### Apigee Organization Cloud Basic
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   org:
///     type: gcp:apigee:Organization
///     properties:
///       analyticsRegion: us-central1
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
/// ### Apigee Organization Cloud Basic Disable Vpc Peering
///
///
/// ```yaml
/// resources:
///   org:
///     type: gcp:apigee:Organization
///     properties:
///       description: Terraform-provisioned basic Apigee Org without VPC Peering.
///       analyticsRegion: us-central1
///       projectId: ${current.project}
///       disableVpcPeering: true
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
/// ### Apigee Organization Cloud Full
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   apigeeKeyring:
///     type: gcp:kms:KeyRing
///     name: apigee_keyring
///     properties:
///       name: apigee-keyring
///       location: us-central1
///   apigeeKey:
///     type: gcp:kms:CryptoKey
///     name: apigee_key
///     properties:
///       name: apigee-key
///       keyRing: ${apigeeKeyring.id}
///   apigeeSa:
///     type: gcp:projects:ServiceIdentity
///     name: apigee_sa
///     properties:
///       project: ${project.projectId}
///       service: ${apigee.service}
///   apigeeSaKeyuser:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: apigee_sa_keyuser
///     properties:
///       cryptoKeyId: ${apigeeKey.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: ${apigeeSa.member}
///   org:
///     type: gcp:apigee:Organization
///     properties:
///       analyticsRegion: us-central1
///       displayName: apigee-org
///       description: Auto-provisioned Apigee Org.
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///       runtimeDatabaseEncryptionKeyName: ${apigeeKey.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///         - ${apigeeSaKeyuser}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
/// ### Apigee Organization Cloud Full Disable Vpc Peering
///
///
/// ```yaml
/// resources:
///   apigeeKeyring:
///     type: gcp:kms:KeyRing
///     name: apigee_keyring
///     properties:
///       name: apigee-keyring
///       location: us-central1
///   apigeeKey:
///     type: gcp:kms:CryptoKey
///     name: apigee_key
///     properties:
///       name: apigee-key
///       keyRing: ${apigeeKeyring.id}
///   apigeeSa:
///     type: gcp:projects:ServiceIdentity
///     name: apigee_sa
///     properties:
///       project: ${project.projectId}
///       service: ${apigee.service}
///   apigeeSaKeyuser:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: apigee_sa_keyuser
///     properties:
///       cryptoKeyId: ${apigeeKey.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: ${apigeeSa.member}
///   org:
///     type: gcp:apigee:Organization
///     properties:
///       analyticsRegion: us-central1
///       displayName: apigee-org
///       description: Terraform-provisioned Apigee Org without VPC Peering.
///       projectId: ${current.project}
///       disableVpcPeering: true
///       runtimeDatabaseEncryptionKeyName: ${apigeeKey.id}
///     options:
///       dependsOn:
///         - ${apigeeSaKeyuser}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Organization can be imported using any of these accepted formats:
///
/// * `organizations/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Organization can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/organization:Organization default organizations/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/organization:Organization default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationArgs {
        /// Primary GCP region for analytics data storage. For valid values, see [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org).
        #[builder(into, default)]
        pub analytics_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Cloud KMS key name used for encrypting API consumer data.
        #[builder(into, default)]
        pub api_consumer_data_encryption_key_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// This field is needed only for customers using non-default data residency regions.
        /// Apigee stores some control plane data only in single region.
        /// This field determines which single region Apigee should use.
        #[builder(into, default)]
        pub api_consumer_data_location: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Compute Engine network used for Service Networking to be peered with Apigee runtime instances.
        /// See [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started).
        /// Valid only when `RuntimeType` is set to CLOUD. The value can be updated only when there are no runtime instances. For example: "default".
        #[builder(into, default)]
        pub authorized_network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Billing type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing).
        #[builder(into, default)]
        pub billing_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Cloud KMS key name used for encrypting control plane data that is stored in a multi region.
        /// Only used for the data residency region "US" or "EU".
        #[builder(into, default)]
        pub control_plane_encryption_key_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Description of the Apigee organization.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Flag that specifies whether the VPC Peering through Private Google Access should be
        /// disabled between the consumer network and Apigee. Required if an `authorizedNetwork`
        /// on the consumer project is not provided, in which case the flag should be set to `true`.
        /// Valid only when `RuntimeType` is set to CLOUD. The value must be set before the creation
        /// of any Apigee runtime instance and can be updated only when there are no runtime instances.
        #[builder(into, default)]
        pub disable_vpc_peering: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The display name of the Apigee organization.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project ID associated with the Apigee organization.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub project_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Properties defined in the Apigee organization profile.
        /// Structure is documented below.
        #[builder(into, default)]
        pub properties: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigee::OrganizationProperties>,
        >,
        /// Optional. This setting is applicable only for organizations that are soft-deleted (i.e., BillingType
        /// is not EVALUATION). It controls how long Organization data will be retained after the initial delete
        /// operation completes. During this period, the Organization may be restored to its last known state.
        /// After this period, the Organization will no longer be able to be restored.
        /// Default value is `DELETION_RETENTION_UNSPECIFIED`.
        /// Possible values are: `DELETION_RETENTION_UNSPECIFIED`, `MINIMUM`.
        #[builder(into, default)]
        pub retention: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Cloud KMS key name used for encrypting the data that is stored and replicated across runtime instances.
        /// Update is not allowed after the organization is created.
        /// If not specified, a Google-Managed encryption key will be used.
        /// Valid only when `RuntimeType` is CLOUD. For example: `projects/foo/locations/us/keyRings/bar/cryptoKeys/baz`.
        #[builder(into, default)]
        pub runtime_database_encryption_key_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Runtime type of the Apigee organization based on the Apigee subscription purchased.
        /// Default value is `CLOUD`.
        /// Possible values are: `CLOUD`, `HYBRID`.
        #[builder(into, default)]
        pub runtime_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationResult {
        /// Primary GCP region for analytics data storage. For valid values, see [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org).
        pub analytics_region: pulumi_gestalt_rust::Output<Option<String>>,
        /// Cloud KMS key name used for encrypting API consumer data.
        pub api_consumer_data_encryption_key_name: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// This field is needed only for customers using non-default data residency regions.
        /// Apigee stores some control plane data only in single region.
        /// This field determines which single region Apigee should use.
        pub api_consumer_data_location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Output only. Project ID of the Apigee Tenant Project.
        pub apigee_project_id: pulumi_gestalt_rust::Output<String>,
        /// Compute Engine network used for Service Networking to be peered with Apigee runtime instances.
        /// See [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started).
        /// Valid only when `RuntimeType` is set to CLOUD. The value can be updated only when there are no runtime instances. For example: "default".
        pub authorized_network: pulumi_gestalt_rust::Output<Option<String>>,
        /// Billing type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing).
        pub billing_type: pulumi_gestalt_rust::Output<String>,
        /// Output only. Base64-encoded public certificate for the root CA of the Apigee organization.
        /// Valid only when `RuntimeType` is CLOUD. A base64-encoded string.
        pub ca_certificate: pulumi_gestalt_rust::Output<String>,
        /// Cloud KMS key name used for encrypting control plane data that is stored in a multi region.
        /// Only used for the data residency region "US" or "EU".
        pub control_plane_encryption_key_name: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Description of the Apigee organization.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Flag that specifies whether the VPC Peering through Private Google Access should be
        /// disabled between the consumer network and Apigee. Required if an `authorizedNetwork`
        /// on the consumer project is not provided, in which case the flag should be set to `true`.
        /// Valid only when `RuntimeType` is set to CLOUD. The value must be set before the creation
        /// of any Apigee runtime instance and can be updated only when there are no runtime instances.
        pub disable_vpc_peering: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The display name of the Apigee organization.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Output only. Name of the Apigee organization.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The project ID associated with the Apigee organization.
        ///
        ///
        /// - - -
        pub project_id: pulumi_gestalt_rust::Output<String>,
        /// Properties defined in the Apigee organization profile.
        /// Structure is documented below.
        pub properties: pulumi_gestalt_rust::Output<
            super::super::types::apigee::OrganizationProperties,
        >,
        /// Optional. This setting is applicable only for organizations that are soft-deleted (i.e., BillingType
        /// is not EVALUATION). It controls how long Organization data will be retained after the initial delete
        /// operation completes. During this period, the Organization may be restored to its last known state.
        /// After this period, the Organization will no longer be able to be restored.
        /// Default value is `DELETION_RETENTION_UNSPECIFIED`.
        /// Possible values are: `DELETION_RETENTION_UNSPECIFIED`, `MINIMUM`.
        pub retention: pulumi_gestalt_rust::Output<Option<String>>,
        /// Cloud KMS key name used for encrypting the data that is stored and replicated across runtime instances.
        /// Update is not allowed after the organization is created.
        /// If not specified, a Google-Managed encryption key will be used.
        /// Valid only when `RuntimeType` is CLOUD. For example: `projects/foo/locations/us/keyRings/bar/cryptoKeys/baz`.
        pub runtime_database_encryption_key_name: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Runtime type of the Apigee organization based on the Apigee subscription purchased.
        /// Default value is `CLOUD`.
        /// Possible values are: `CLOUD`, `HYBRID`.
        pub runtime_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Output only. Subscription type of the Apigee organization.
        /// Valid values include trial (free, limited, and for evaluation purposes only) or paid (full subscription has been purchased).
        pub subscription_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationArgs,
    ) -> OrganizationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let analytics_region_binding = args.analytics_region.get_output(context);
        let api_consumer_data_encryption_key_name_binding = args
            .api_consumer_data_encryption_key_name
            .get_output(context);
        let api_consumer_data_location_binding = args
            .api_consumer_data_location
            .get_output(context);
        let authorized_network_binding = args.authorized_network.get_output(context);
        let billing_type_binding = args.billing_type.get_output(context);
        let control_plane_encryption_key_name_binding = args
            .control_plane_encryption_key_name
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let disable_vpc_peering_binding = args.disable_vpc_peering.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let project_id_binding = args.project_id.get_output(context);
        let properties_binding = args.properties.get_output(context);
        let retention_binding = args.retention.get_output(context);
        let runtime_database_encryption_key_name_binding = args
            .runtime_database_encryption_key_name
            .get_output(context);
        let runtime_type_binding = args.runtime_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/organization:Organization".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "analyticsRegion".into(),
                    value: &analytics_region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiConsumerDataEncryptionKeyName".into(),
                    value: &api_consumer_data_encryption_key_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiConsumerDataLocation".into(),
                    value: &api_consumer_data_location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizedNetwork".into(),
                    value: &authorized_network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "billingType".into(),
                    value: &billing_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlPlaneEncryptionKeyName".into(),
                    value: &control_plane_encryption_key_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableVpcPeering".into(),
                    value: &disable_vpc_peering_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "projectId".into(),
                    value: &project_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "properties".into(),
                    value: &properties_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retention".into(),
                    value: &retention_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtimeDatabaseEncryptionKeyName".into(),
                    value: &runtime_database_encryption_key_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtimeType".into(),
                    value: &runtime_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationResult {
            analytics_region: o.get_field("analyticsRegion"),
            api_consumer_data_encryption_key_name: o
                .get_field("apiConsumerDataEncryptionKeyName"),
            api_consumer_data_location: o.get_field("apiConsumerDataLocation"),
            apigee_project_id: o.get_field("apigeeProjectId"),
            authorized_network: o.get_field("authorizedNetwork"),
            billing_type: o.get_field("billingType"),
            ca_certificate: o.get_field("caCertificate"),
            control_plane_encryption_key_name: o
                .get_field("controlPlaneEncryptionKeyName"),
            description: o.get_field("description"),
            disable_vpc_peering: o.get_field("disableVpcPeering"),
            display_name: o.get_field("displayName"),
            name: o.get_field("name"),
            project_id: o.get_field("projectId"),
            properties: o.get_field("properties"),
            retention: o.get_field("retention"),
            runtime_database_encryption_key_name: o
                .get_field("runtimeDatabaseEncryptionKeyName"),
            runtime_type: o.get_field("runtimeType"),
            subscription_type: o.get_field("subscriptionType"),
        }
    }
}
