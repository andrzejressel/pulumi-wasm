/// Manages a Object Storage Location within AWS DataSync.
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
///     let example = location_object_storage::create(
///         "example",
///         LocationObjectStorageArgs::builder()
///             .agent_arns(vec!["${exampleAwsDatasyncAgent.arn}",])
///             .bucket_name("example")
///             .server_hostname("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datasync_location_object_storage` using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:datasync/locationObjectStorage:LocationObjectStorage example arn:aws:datasync:us-east-1:123456789012:location/loc-12345678901234567
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod location_object_storage {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocationObjectStorageArgs {
        /// The access key is used if credentials are required to access the self-managed object storage server. If your object storage requires a user name and password to authenticate, use `access_key` and `secret_key` to provide the user name and password, respectively.
        #[builder(into, default)]
        pub access_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of DataSync Agent ARNs with which this location will be associated.
        #[builder(into)]
        pub agent_arns: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The bucket on the self-managed object storage server that is used to read data from.
        #[builder(into)]
        pub bucket_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The secret key is used if credentials are required to access the self-managed object storage server. If your object storage requires a user name and password to authenticate, use `access_key` and `secret_key` to provide the user name and password, respectively.
        #[builder(into, default)]
        pub secret_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a certificate to authenticate with an object storage system that uses a private or self-signed certificate authority (CA). You must specify a Base64-encoded .pem string. The certificate can be up to 32768 bytes (before Base64 encoding).
        #[builder(into, default)]
        pub server_certificate: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the self-managed object storage server. This value is the IP address or Domain Name Service (DNS) name of the object storage server. An agent uses this host name to mount the object storage server in a network.
        #[builder(into)]
        pub server_hostname: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The port that your self-managed object storage server accepts inbound network traffic on. The server port is set by default to TCP 80 (`HTTP`) or TCP 443 (`HTTPS`). You can specify a custom port if your self-managed object storage server requires one.
        #[builder(into, default)]
        pub server_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The protocol that the object storage server uses to communicate. Valid values are `HTTP` or `HTTPS`.
        #[builder(into, default)]
        pub server_protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
    pub struct LocationObjectStorageResult {
        /// The access key is used if credentials are required to access the self-managed object storage server. If your object storage requires a user name and password to authenticate, use `access_key` and `secret_key` to provide the user name and password, respectively.
        pub access_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of DataSync Agent ARNs with which this location will be associated.
        pub agent_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Amazon Resource Name (ARN) of the DataSync Location.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The bucket on the self-managed object storage server that is used to read data from.
        pub bucket_name: pulumi_gestalt_rust::Output<String>,
        /// The secret key is used if credentials are required to access the self-managed object storage server. If your object storage requires a user name and password to authenticate, use `access_key` and `secret_key` to provide the user name and password, respectively.
        pub secret_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies a certificate to authenticate with an object storage system that uses a private or self-signed certificate authority (CA). You must specify a Base64-encoded .pem string. The certificate can be up to 32768 bytes (before Base64 encoding).
        pub server_certificate: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the self-managed object storage server. This value is the IP address or Domain Name Service (DNS) name of the object storage server. An agent uses this host name to mount the object storage server in a network.
        pub server_hostname: pulumi_gestalt_rust::Output<String>,
        /// The port that your self-managed object storage server accepts inbound network traffic on. The server port is set by default to TCP 80 (`HTTP`) or TCP 443 (`HTTPS`). You can specify a custom port if your self-managed object storage server requires one.
        pub server_port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The protocol that the object storage server uses to communicate. Valid values are `HTTP` or `HTTPS`.
        pub server_protocol: pulumi_gestalt_rust::Output<Option<String>>,
        /// A subdirectory in the HDFS cluster. This subdirectory is used to read data from or write data to the HDFS cluster. If the subdirectory isn't specified, it will default to /.
        pub subdirectory: pulumi_gestalt_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The URL of the Object Storage location that was described.
        pub uri: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocationObjectStorageArgs,
    ) -> LocationObjectStorageResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_key_binding = args.access_key.get_output(context);
        let agent_arns_binding = args.agent_arns.get_output(context);
        let bucket_name_binding = args.bucket_name.get_output(context);
        let secret_key_binding = args.secret_key.get_output(context);
        let server_certificate_binding = args.server_certificate.get_output(context);
        let server_hostname_binding = args.server_hostname.get_output(context);
        let server_port_binding = args.server_port.get_output(context);
        let server_protocol_binding = args.server_protocol.get_output(context);
        let subdirectory_binding = args.subdirectory.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:datasync/locationObjectStorage:LocationObjectStorage".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessKey".into(),
                    value: access_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentArns".into(),
                    value: agent_arns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucketName".into(),
                    value: bucket_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretKey".into(),
                    value: secret_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverCertificate".into(),
                    value: server_certificate_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverHostname".into(),
                    value: server_hostname_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverPort".into(),
                    value: server_port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverProtocol".into(),
                    value: server_protocol_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subdirectory".into(),
                    value: subdirectory_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LocationObjectStorageResult {
            access_key: o.get_field("accessKey"),
            agent_arns: o.get_field("agentArns"),
            arn: o.get_field("arn"),
            bucket_name: o.get_field("bucketName"),
            secret_key: o.get_field("secretKey"),
            server_certificate: o.get_field("serverCertificate"),
            server_hostname: o.get_field("serverHostname"),
            server_port: o.get_field("serverPort"),
            server_protocol: o.get_field("serverProtocol"),
            subdirectory: o.get_field("subdirectory"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            uri: o.get_field("uri"),
        }
    }
}
