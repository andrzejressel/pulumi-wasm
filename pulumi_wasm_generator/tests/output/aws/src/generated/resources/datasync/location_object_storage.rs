/// Manages a Object Storage Location within AWS DataSync.
///
/// > **NOTE:** The DataSync Agents must be available before creating this resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod location_object_storage {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocationObjectStorageArgs {
        /// The access key is used if credentials are required to access the self-managed object storage server. If your object storage requires a user name and password to authenticate, use `access_key` and `secret_key` to provide the user name and password, respectively.
        #[builder(into, default)]
        pub access_key: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of DataSync Agent ARNs with which this location will be associated.
        #[builder(into)]
        pub agent_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// The bucket on the self-managed object storage server that is used to read data from.
        #[builder(into)]
        pub bucket_name: pulumi_wasm_rust::Output<String>,
        /// The secret key is used if credentials are required to access the self-managed object storage server. If your object storage requires a user name and password to authenticate, use `access_key` and `secret_key` to provide the user name and password, respectively.
        #[builder(into, default)]
        pub secret_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a certificate to authenticate with an object storage system that uses a private or self-signed certificate authority (CA). You must specify a Base64-encoded .pem string. The certificate can be up to 32768 bytes (before Base64 encoding).
        #[builder(into, default)]
        pub server_certificate: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the self-managed object storage server. This value is the IP address or Domain Name Service (DNS) name of the object storage server. An agent uses this host name to mount the object storage server in a network.
        #[builder(into)]
        pub server_hostname: pulumi_wasm_rust::Output<String>,
        /// The port that your self-managed object storage server accepts inbound network traffic on. The server port is set by default to TCP 80 (`HTTP`) or TCP 443 (`HTTPS`). You can specify a custom port if your self-managed object storage server requires one.
        #[builder(into, default)]
        pub server_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The protocol that the object storage server uses to communicate. Valid values are `HTTP` or `HTTPS`.
        #[builder(into, default)]
        pub server_protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// A subdirectory in the HDFS cluster. This subdirectory is used to read data from or write data to the HDFS cluster. If the subdirectory isn't specified, it will default to /.
        #[builder(into, default)]
        pub subdirectory: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LocationObjectStorageResult {
        /// The access key is used if credentials are required to access the self-managed object storage server. If your object storage requires a user name and password to authenticate, use `access_key` and `secret_key` to provide the user name and password, respectively.
        pub access_key: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of DataSync Agent ARNs with which this location will be associated.
        pub agent_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// Amazon Resource Name (ARN) of the DataSync Location.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The bucket on the self-managed object storage server that is used to read data from.
        pub bucket_name: pulumi_wasm_rust::Output<String>,
        /// The secret key is used if credentials are required to access the self-managed object storage server. If your object storage requires a user name and password to authenticate, use `access_key` and `secret_key` to provide the user name and password, respectively.
        pub secret_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a certificate to authenticate with an object storage system that uses a private or self-signed certificate authority (CA). You must specify a Base64-encoded .pem string. The certificate can be up to 32768 bytes (before Base64 encoding).
        pub server_certificate: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the self-managed object storage server. This value is the IP address or Domain Name Service (DNS) name of the object storage server. An agent uses this host name to mount the object storage server in a network.
        pub server_hostname: pulumi_wasm_rust::Output<String>,
        /// The port that your self-managed object storage server accepts inbound network traffic on. The server port is set by default to TCP 80 (`HTTP`) or TCP 443 (`HTTPS`). You can specify a custom port if your self-managed object storage server requires one.
        pub server_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The protocol that the object storage server uses to communicate. Valid values are `HTTP` or `HTTPS`.
        pub server_protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// A subdirectory in the HDFS cluster. This subdirectory is used to read data from or write data to the HDFS cluster. If the subdirectory isn't specified, it will default to /.
        pub subdirectory: pulumi_wasm_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The URL of the Object Storage location that was described.
        pub uri: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: LocationObjectStorageArgs,
    ) -> LocationObjectStorageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_key_binding = args.access_key.get_inner();
        let agent_arns_binding = args.agent_arns.get_inner();
        let bucket_name_binding = args.bucket_name.get_inner();
        let secret_key_binding = args.secret_key.get_inner();
        let server_certificate_binding = args.server_certificate.get_inner();
        let server_hostname_binding = args.server_hostname.get_inner();
        let server_port_binding = args.server_port.get_inner();
        let server_protocol_binding = args.server_protocol.get_inner();
        let subdirectory_binding = args.subdirectory.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datasync/locationObjectStorage:LocationObjectStorage".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessKey".into(),
                    value: &access_key_binding,
                },
                register_interface::ObjectField {
                    name: "agentArns".into(),
                    value: &agent_arns_binding,
                },
                register_interface::ObjectField {
                    name: "bucketName".into(),
                    value: &bucket_name_binding,
                },
                register_interface::ObjectField {
                    name: "secretKey".into(),
                    value: &secret_key_binding,
                },
                register_interface::ObjectField {
                    name: "serverCertificate".into(),
                    value: &server_certificate_binding,
                },
                register_interface::ObjectField {
                    name: "serverHostname".into(),
                    value: &server_hostname_binding,
                },
                register_interface::ObjectField {
                    name: "serverPort".into(),
                    value: &server_port_binding,
                },
                register_interface::ObjectField {
                    name: "serverProtocol".into(),
                    value: &server_protocol_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessKey".into(),
                },
                register_interface::ResultField {
                    name: "agentArns".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "bucketName".into(),
                },
                register_interface::ResultField {
                    name: "secretKey".into(),
                },
                register_interface::ResultField {
                    name: "serverCertificate".into(),
                },
                register_interface::ResultField {
                    name: "serverHostname".into(),
                },
                register_interface::ResultField {
                    name: "serverPort".into(),
                },
                register_interface::ResultField {
                    name: "serverProtocol".into(),
                },
                register_interface::ResultField {
                    name: "subdirectory".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "uri".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LocationObjectStorageResult {
            access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessKey").unwrap(),
            ),
            agent_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentArns").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bucket_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucketName").unwrap(),
            ),
            secret_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretKey").unwrap(),
            ),
            server_certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverCertificate").unwrap(),
            ),
            server_hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverHostname").unwrap(),
            ),
            server_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverPort").unwrap(),
            ),
            server_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverProtocol").unwrap(),
            ),
            subdirectory: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subdirectory").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            uri: pulumi_wasm_rust::__private::into_domain(hashmap.remove("uri").unwrap()),
        }
    }
}
