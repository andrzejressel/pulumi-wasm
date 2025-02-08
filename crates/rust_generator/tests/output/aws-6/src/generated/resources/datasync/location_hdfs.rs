/// Manages an HDFS Location within AWS DataSync.
///
/// > **NOTE:** The DataSync Agents must be available before creating this resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = location_hdfs::create(
///         "example",
///         LocationHdfsArgs::builder()
///             .agent_arns(vec!["${exampleAwsDatasyncAgent.arn}",])
///             .authentication_type("SIMPLE")
///             .name_nodes(
///                 vec![
///                     LocationHdfsNameNode::builder()
///                     .hostname("${exampleAwsInstance.privateDns}").port(80)
///                     .build_struct(),
///                 ],
///             )
///             .simple_user("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Kerberos Authentication
///
/// ```yaml
/// resources:
///   example:
///     type: aws:datasync:LocationHdfs
///     properties:
///       agentArns:
///         - ${exampleAwsDatasyncAgent.arn}
///       authenticationType: KERBEROS
///       nameNodes:
///         - hostname: ${exampleAwsInstance.privateDns}
///           port: 80
///       kerberosPrincipal: user@example.com
///       kerberosKeytabBase64:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: user.keytab
///           return: result
///       kerberosKrb5Conf:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: krb5.conf
///           return: result
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datasync_location_hdfs` using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:datasync/locationHdfs:LocationHdfs example arn:aws:datasync:us-east-1:123456789012:location/loc-12345678901234567
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod location_hdfs {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocationHdfsArgs {
        /// A list of DataSync Agent ARNs with which this location will be associated.
        #[builder(into)]
        pub agent_arns: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The type of authentication used to determine the identity of the user. Valid values are `SIMPLE` and `KERBEROS`.
        #[builder(into, default)]
        pub authentication_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The size of data blocks to write into the HDFS cluster. The block size must be a multiple of 512 bytes. The default block size is 128 mebibytes (MiB).
        #[builder(into, default)]
        pub block_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The Kerberos key table (keytab) that contains mappings between the defined Kerberos principal and the encrypted keys. Use `kerberos_keytab_base64` instead whenever the value is not a valid UTF-8 string. If `KERBEROS` is specified for `authentication_type`, this parameter (or `kerberos_keytab_base64`) is required.
        #[builder(into, default)]
        pub kerberos_keytab: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Use instead of `kerberos_keytab` to pass base64-encoded binary data directly. If `KERBEROS` is specified for `authentication_type`, this parameter (or `kerberos_keytab`) is required.
        #[builder(into, default)]
        pub kerberos_keytab_base64: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The krb5.conf file that contains the Kerberos configuration information. Use `kerberos_krb5_conf_base64` instead whenever the value is not a valid UTF-8 string. If `KERBEROS` is specified for `authentication_type`, this parameter (or `kerberos_krb5_conf_base64`) is required.
        #[builder(into, default)]
        pub kerberos_krb5_conf: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Use instead of `kerberos_krb5_conf` to pass base64-encoded binary data directly. If `KERBEROS` is specified for `authentication_type`, this parameter (or `kerberos_krb5_conf`) is required.
        #[builder(into, default)]
        pub kerberos_krb5_conf_base64: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Kerberos principal with access to the files and folders on the HDFS cluster. If `KERBEROS` is specified for `authentication_type`, this parameter is required.
        #[builder(into, default)]
        pub kerberos_principal: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URI of the HDFS cluster's Key Management Server (KMS).
        #[builder(into, default)]
        pub kms_key_provider_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The NameNode that manages the HDFS namespace. The NameNode performs operations such as opening, closing, and renaming files and directories. The NameNode contains the information to map blocks of data to the DataNodes. You can use only one NameNode. See configuration below.
        #[builder(into)]
        pub name_nodes: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::datasync::LocationHdfsNameNode>,
        >,
        /// The Quality of Protection (QOP) configuration specifies the Remote Procedure Call (RPC) and data transfer protection settings configured on the Hadoop Distributed File System (HDFS) cluster. If `qop_configuration` isn't specified, `rpc_protection` and `data_transfer_protection` default to `PRIVACY`. If you set RpcProtection or DataTransferProtection, the other parameter assumes the same value.  See configuration below.
        #[builder(into, default)]
        pub qop_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datasync::LocationHdfsQopConfiguration>,
        >,
        /// The number of DataNodes to replicate the data to when writing to the HDFS cluster. By default, data is replicated to three DataNodes.
        #[builder(into, default)]
        pub replication_factor: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The user name used to identify the client on the host operating system. If `SIMPLE` is specified for `authentication_type`, this parameter is required.
        #[builder(into, default)]
        pub simple_user: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A subdirectory in the HDFS cluster. This subdirectory is used to read data from or write data to the HDFS cluster. If the subdirectory isn't specified, it will default to /.
        #[builder(into, default)]
        pub subdirectory: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LocationHdfsResult {
        /// A list of DataSync Agent ARNs with which this location will be associated.
        pub agent_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Amazon Resource Name (ARN) of the DataSync Location.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The type of authentication used to determine the identity of the user. Valid values are `SIMPLE` and `KERBEROS`.
        pub authentication_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The size of data blocks to write into the HDFS cluster. The block size must be a multiple of 512 bytes. The default block size is 128 mebibytes (MiB).
        pub block_size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The Kerberos key table (keytab) that contains mappings between the defined Kerberos principal and the encrypted keys. Use `kerberos_keytab_base64` instead whenever the value is not a valid UTF-8 string. If `KERBEROS` is specified for `authentication_type`, this parameter (or `kerberos_keytab_base64`) is required.
        pub kerberos_keytab: pulumi_gestalt_rust::Output<Option<String>>,
        /// Use instead of `kerberos_keytab` to pass base64-encoded binary data directly. If `KERBEROS` is specified for `authentication_type`, this parameter (or `kerberos_keytab`) is required.
        pub kerberos_keytab_base64: pulumi_gestalt_rust::Output<Option<String>>,
        /// The krb5.conf file that contains the Kerberos configuration information. Use `kerberos_krb5_conf_base64` instead whenever the value is not a valid UTF-8 string. If `KERBEROS` is specified for `authentication_type`, this parameter (or `kerberos_krb5_conf_base64`) is required.
        pub kerberos_krb5_conf: pulumi_gestalt_rust::Output<Option<String>>,
        /// Use instead of `kerberos_krb5_conf` to pass base64-encoded binary data directly. If `KERBEROS` is specified for `authentication_type`, this parameter (or `kerberos_krb5_conf`) is required.
        pub kerberos_krb5_conf_base64: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Kerberos principal with access to the files and folders on the HDFS cluster. If `KERBEROS` is specified for `authentication_type`, this parameter is required.
        pub kerberos_principal: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URI of the HDFS cluster's Key Management Server (KMS).
        pub kms_key_provider_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// The NameNode that manages the HDFS namespace. The NameNode performs operations such as opening, closing, and renaming files and directories. The NameNode contains the information to map blocks of data to the DataNodes. You can use only one NameNode. See configuration below.
        pub name_nodes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datasync::LocationHdfsNameNode>,
        >,
        /// The Quality of Protection (QOP) configuration specifies the Remote Procedure Call (RPC) and data transfer protection settings configured on the Hadoop Distributed File System (HDFS) cluster. If `qop_configuration` isn't specified, `rpc_protection` and `data_transfer_protection` default to `PRIVACY`. If you set RpcProtection or DataTransferProtection, the other parameter assumes the same value.  See configuration below.
        pub qop_configuration: pulumi_gestalt_rust::Output<
            super::super::types::datasync::LocationHdfsQopConfiguration,
        >,
        /// The number of DataNodes to replicate the data to when writing to the HDFS cluster. By default, data is replicated to three DataNodes.
        pub replication_factor: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The user name used to identify the client on the host operating system. If `SIMPLE` is specified for `authentication_type`, this parameter is required.
        pub simple_user: pulumi_gestalt_rust::Output<Option<String>>,
        /// A subdirectory in the HDFS cluster. This subdirectory is used to read data from or write data to the HDFS cluster. If the subdirectory isn't specified, it will default to /.
        pub subdirectory: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub uri: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LocationHdfsArgs,
    ) -> LocationHdfsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let agent_arns_binding = args.agent_arns.get_output(context).get_inner();
        let authentication_type_binding = args
            .authentication_type
            .get_output(context)
            .get_inner();
        let block_size_binding = args.block_size.get_output(context).get_inner();
        let kerberos_keytab_binding = args
            .kerberos_keytab
            .get_output(context)
            .get_inner();
        let kerberos_keytab_base64_binding = args
            .kerberos_keytab_base64
            .get_output(context)
            .get_inner();
        let kerberos_krb5_conf_binding = args
            .kerberos_krb5_conf
            .get_output(context)
            .get_inner();
        let kerberos_krb5_conf_base64_binding = args
            .kerberos_krb5_conf_base64
            .get_output(context)
            .get_inner();
        let kerberos_principal_binding = args
            .kerberos_principal
            .get_output(context)
            .get_inner();
        let kms_key_provider_uri_binding = args
            .kms_key_provider_uri
            .get_output(context)
            .get_inner();
        let name_nodes_binding = args.name_nodes.get_output(context).get_inner();
        let qop_configuration_binding = args
            .qop_configuration
            .get_output(context)
            .get_inner();
        let replication_factor_binding = args
            .replication_factor
            .get_output(context)
            .get_inner();
        let simple_user_binding = args.simple_user.get_output(context).get_inner();
        let subdirectory_binding = args.subdirectory.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datasync/locationHdfs:LocationHdfs".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "agentArns".into(),
                    value: &agent_arns_binding,
                },
                register_interface::ObjectField {
                    name: "authenticationType".into(),
                    value: &authentication_type_binding,
                },
                register_interface::ObjectField {
                    name: "blockSize".into(),
                    value: &block_size_binding,
                },
                register_interface::ObjectField {
                    name: "kerberosKeytab".into(),
                    value: &kerberos_keytab_binding,
                },
                register_interface::ObjectField {
                    name: "kerberosKeytabBase64".into(),
                    value: &kerberos_keytab_base64_binding,
                },
                register_interface::ObjectField {
                    name: "kerberosKrb5Conf".into(),
                    value: &kerberos_krb5_conf_binding,
                },
                register_interface::ObjectField {
                    name: "kerberosKrb5ConfBase64".into(),
                    value: &kerberos_krb5_conf_base64_binding,
                },
                register_interface::ObjectField {
                    name: "kerberosPrincipal".into(),
                    value: &kerberos_principal_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyProviderUri".into(),
                    value: &kms_key_provider_uri_binding,
                },
                register_interface::ObjectField {
                    name: "nameNodes".into(),
                    value: &name_nodes_binding,
                },
                register_interface::ObjectField {
                    name: "qopConfiguration".into(),
                    value: &qop_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "replicationFactor".into(),
                    value: &replication_factor_binding,
                },
                register_interface::ObjectField {
                    name: "simpleUser".into(),
                    value: &simple_user_binding,
                },
                register_interface::ObjectField {
                    name: "subdirectory".into(),
                    value: &subdirectory_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LocationHdfsResult {
            agent_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("agentArns"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            authentication_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authenticationType"),
            ),
            block_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("blockSize"),
            ),
            kerberos_keytab: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kerberosKeytab"),
            ),
            kerberos_keytab_base64: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kerberosKeytabBase64"),
            ),
            kerberos_krb5_conf: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kerberosKrb5Conf"),
            ),
            kerberos_krb5_conf_base64: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kerberosKrb5ConfBase64"),
            ),
            kerberos_principal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kerberosPrincipal"),
            ),
            kms_key_provider_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyProviderUri"),
            ),
            name_nodes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nameNodes"),
            ),
            qop_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("qopConfiguration"),
            ),
            replication_factor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicationFactor"),
            ),
            simple_user: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("simpleUser"),
            ),
            subdirectory: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subdirectory"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            uri: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uri")),
        }
    }
}
