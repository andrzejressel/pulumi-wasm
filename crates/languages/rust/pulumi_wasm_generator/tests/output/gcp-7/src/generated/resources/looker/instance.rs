/// A Google Cloud Looker instance.
///
///
/// To get more information about Instance, see:
///
/// * [API documentation](https://cloud.google.com/looker/docs/reference/rest/v1/projects.locations.instances)
/// * How-to Guides
///     * [Configure a Looker (Google Cloud core) instance](https://cloud.google.com/looker/docs/looker-core-instance-setup)
///     * [Create a Looker (Google Cloud core) instance](https://cloud.google.com/looker/docs/looker-core-instance-create)
///
/// ## Example Usage
///
/// ### Looker Instance Basic
///
///
/// ```yaml
/// resources:
///   looker-instance:
///     type: gcp:looker:Instance
///     properties:
///       name: my-instance
///       platformEdition: LOOKER_CORE_STANDARD_ANNUAL
///       region: us-central1
///       oauthConfig:
///         clientId: my-client-id
///         clientSecret: my-client-secret
///       deletionPolicy: DEFAULT
/// ```
/// ### Looker Instance Full
///
///
/// ```yaml
/// resources:
///   looker-instance:
///     type: gcp:looker:Instance
///     properties:
///       name: my-instance
///       platformEdition: LOOKER_CORE_STANDARD_ANNUAL
///       region: us-central1
///       publicIpEnabled: true
///       adminSettings:
///         allowedEmailDomains:
///           - google.com
///       maintenanceWindow:
///         dayOfWeek: THURSDAY
///         startTime:
///           hours: 22
///           minutes: 0
///           seconds: 0
///           nanos: 0
///       denyMaintenancePeriod:
///         startDate:
///           year: 2050
///           month: 1
///           day: 1
///         endDate:
///           year: 2050
///           month: 2
///           day: 1
///         time:
///           hours: 10
///           minutes: 0
///           seconds: 0
///           nanos: 0
///       oauthConfig:
///         clientId: my-client-id
///         clientSecret: my-client-secret
/// ```
/// ### Looker Instance Fips
///
///
/// ```yaml
/// resources:
///   looker-instance:
///     type: gcp:looker:Instance
///     properties:
///       name: my-instance-fips
///       platformEdition: LOOKER_CORE_ENTERPRISE_ANNUAL
///       region: us-central1
///       publicIpEnabled: true
///       fipsEnabled: true
///       oauthConfig:
///         clientId: my-client-id
///         clientSecret: my-client-secret
/// ```
/// ### Looker Instance Enterprise Full
///
///
/// ```yaml
/// resources:
///   looker-instance:
///     type: gcp:looker:Instance
///     properties:
///       name: my-instance
///       platformEdition: LOOKER_CORE_ENTERPRISE_ANNUAL
///       region: us-central1
///       privateIpEnabled: true
///       publicIpEnabled: false
///       reservedRange: ${lookerRange.name}
///       consumerNetwork: ${lookerNetwork.id}
///       adminSettings:
///         allowedEmailDomains:
///           - google.com
///       encryptionConfig:
///         kmsKeyName: looker-kms-key
///       maintenanceWindow:
///         dayOfWeek: THURSDAY
///         startTime:
///           hours: 22
///           minutes: 0
///           seconds: 0
///           nanos: 0
///       denyMaintenancePeriod:
///         startDate:
///           year: 2050
///           month: 1
///           day: 1
///         endDate:
///           year: 2050
///           month: 2
///           day: 1
///         time:
///           hours: 10
///           minutes: 0
///           seconds: 0
///           nanos: 0
///       oauthConfig:
///         clientId: my-client-id
///         clientSecret: my-client-secret
///     options:
///       dependsOn:
///         - ${lookerVpcConnection}
///   lookerVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: looker_vpc_connection
///     properties:
///       network: ${lookerNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${lookerRange.name}
///   lookerRange:
///     type: gcp:compute:GlobalAddress
///     name: looker_range
///     properties:
///       name: looker-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 20
///       network: ${lookerNetwork.id}
///   lookerNetwork:
///     type: gcp:compute:Network
///     name: looker_network
///     properties:
///       name: looker-network
///   cryptoKey:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: crypto_key
///     properties:
///       cryptoKeyId: looker-kms-key
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:service-${project.number}@gcp-sa-looker.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Looker Instance Custom Domain
///
///
/// ```yaml
/// resources:
///   looker-instance:
///     type: gcp:looker:Instance
///     properties:
///       name: my-instance
///       platformEdition: LOOKER_CORE_STANDARD_ANNUAL
///       region: us-central1
///       oauthConfig:
///         clientId: my-client-id
///         clientSecret: my-client-secret
///       customDomain:
///         domain: my-custom-domain.com
/// ```
/// ### Looker Instance Psc
///
///
/// ```yaml
/// resources:
///   looker-instance:
///     type: gcp:looker:Instance
///     properties:
///       name: my-instance
///       platformEdition: LOOKER_CORE_ENTERPRISE_ANNUAL
///       region: us-central1
///       privateIpEnabled: false
///       publicIpEnabled: false
///       pscEnabled: true
///       oauthConfig:
///         clientId: my-client-id
///         clientSecret: my-client-secret
///       pscConfig:
///         allowedVpcs:
///           - projects/test-project/global/networks/test
/// ```
/// ### Looker Instance Force Delete
///
///
/// ```yaml
/// resources:
///   looker-instance:
///     type: gcp:looker:Instance
///     properties:
///       name: my-instance
///       platformEdition: LOOKER_CORE_STANDARD_ANNUAL
///       region: us-central1
///       oauthConfig:
///         clientId: my-client-id
///         clientSecret: my-client-secret
///       deletionPolicy: FORCE
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/instances/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:looker/instance:Instance default projects/{{project}}/locations/{{region}}/instances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:looker/instance:Instance default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:looker/instance:Instance default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:looker/instance:Instance default {{name}}
/// ```
///
pub mod instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// Looker instance Admin settings.
        #[builder(into, default)]
        pub admin_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::looker::InstanceAdminSettings>,
        >,
        /// Network name in the consumer project in the format of: projects/{project}/global/networks/{network} Note that the
        /// consumer network may be in a different GCP project than the consumer project that is hosting the Looker Instance.
        #[builder(into, default)]
        pub consumer_network: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Custom domain settings for a Looker instance.
        #[builder(into, default)]
        pub custom_domain: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::looker::InstanceCustomDomain>,
        >,
        /// Policy to determine if the cluster should be deleted forcefully. If setting deletion_policy = "FORCE", the Looker
        /// instance will be deleted regardless of its nested resources. If set to "DEFAULT", Looker instances that still have
        /// nested resources will return an error. Possible values: DEFAULT, FORCE
        #[builder(into, default)]
        pub deletion_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Maintenance denial period for this instance. You must allow at least 14 days of maintenance availability between any two
        /// deny maintenance periods.
        #[builder(into, default)]
        pub deny_maintenance_period: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::looker::InstanceDenyMaintenancePeriod>,
        >,
        /// Looker instance encryption settings.
        #[builder(into, default)]
        pub encryption_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::looker::InstanceEncryptionConfig>,
        >,
        /// FIPS 140-2 Encryption enablement for Looker (Google Cloud Core).
        #[builder(into, default)]
        pub fips_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Maintenance window for an instance. Maintenance of your instance takes place once a month, and will require your
        /// instance to be restarted during updates, which will temporarily disrupt service.
        #[builder(into, default)]
        pub maintenance_window: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::looker::InstanceMaintenanceWindow>,
        >,
        /// The ID of the instance or a fully qualified identifier for the instance.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Looker Instance OAuth login settings.
        /// Structure is documented below.
        #[builder(into)]
        pub oauth_config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::looker::InstanceOauthConfig,
        >,
        /// Platform editions for a Looker instance. Each edition maps to a set of instance features, like its size. Must be one of
        /// these values: - LOOKER_CORE_TRIAL: trial instance (Currently Unavailable) - LOOKER_CORE_STANDARD: pay as you go standard
        /// instance (Currently Unavailable) - LOOKER_CORE_STANDARD_ANNUAL: subscription standard instance -
        /// LOOKER_CORE_ENTERPRISE_ANNUAL: subscription enterprise instance - LOOKER_CORE_EMBED_ANNUAL: subscription embed instance
        /// - LOOKER_CORE_NONPROD_STANDARD_ANNUAL: nonprod subscription standard instance - LOOKER_CORE_NONPROD_ENTERPRISE_ANNUAL:
        /// nonprod subscription enterprise instance - LOOKER_CORE_NONPROD_EMBED_ANNUAL: nonprod subscription embed instance Default
        /// value: "LOOKER_CORE_TRIAL" Possible values: ["LOOKER_CORE_TRIAL", "LOOKER_CORE_STANDARD", "LOOKER_CORE_STANDARD_ANNUAL",
        /// "LOOKER_CORE_ENTERPRISE_ANNUAL", "LOOKER_CORE_EMBED_ANNUAL", "LOOKER_CORE_NONPROD_STANDARD_ANNUAL",
        /// "LOOKER_CORE_NONPROD_ENTERPRISE_ANNUAL", "LOOKER_CORE_NONPROD_EMBED_ANNUAL"]
        #[builder(into, default)]
        pub platform_edition: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether private IP is enabled on the Looker instance.
        #[builder(into, default)]
        pub private_ip_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Information for Private Service Connect (PSC) setup for a Looker instance.
        #[builder(into, default)]
        pub psc_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::looker::InstancePscConfig>,
        >,
        /// Whether Public Service Connect (PSC) is enabled on the Looker instance
        #[builder(into, default)]
        pub psc_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether public IP is enabled on the Looker instance.
        #[builder(into, default)]
        pub public_ip_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the Looker region of the instance.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of a reserved IP address range within the consumer network, to be used for private service access connection. User
        /// may or may not specify this in a request.
        #[builder(into, default)]
        pub reserved_range: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Metadata about users for a Looker instance. These settings are only available when platform edition LOOKER_CORE_STANDARD
        /// is set. There are ten Standard and two Developer users included in the cost of the product. You can allocate additional
        /// Standard, Viewer, and Developer users for this instance. It is an optional step and can be modified later. With the
        /// Standard edition of Looker (Google Cloud core), you can provision up to 50 total users, distributed across Viewer,
        /// Standard, and Developer.
        #[builder(into, default)]
        pub user_metadata: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::looker::InstanceUserMetadata>,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// Looker instance Admin settings.
        pub admin_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::looker::InstanceAdminSettings>,
        >,
        /// Network name in the consumer project in the format of: projects/{project}/global/networks/{network} Note that the
        /// consumer network may be in a different GCP project than the consumer project that is hosting the Looker Instance.
        pub consumer_network: pulumi_wasm_rust::Output<Option<String>>,
        /// The time the instance was created in RFC3339 UTC "Zulu" format,
        /// accurate to nanoseconds.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Custom domain settings for a Looker instance.
        pub custom_domain: pulumi_wasm_rust::Output<
            Option<super::super::types::looker::InstanceCustomDomain>,
        >,
        /// Policy to determine if the cluster should be deleted forcefully. If setting deletion_policy = "FORCE", the Looker
        /// instance will be deleted regardless of its nested resources. If set to "DEFAULT", Looker instances that still have
        /// nested resources will return an error. Possible values: DEFAULT, FORCE
        pub deletion_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Maintenance denial period for this instance. You must allow at least 14 days of maintenance availability between any two
        /// deny maintenance periods.
        pub deny_maintenance_period: pulumi_wasm_rust::Output<
            Option<super::super::types::looker::InstanceDenyMaintenancePeriod>,
        >,
        /// Public Egress IP (IPv4).
        pub egress_public_ip: pulumi_wasm_rust::Output<String>,
        /// Looker instance encryption settings.
        pub encryption_config: pulumi_wasm_rust::Output<
            super::super::types::looker::InstanceEncryptionConfig,
        >,
        /// FIPS 140-2 Encryption enablement for Looker (Google Cloud Core).
        pub fips_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Private Ingress IP (IPv4).
        pub ingress_private_ip: pulumi_wasm_rust::Output<String>,
        /// Public Ingress IP (IPv4).
        pub ingress_public_ip: pulumi_wasm_rust::Output<String>,
        /// Looker instance URI which can be used to access the Looker Instance UI.
        pub looker_uri: pulumi_wasm_rust::Output<String>,
        /// The Looker version that the instance is using.
        pub looker_version: pulumi_wasm_rust::Output<String>,
        /// Maintenance window for an instance. Maintenance of your instance takes place once a month, and will require your
        /// instance to be restarted during updates, which will temporarily disrupt service.
        pub maintenance_window: pulumi_wasm_rust::Output<
            Option<super::super::types::looker::InstanceMaintenanceWindow>,
        >,
        /// The ID of the instance or a fully qualified identifier for the instance.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Looker Instance OAuth login settings.
        /// Structure is documented below.
        pub oauth_config: pulumi_wasm_rust::Output<
            super::super::types::looker::InstanceOauthConfig,
        >,
        /// Platform editions for a Looker instance. Each edition maps to a set of instance features, like its size. Must be one of
        /// these values: - LOOKER_CORE_TRIAL: trial instance (Currently Unavailable) - LOOKER_CORE_STANDARD: pay as you go standard
        /// instance (Currently Unavailable) - LOOKER_CORE_STANDARD_ANNUAL: subscription standard instance -
        /// LOOKER_CORE_ENTERPRISE_ANNUAL: subscription enterprise instance - LOOKER_CORE_EMBED_ANNUAL: subscription embed instance
        /// - LOOKER_CORE_NONPROD_STANDARD_ANNUAL: nonprod subscription standard instance - LOOKER_CORE_NONPROD_ENTERPRISE_ANNUAL:
        /// nonprod subscription enterprise instance - LOOKER_CORE_NONPROD_EMBED_ANNUAL: nonprod subscription embed instance Default
        /// value: "LOOKER_CORE_TRIAL" Possible values: ["LOOKER_CORE_TRIAL", "LOOKER_CORE_STANDARD", "LOOKER_CORE_STANDARD_ANNUAL",
        /// "LOOKER_CORE_ENTERPRISE_ANNUAL", "LOOKER_CORE_EMBED_ANNUAL", "LOOKER_CORE_NONPROD_STANDARD_ANNUAL",
        /// "LOOKER_CORE_NONPROD_ENTERPRISE_ANNUAL", "LOOKER_CORE_NONPROD_EMBED_ANNUAL"]
        pub platform_edition: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether private IP is enabled on the Looker instance.
        pub private_ip_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// Information for Private Service Connect (PSC) setup for a Looker instance.
        pub psc_config: pulumi_wasm_rust::Output<
            Option<super::super::types::looker::InstancePscConfig>,
        >,
        /// Whether Public Service Connect (PSC) is enabled on the Looker instance
        pub psc_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether public IP is enabled on the Looker instance.
        pub public_ip_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Looker region of the instance.
        pub region: pulumi_wasm_rust::Output<String>,
        /// Name of a reserved IP address range within the consumer network, to be used for private service access connection. User
        /// may or may not specify this in a request.
        pub reserved_range: pulumi_wasm_rust::Output<Option<String>>,
        /// The time the instance was updated in RFC3339 UTC "Zulu" format,
        /// accurate to nanoseconds.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// Metadata about users for a Looker instance. These settings are only available when platform edition LOOKER_CORE_STANDARD
        /// is set. There are ten Standard and two Developer users included in the cost of the product. You can allocate additional
        /// Standard, Viewer, and Developer users for this instance. It is an optional step and can be modified later. With the
        /// Standard edition of Looker (Google Cloud core), you can provision up to 50 total users, distributed across Viewer,
        /// Standard, and Developer.
        pub user_metadata: pulumi_wasm_rust::Output<
            Option<super::super::types::looker::InstanceUserMetadata>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InstanceArgs,
    ) -> InstanceResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admin_settings_binding = args.admin_settings.get_output(context).get_inner();
        let consumer_network_binding = args
            .consumer_network
            .get_output(context)
            .get_inner();
        let custom_domain_binding = args.custom_domain.get_output(context).get_inner();
        let deletion_policy_binding = args
            .deletion_policy
            .get_output(context)
            .get_inner();
        let deny_maintenance_period_binding = args
            .deny_maintenance_period
            .get_output(context)
            .get_inner();
        let encryption_config_binding = args
            .encryption_config
            .get_output(context)
            .get_inner();
        let fips_enabled_binding = args.fips_enabled.get_output(context).get_inner();
        let maintenance_window_binding = args
            .maintenance_window
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let oauth_config_binding = args.oauth_config.get_output(context).get_inner();
        let platform_edition_binding = args
            .platform_edition
            .get_output(context)
            .get_inner();
        let private_ip_enabled_binding = args
            .private_ip_enabled
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let psc_config_binding = args.psc_config.get_output(context).get_inner();
        let psc_enabled_binding = args.psc_enabled.get_output(context).get_inner();
        let public_ip_enabled_binding = args
            .public_ip_enabled
            .get_output(context)
            .get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let reserved_range_binding = args.reserved_range.get_output(context).get_inner();
        let user_metadata_binding = args.user_metadata.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:looker/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "adminSettings".into(),
                    value: &admin_settings_binding,
                },
                register_interface::ObjectField {
                    name: "consumerNetwork".into(),
                    value: &consumer_network_binding,
                },
                register_interface::ObjectField {
                    name: "customDomain".into(),
                    value: &custom_domain_binding,
                },
                register_interface::ObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding,
                },
                register_interface::ObjectField {
                    name: "denyMaintenancePeriod".into(),
                    value: &deny_maintenance_period_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionConfig".into(),
                    value: &encryption_config_binding,
                },
                register_interface::ObjectField {
                    name: "fipsEnabled".into(),
                    value: &fips_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceWindow".into(),
                    value: &maintenance_window_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "oauthConfig".into(),
                    value: &oauth_config_binding,
                },
                register_interface::ObjectField {
                    name: "platformEdition".into(),
                    value: &platform_edition_binding,
                },
                register_interface::ObjectField {
                    name: "privateIpEnabled".into(),
                    value: &private_ip_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "pscConfig".into(),
                    value: &psc_config_binding,
                },
                register_interface::ObjectField {
                    name: "pscEnabled".into(),
                    value: &psc_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "publicIpEnabled".into(),
                    value: &public_ip_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "reservedRange".into(),
                    value: &reserved_range_binding,
                },
                register_interface::ObjectField {
                    name: "userMetadata".into(),
                    value: &user_metadata_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceResult {
            admin_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminSettings"),
            ),
            consumer_network: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("consumerNetwork"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            custom_domain: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customDomain"),
            ),
            deletion_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deletionPolicy"),
            ),
            deny_maintenance_period: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("denyMaintenancePeriod"),
            ),
            egress_public_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("egressPublicIp"),
            ),
            encryption_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryptionConfig"),
            ),
            fips_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fipsEnabled"),
            ),
            ingress_private_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ingressPrivateIp"),
            ),
            ingress_public_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ingressPublicIp"),
            ),
            looker_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lookerUri"),
            ),
            looker_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lookerVersion"),
            ),
            maintenance_window: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maintenanceWindow"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            oauth_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("oauthConfig"),
            ),
            platform_edition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("platformEdition"),
            ),
            private_ip_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateIpEnabled"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            psc_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pscConfig"),
            ),
            psc_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pscEnabled"),
            ),
            public_ip_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicIpEnabled"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            reserved_range: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reservedRange"),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            user_metadata: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userMetadata"),
            ),
        }
    }
}
