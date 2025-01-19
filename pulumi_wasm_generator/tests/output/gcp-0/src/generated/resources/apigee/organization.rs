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
pub mod organization {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationArgs {
        /// Primary GCP region for analytics data storage. For valid values, see [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org).
        #[builder(into, default)]
        pub analytics_region: pulumi_wasm_rust::Output<Option<String>>,
        /// Cloud KMS key name used for encrypting API consumer data.
        #[builder(into, default)]
        pub api_consumer_data_encryption_key_name: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// This field is needed only for customers using non-default data residency regions.
        /// Apigee stores some control plane data only in single region.
        /// This field determines which single region Apigee should use.
        #[builder(into, default)]
        pub api_consumer_data_location: pulumi_wasm_rust::Output<Option<String>>,
        /// Compute Engine network used for Service Networking to be peered with Apigee runtime instances.
        /// See [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started).
        /// Valid only when `RuntimeType` is set to CLOUD. The value can be updated only when there are no runtime instances. For example: "default".
        #[builder(into, default)]
        pub authorized_network: pulumi_wasm_rust::Output<Option<String>>,
        /// Billing type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing).
        #[builder(into, default)]
        pub billing_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Cloud KMS key name used for encrypting control plane data that is stored in a multi region.
        /// Only used for the data residency region "US" or "EU".
        #[builder(into, default)]
        pub control_plane_encryption_key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the Apigee organization.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Flag that specifies whether the VPC Peering through Private Google Access should be
        /// disabled between the consumer network and Apigee. Required if an `authorizedNetwork`
        /// on the consumer project is not provided, in which case the flag should be set to `true`.
        /// Valid only when `RuntimeType` is set to CLOUD. The value must be set before the creation
        /// of any Apigee runtime instance and can be updated only when there are no runtime instances.
        #[builder(into, default)]
        pub disable_vpc_peering: pulumi_wasm_rust::Output<Option<bool>>,
        /// The display name of the Apigee organization.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The project ID associated with the Apigee organization.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub project_id: pulumi_wasm_rust::Output<String>,
        /// Properties defined in the Apigee organization profile.
        /// Structure is documented below.
        #[builder(into, default)]
        pub properties: pulumi_wasm_rust::Output<
            Option<super::super::types::apigee::OrganizationProperties>,
        >,
        /// Optional. This setting is applicable only for organizations that are soft-deleted (i.e., BillingType
        /// is not EVALUATION). It controls how long Organization data will be retained after the initial delete
        /// operation completes. During this period, the Organization may be restored to its last known state.
        /// After this period, the Organization will no longer be able to be restored.
        /// Default value is `DELETION_RETENTION_UNSPECIFIED`.
        /// Possible values are: `DELETION_RETENTION_UNSPECIFIED`, `MINIMUM`.
        #[builder(into, default)]
        pub retention: pulumi_wasm_rust::Output<Option<String>>,
        /// Cloud KMS key name used for encrypting the data that is stored and replicated across runtime instances.
        /// Update is not allowed after the organization is created.
        /// If not specified, a Google-Managed encryption key will be used.
        /// Valid only when `RuntimeType` is CLOUD. For example: `projects/foo/locations/us/keyRings/bar/cryptoKeys/baz`.
        #[builder(into, default)]
        pub runtime_database_encryption_key_name: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Runtime type of the Apigee organization based on the Apigee subscription purchased.
        /// Default value is `CLOUD`.
        /// Possible values are: `CLOUD`, `HYBRID`.
        #[builder(into, default)]
        pub runtime_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationResult {
        /// Primary GCP region for analytics data storage. For valid values, see [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org).
        pub analytics_region: pulumi_wasm_rust::Output<Option<String>>,
        /// Cloud KMS key name used for encrypting API consumer data.
        pub api_consumer_data_encryption_key_name: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// This field is needed only for customers using non-default data residency regions.
        /// Apigee stores some control plane data only in single region.
        /// This field determines which single region Apigee should use.
        pub api_consumer_data_location: pulumi_wasm_rust::Output<Option<String>>,
        /// Output only. Project ID of the Apigee Tenant Project.
        pub apigee_project_id: pulumi_wasm_rust::Output<String>,
        /// Compute Engine network used for Service Networking to be peered with Apigee runtime instances.
        /// See [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started).
        /// Valid only when `RuntimeType` is set to CLOUD. The value can be updated only when there are no runtime instances. For example: "default".
        pub authorized_network: pulumi_wasm_rust::Output<Option<String>>,
        /// Billing type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing).
        pub billing_type: pulumi_wasm_rust::Output<String>,
        /// Output only. Base64-encoded public certificate for the root CA of the Apigee organization.
        /// Valid only when `RuntimeType` is CLOUD. A base64-encoded string.
        pub ca_certificate: pulumi_wasm_rust::Output<String>,
        /// Cloud KMS key name used for encrypting control plane data that is stored in a multi region.
        /// Only used for the data residency region "US" or "EU".
        pub control_plane_encryption_key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the Apigee organization.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Flag that specifies whether the VPC Peering through Private Google Access should be
        /// disabled between the consumer network and Apigee. Required if an `authorizedNetwork`
        /// on the consumer project is not provided, in which case the flag should be set to `true`.
        /// Valid only when `RuntimeType` is set to CLOUD. The value must be set before the creation
        /// of any Apigee runtime instance and can be updated only when there are no runtime instances.
        pub disable_vpc_peering: pulumi_wasm_rust::Output<Option<bool>>,
        /// The display name of the Apigee organization.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Output only. Name of the Apigee organization.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project ID associated with the Apigee organization.
        ///
        ///
        /// - - -
        pub project_id: pulumi_wasm_rust::Output<String>,
        /// Properties defined in the Apigee organization profile.
        /// Structure is documented below.
        pub properties: pulumi_wasm_rust::Output<
            super::super::types::apigee::OrganizationProperties,
        >,
        /// Optional. This setting is applicable only for organizations that are soft-deleted (i.e., BillingType
        /// is not EVALUATION). It controls how long Organization data will be retained after the initial delete
        /// operation completes. During this period, the Organization may be restored to its last known state.
        /// After this period, the Organization will no longer be able to be restored.
        /// Default value is `DELETION_RETENTION_UNSPECIFIED`.
        /// Possible values are: `DELETION_RETENTION_UNSPECIFIED`, `MINIMUM`.
        pub retention: pulumi_wasm_rust::Output<Option<String>>,
        /// Cloud KMS key name used for encrypting the data that is stored and replicated across runtime instances.
        /// Update is not allowed after the organization is created.
        /// If not specified, a Google-Managed encryption key will be used.
        /// Valid only when `RuntimeType` is CLOUD. For example: `projects/foo/locations/us/keyRings/bar/cryptoKeys/baz`.
        pub runtime_database_encryption_key_name: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Runtime type of the Apigee organization based on the Apigee subscription purchased.
        /// Default value is `CLOUD`.
        /// Possible values are: `CLOUD`, `HYBRID`.
        pub runtime_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Output only. Subscription type of the Apigee organization.
        /// Valid values include trial (free, limited, and for evaluation purposes only) or paid (full subscription has been purchased).
        pub subscription_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: OrganizationArgs) -> OrganizationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let analytics_region_binding = args.analytics_region.get_inner();
        let api_consumer_data_encryption_key_name_binding = args
            .api_consumer_data_encryption_key_name
            .get_inner();
        let api_consumer_data_location_binding = args
            .api_consumer_data_location
            .get_inner();
        let authorized_network_binding = args.authorized_network.get_inner();
        let billing_type_binding = args.billing_type.get_inner();
        let control_plane_encryption_key_name_binding = args
            .control_plane_encryption_key_name
            .get_inner();
        let description_binding = args.description.get_inner();
        let disable_vpc_peering_binding = args.disable_vpc_peering.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let project_id_binding = args.project_id.get_inner();
        let properties_binding = args.properties.get_inner();
        let retention_binding = args.retention.get_inner();
        let runtime_database_encryption_key_name_binding = args
            .runtime_database_encryption_key_name
            .get_inner();
        let runtime_type_binding = args.runtime_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/organization:Organization".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "analyticsRegion".into(),
                    value: &analytics_region_binding,
                },
                register_interface::ObjectField {
                    name: "apiConsumerDataEncryptionKeyName".into(),
                    value: &api_consumer_data_encryption_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "apiConsumerDataLocation".into(),
                    value: &api_consumer_data_location_binding,
                },
                register_interface::ObjectField {
                    name: "authorizedNetwork".into(),
                    value: &authorized_network_binding,
                },
                register_interface::ObjectField {
                    name: "billingType".into(),
                    value: &billing_type_binding,
                },
                register_interface::ObjectField {
                    name: "controlPlaneEncryptionKeyName".into(),
                    value: &control_plane_encryption_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disableVpcPeering".into(),
                    value: &disable_vpc_peering_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "projectId".into(),
                    value: &project_id_binding,
                },
                register_interface::ObjectField {
                    name: "properties".into(),
                    value: &properties_binding,
                },
                register_interface::ObjectField {
                    name: "retention".into(),
                    value: &retention_binding,
                },
                register_interface::ObjectField {
                    name: "runtimeDatabaseEncryptionKeyName".into(),
                    value: &runtime_database_encryption_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "runtimeType".into(),
                    value: &runtime_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "analyticsRegion".into(),
                },
                register_interface::ResultField {
                    name: "apiConsumerDataEncryptionKeyName".into(),
                },
                register_interface::ResultField {
                    name: "apiConsumerDataLocation".into(),
                },
                register_interface::ResultField {
                    name: "apigeeProjectId".into(),
                },
                register_interface::ResultField {
                    name: "authorizedNetwork".into(),
                },
                register_interface::ResultField {
                    name: "billingType".into(),
                },
                register_interface::ResultField {
                    name: "caCertificate".into(),
                },
                register_interface::ResultField {
                    name: "controlPlaneEncryptionKeyName".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disableVpcPeering".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "projectId".into(),
                },
                register_interface::ResultField {
                    name: "properties".into(),
                },
                register_interface::ResultField {
                    name: "retention".into(),
                },
                register_interface::ResultField {
                    name: "runtimeDatabaseEncryptionKeyName".into(),
                },
                register_interface::ResultField {
                    name: "runtimeType".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OrganizationResult {
            analytics_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("analyticsRegion").unwrap(),
            ),
            api_consumer_data_encryption_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiConsumerDataEncryptionKeyName").unwrap(),
            ),
            api_consumer_data_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiConsumerDataLocation").unwrap(),
            ),
            apigee_project_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apigeeProjectId").unwrap(),
            ),
            authorized_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizedNetwork").unwrap(),
            ),
            billing_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("billingType").unwrap(),
            ),
            ca_certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("caCertificate").unwrap(),
            ),
            control_plane_encryption_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlPlaneEncryptionKeyName").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disable_vpc_peering: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableVpcPeering").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("projectId").unwrap(),
            ),
            properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("properties").unwrap(),
            ),
            retention: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retention").unwrap(),
            ),
            runtime_database_encryption_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtimeDatabaseEncryptionKeyName").unwrap(),
            ),
            runtime_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtimeType").unwrap(),
            ),
            subscription_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionType").unwrap(),
            ),
        }
    }
}
