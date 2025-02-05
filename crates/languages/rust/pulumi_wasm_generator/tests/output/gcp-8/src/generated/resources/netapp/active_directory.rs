/// ActiveDirectory is the public representation of the active directory config.
///
///
/// To get more information about ActiveDirectory, see:
///
/// * [API documentation](https://cloud.google.com/netapp/volumes/docs/reference/rest/v1/projects.locations.activeDirectories)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/netapp/volumes/docs/configure-and-use/active-directory/about-ad)
///
///
///
/// ## Example Usage
///
/// ### Netapp Active Directory Full
///
///
/// ```yaml
/// resources:
///   testActiveDirectoryFull:
///     type: gcp:netapp:ActiveDirectory
///     name: test_active_directory_full
///     properties:
///       name: test-active-directory-full
///       location: us-central1
///       domain: ad.internal
///       dns: 172.30.64.3
///       netBiosPrefix: smbserver
///       username: user
///       password: pass
///       aesEncryption: false
///       backupOperators:
///         - test1
///         - test2
///       administrators:
///         - test1
///         - test2
///       description: ActiveDirectory is the public representation of the active directory config.
///       encryptDcConnections: false
///       kdcHostname: hostname
///       kdcIp: 10.10.0.11
///       labels:
///         foo: bar
///       ldapSigning: false
///       nfsUsersWithLdap: false
///       organizationalUnit: CN=Computers
///       securityOperators:
///         - test1
///         - test2
///       site: test-site
/// ```
///
/// ## Import
///
/// ActiveDirectory can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/activeDirectories/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, ActiveDirectory can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:netapp/activeDirectory:ActiveDirectory default projects/{{project}}/locations/{{location}}/activeDirectories/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/activeDirectory:ActiveDirectory default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/activeDirectory:ActiveDirectory default {{location}}/{{name}}
/// ```
///
pub mod active_directory {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ActiveDirectoryArgs {
        /// Domain user accounts to be added to the local Administrators group of the SMB service. Comma-separated list of domain users or groups. The Domain Admin group is automatically added when the service joins your domain as a hidden group.
        #[builder(into, default)]
        pub administrators: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Enables AES-128 and AES-256 encryption for Kerberos-based communication with Active Directory.
        #[builder(into, default)]
        pub aes_encryption: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Domain user/group accounts to be added to the Backup Operators group of the SMB service. The Backup Operators group allows members to backup and restore files regardless of whether they have read or write access to the files. Comma-separated list.
        #[builder(into, default)]
        pub backup_operators: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Comma separated list of DNS server IP addresses for the Active Directory domain.
        #[builder(into)]
        pub dns: pulumi_wasm_rust::InputOrOutput<String>,
        /// Fully qualified domain name for the Active Directory domain.
        #[builder(into)]
        pub domain: pulumi_wasm_rust::InputOrOutput<String>,
        /// If enabled, traffic between the SMB server to Domain Controller (DC) will be encrypted.
        #[builder(into, default)]
        pub encrypt_dc_connections: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Hostname of the Active Directory server used as Kerberos Key Distribution Center. Only required for volumes using kerberized NFSv4.1
        #[builder(into, default)]
        pub kdc_hostname: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// IP address of the Active Directory server used as Kerberos Key Distribution Center.
        #[builder(into, default)]
        pub kdc_ip: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies whether or not the LDAP traffic needs to be signed.
        #[builder(into, default)]
        pub ldap_signing: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Name of the region for the policy to apply to.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource name of the Active Directory pool. Needs to be unique per location.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// NetBIOS name prefix of the server to be created.
        /// A five-character random ID is generated automatically, for example, -6f9a, and appended to the prefix. The full UNC share path will have the following format:
        /// `\\NetBIOS_PREFIX-ABCD.DOMAIN_NAME\SHARE_NAME`
        #[builder(into)]
        pub net_bios_prefix: pulumi_wasm_rust::InputOrOutput<String>,
        /// Local UNIX users on clients without valid user information in Active Directory are blocked from access to LDAP enabled volumes.
        /// This option can be used to temporarily switch such volumes to AUTH_SYS authentication (user ID + 1-16 groups).
        #[builder(into, default)]
        pub nfs_users_with_ldap: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Name of the Organizational Unit where you intend to create the computer account for NetApp Volumes.
        /// Defaults to `CN=Computers` if left empty.
        #[builder(into, default)]
        pub organizational_unit: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub password: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Domain accounts that require elevated privileges such as `SeSecurityPrivilege` to manage security logs. Comma-separated list.
        #[builder(into, default)]
        pub security_operators: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies an Active Directory site to manage domain controller selection.
        /// Use when Active Directory domain controllers in multiple regions are configured. Defaults to `Default-First-Site-Name` if left empty.
        #[builder(into, default)]
        pub site: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Username for the Active Directory account with permissions to create the compute account within the specified organizational unit.
        #[builder(into)]
        pub username: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ActiveDirectoryResult {
        /// Domain user accounts to be added to the local Administrators group of the SMB service. Comma-separated list of domain users or groups. The Domain Admin group is automatically added when the service joins your domain as a hidden group.
        pub administrators: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Enables AES-128 and AES-256 encryption for Kerberos-based communication with Active Directory.
        pub aes_encryption: pulumi_wasm_rust::Output<Option<bool>>,
        /// Domain user/group accounts to be added to the Backup Operators group of the SMB service. The Backup Operators group allows members to backup and restore files regardless of whether they have read or write access to the files. Comma-separated list.
        pub backup_operators: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Create time of the active directory. A timestamp in RFC3339 UTC "Zulu" format. Examples: "2023-06-22T09:13:01.617Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Comma separated list of DNS server IP addresses for the Active Directory domain.
        pub dns: pulumi_wasm_rust::Output<String>,
        /// Fully qualified domain name for the Active Directory domain.
        pub domain: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If enabled, traffic between the SMB server to Domain Controller (DC) will be encrypted.
        pub encrypt_dc_connections: pulumi_wasm_rust::Output<Option<bool>>,
        /// Hostname of the Active Directory server used as Kerberos Key Distribution Center. Only required for volumes using kerberized NFSv4.1
        pub kdc_hostname: pulumi_wasm_rust::Output<Option<String>>,
        /// IP address of the Active Directory server used as Kerberos Key Distribution Center.
        pub kdc_ip: pulumi_wasm_rust::Output<Option<String>>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies whether or not the LDAP traffic needs to be signed.
        pub ldap_signing: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the region for the policy to apply to.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name of the Active Directory pool. Needs to be unique per location.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// NetBIOS name prefix of the server to be created.
        /// A five-character random ID is generated automatically, for example, -6f9a, and appended to the prefix. The full UNC share path will have the following format:
        /// `\\NetBIOS_PREFIX-ABCD.DOMAIN_NAME\SHARE_NAME`
        pub net_bios_prefix: pulumi_wasm_rust::Output<String>,
        /// Local UNIX users on clients without valid user information in Active Directory are blocked from access to LDAP enabled volumes.
        /// This option can be used to temporarily switch such volumes to AUTH_SYS authentication (user ID + 1-16 groups).
        pub nfs_users_with_ldap: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the Organizational Unit where you intend to create the computer account for NetApp Volumes.
        /// Defaults to `CN=Computers` if left empty.
        pub organizational_unit: pulumi_wasm_rust::Output<String>,
        pub password: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Domain accounts that require elevated privileges such as `SeSecurityPrivilege` to manage security logs. Comma-separated list.
        pub security_operators: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies an Active Directory site to manage domain controller selection.
        /// Use when Active Directory domain controllers in multiple regions are configured. Defaults to `Default-First-Site-Name` if left empty.
        pub site: pulumi_wasm_rust::Output<Option<String>>,
        /// The state of the Active Directory policy (not the Active Directory itself).
        pub state: pulumi_wasm_rust::Output<String>,
        /// The state details of the Active Directory.
        pub state_details: pulumi_wasm_rust::Output<String>,
        /// Username for the Active Directory account with permissions to create the compute account within the specified organizational unit.
        pub username: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ActiveDirectoryArgs,
    ) -> ActiveDirectoryResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let administrators_binding = args.administrators.get_output(context).get_inner();
        let aes_encryption_binding = args.aes_encryption.get_output(context).get_inner();
        let backup_operators_binding = args
            .backup_operators
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let dns_binding = args.dns.get_output(context).get_inner();
        let domain_binding = args.domain.get_output(context).get_inner();
        let encrypt_dc_connections_binding = args
            .encrypt_dc_connections
            .get_output(context)
            .get_inner();
        let kdc_hostname_binding = args.kdc_hostname.get_output(context).get_inner();
        let kdc_ip_binding = args.kdc_ip.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let ldap_signing_binding = args.ldap_signing.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let net_bios_prefix_binding = args
            .net_bios_prefix
            .get_output(context)
            .get_inner();
        let nfs_users_with_ldap_binding = args
            .nfs_users_with_ldap
            .get_output(context)
            .get_inner();
        let organizational_unit_binding = args
            .organizational_unit
            .get_output(context)
            .get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let security_operators_binding = args
            .security_operators
            .get_output(context)
            .get_inner();
        let site_binding = args.site.get_output(context).get_inner();
        let username_binding = args.username.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:netapp/activeDirectory:ActiveDirectory".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "administrators".into(),
                    value: &administrators_binding,
                },
                register_interface::ObjectField {
                    name: "aesEncryption".into(),
                    value: &aes_encryption_binding,
                },
                register_interface::ObjectField {
                    name: "backupOperators".into(),
                    value: &backup_operators_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "dns".into(),
                    value: &dns_binding,
                },
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "encryptDcConnections".into(),
                    value: &encrypt_dc_connections_binding,
                },
                register_interface::ObjectField {
                    name: "kdcHostname".into(),
                    value: &kdc_hostname_binding,
                },
                register_interface::ObjectField {
                    name: "kdcIp".into(),
                    value: &kdc_ip_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "ldapSigning".into(),
                    value: &ldap_signing_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "netBiosPrefix".into(),
                    value: &net_bios_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "nfsUsersWithLdap".into(),
                    value: &nfs_users_with_ldap_binding,
                },
                register_interface::ObjectField {
                    name: "organizationalUnit".into(),
                    value: &organizational_unit_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "securityOperators".into(),
                    value: &security_operators_binding,
                },
                register_interface::ObjectField {
                    name: "site".into(),
                    value: &site_binding,
                },
                register_interface::ObjectField {
                    name: "username".into(),
                    value: &username_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ActiveDirectoryResult {
            administrators: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("administrators"),
            ),
            aes_encryption: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("aesEncryption"),
            ),
            backup_operators: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupOperators"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            dns: pulumi_wasm_rust::__private::into_domain(o.extract_field("dns")),
            domain: pulumi_wasm_rust::__private::into_domain(o.extract_field("domain")),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            encrypt_dc_connections: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryptDcConnections"),
            ),
            kdc_hostname: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kdcHostname"),
            ),
            kdc_ip: pulumi_wasm_rust::__private::into_domain(o.extract_field("kdcIp")),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            ldap_signing: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ldapSigning"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            net_bios_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("netBiosPrefix"),
            ),
            nfs_users_with_ldap: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nfsUsersWithLdap"),
            ),
            organizational_unit: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("organizationalUnit"),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            security_operators: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityOperators"),
            ),
            site: pulumi_wasm_rust::__private::into_domain(o.extract_field("site")),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            state_details: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stateDetails"),
            ),
            username: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("username"),
            ),
        }
    }
}
