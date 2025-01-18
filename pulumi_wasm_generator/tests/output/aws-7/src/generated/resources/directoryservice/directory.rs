/// Provides a Simple or Managed Microsoft directory in AWS Directory Service.
///
/// ## Example Usage
///
/// ### SimpleAD
///
/// ```yaml
/// resources:
///   bar:
///     type: aws:directoryservice:Directory
///     properties:
///       name: corp.notexample.com
///       password: SuperSecretPassw0rd
///       size: Small
///       vpcSettings:
///         vpcId: ${main.id}
///         subnetIds:
///           - ${foo.id}
///           - ${barSubnet.id}
///       tags:
///         Project: foo
///   main:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.0.0.0/16
///   foo:
///     type: aws:ec2:Subnet
///     properties:
///       vpcId: ${main.id}
///       availabilityZone: us-west-2a
///       cidrBlock: 10.0.1.0/24
///   barSubnet:
///     type: aws:ec2:Subnet
///     name: bar
///     properties:
///       vpcId: ${main.id}
///       availabilityZone: us-west-2b
///       cidrBlock: 10.0.2.0/24
/// ```
///
/// ### Microsoft Active Directory (MicrosoftAD)
///
/// ```yaml
/// resources:
///   bar:
///     type: aws:directoryservice:Directory
///     properties:
///       name: corp.notexample.com
///       password: SuperSecretPassw0rd
///       edition: Standard
///       type: MicrosoftAD
///       vpcSettings:
///         vpcId: ${main.id}
///         subnetIds:
///           - ${foo.id}
///           - ${barSubnet.id}
///       tags:
///         Project: foo
///   main:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.0.0.0/16
///   foo:
///     type: aws:ec2:Subnet
///     properties:
///       vpcId: ${main.id}
///       availabilityZone: us-west-2a
///       cidrBlock: 10.0.1.0/24
///   barSubnet:
///     type: aws:ec2:Subnet
///     name: bar
///     properties:
///       vpcId: ${main.id}
///       availabilityZone: us-west-2b
///       cidrBlock: 10.0.2.0/24
/// ```
///
/// ### Microsoft Active Directory Connector (ADConnector)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bar = subnet::create(
///         "bar",
///         SubnetArgs::builder()
///             .availability_zone("us-west-2b")
///             .cidr_block("10.0.2.0/24")
///             .vpc_id("${main.id}")
///             .build_struct(),
///     );
///     let connector = directory::create(
///         "connector",
///         DirectoryArgs::builder()
///             .connect_settings(
///                 DirectoryConnectSettings::builder()
///                     .customerDnsIps(vec!["A.B.C.D",])
///                     .customerUsername("Admin")
///                     .subnetIds(vec!["${foo.id}", "${bar.id}",])
///                     .vpcId("${main.id}")
///                     .build_struct(),
///             )
///             .name("corp.notexample.com")
///             .password("SuperSecretPassw0rd")
///             .size("Small")
///             .type_("ADConnector")
///             .build_struct(),
///     );
///     let foo = subnet::create(
///         "foo",
///         SubnetArgs::builder()
///             .availability_zone("us-west-2a")
///             .cidr_block("10.0.1.0/24")
///             .vpc_id("${main.id}")
///             .build_struct(),
///     );
///     let main = vpc::create(
///         "main",
///         VpcArgs::builder().cidr_block("10.0.0.0/16").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DirectoryService directories using the directory `id`. For example:
///
/// ```sh
/// $ pulumi import aws:directoryservice/directory:Directory sample d-926724cf57
/// ```
pub mod directory {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DirectoryArgs {
        /// The alias for the directory (must be unique amongst all aliases in AWS). Required for `enable_sso`.
        #[builder(into, default)]
        pub alias: pulumi_wasm_rust::Output<Option<String>>,
        /// Connector related information about the directory. Fields documented below.
        #[builder(into, default)]
        pub connect_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::directoryservice::DirectoryConnectSettings>,
        >,
        /// A textual description for the directory.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of domain controllers desired in the directory. Minimum value of `2`. Scaling of domain controllers is only supported for `MicrosoftAD` directories.
        #[builder(into, default)]
        pub desired_number_of_domain_controllers: pulumi_wasm_rust::Output<Option<i32>>,
        /// The MicrosoftAD edition (`Standard` or `Enterprise`). Defaults to `Enterprise`.
        #[builder(into, default)]
        pub edition: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to enable single-sign on for the directory. Requires `alias`. Defaults to `false`.
        #[builder(into, default)]
        pub enable_sso: pulumi_wasm_rust::Output<Option<bool>>,
        /// The fully qualified name for the directory, such as `corp.example.com`
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The password for the directory administrator or connector user.
        #[builder(into)]
        pub password: pulumi_wasm_rust::Output<String>,
        /// The short name of the directory, such as `CORP`.
        #[builder(into, default)]
        pub short_name: pulumi_wasm_rust::Output<Option<String>>,
        /// (For `SimpleAD` and `ADConnector` types) The size of the directory (`Small` or `Large` are accepted values). `Large` by default.
        #[builder(into, default)]
        pub size: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The directory type (`SimpleAD`, `ADConnector` or `MicrosoftAD` are accepted values). Defaults to `SimpleAD`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// VPC related information about the directory. Fields documented below.
        #[builder(into, default)]
        pub vpc_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::directoryservice::DirectoryVpcSettings>,
        >,
    }
    #[allow(dead_code)]
    pub struct DirectoryResult {
        /// The access URL for the directory, such as `http://alias.awsapps.com`.
        pub access_url: pulumi_wasm_rust::Output<String>,
        /// The alias for the directory (must be unique amongst all aliases in AWS). Required for `enable_sso`.
        pub alias: pulumi_wasm_rust::Output<String>,
        /// Connector related information about the directory. Fields documented below.
        pub connect_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::directoryservice::DirectoryConnectSettings>,
        >,
        /// A textual description for the directory.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of domain controllers desired in the directory. Minimum value of `2`. Scaling of domain controllers is only supported for `MicrosoftAD` directories.
        pub desired_number_of_domain_controllers: pulumi_wasm_rust::Output<i32>,
        /// A list of IP addresses of the DNS servers for the directory or connector.
        pub dns_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The MicrosoftAD edition (`Standard` or `Enterprise`). Defaults to `Enterprise`.
        pub edition: pulumi_wasm_rust::Output<String>,
        /// Whether to enable single-sign on for the directory. Requires `alias`. Defaults to `false`.
        pub enable_sso: pulumi_wasm_rust::Output<Option<bool>>,
        /// The fully qualified name for the directory, such as `corp.example.com`
        pub name: pulumi_wasm_rust::Output<String>,
        /// The password for the directory administrator or connector user.
        pub password: pulumi_wasm_rust::Output<String>,
        /// The ID of the security group created by the directory.
        pub security_group_id: pulumi_wasm_rust::Output<String>,
        /// The short name of the directory, such as `CORP`.
        pub short_name: pulumi_wasm_rust::Output<String>,
        /// (For `SimpleAD` and `ADConnector` types) The size of the directory (`Small` or `Large` are accepted values). `Large` by default.
        pub size: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The directory type (`SimpleAD`, `ADConnector` or `MicrosoftAD` are accepted values). Defaults to `SimpleAD`.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// VPC related information about the directory. Fields documented below.
        pub vpc_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::directoryservice::DirectoryVpcSettings>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DirectoryArgs) -> DirectoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_binding = args.alias.get_inner();
        let connect_settings_binding = args.connect_settings.get_inner();
        let description_binding = args.description.get_inner();
        let desired_number_of_domain_controllers_binding = args
            .desired_number_of_domain_controllers
            .get_inner();
        let edition_binding = args.edition.get_inner();
        let enable_sso_binding = args.enable_sso.get_inner();
        let name_binding = args.name.get_inner();
        let password_binding = args.password.get_inner();
        let short_name_binding = args.short_name.get_inner();
        let size_binding = args.size.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let vpc_settings_binding = args.vpc_settings.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directoryservice/directory:Directory".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alias".into(),
                    value: &alias_binding,
                },
                register_interface::ObjectField {
                    name: "connectSettings".into(),
                    value: &connect_settings_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "desiredNumberOfDomainControllers".into(),
                    value: &desired_number_of_domain_controllers_binding,
                },
                register_interface::ObjectField {
                    name: "edition".into(),
                    value: &edition_binding,
                },
                register_interface::ObjectField {
                    name: "enableSso".into(),
                    value: &enable_sso_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "shortName".into(),
                    value: &short_name_binding,
                },
                register_interface::ObjectField {
                    name: "size".into(),
                    value: &size_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "vpcSettings".into(),
                    value: &vpc_settings_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessUrl".into(),
                },
                register_interface::ResultField {
                    name: "alias".into(),
                },
                register_interface::ResultField {
                    name: "connectSettings".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "desiredNumberOfDomainControllers".into(),
                },
                register_interface::ResultField {
                    name: "dnsIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "edition".into(),
                },
                register_interface::ResultField {
                    name: "enableSso".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupId".into(),
                },
                register_interface::ResultField {
                    name: "shortName".into(),
                },
                register_interface::ResultField {
                    name: "size".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "vpcSettings".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DirectoryResult {
            access_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessUrl").unwrap(),
            ),
            alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alias").unwrap(),
            ),
            connect_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectSettings").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            desired_number_of_domain_controllers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desiredNumberOfDomainControllers").unwrap(),
            ),
            dns_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsIpAddresses").unwrap(),
            ),
            edition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edition").unwrap(),
            ),
            enable_sso: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableSso").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
            security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupId").unwrap(),
            ),
            short_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shortName").unwrap(),
            ),
            size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("size").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            vpc_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcSettings").unwrap(),
            ),
        }
    }
}
