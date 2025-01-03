/// Manages an NFS Location within AWS DataSync.
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
///     let example = nfs_location::create(
///         "example",
///         NfsLocationArgs::builder()
///             .on_prem_config(
///                 NfsLocationOnPremConfig::builder()
///                     .agentArns(vec!["${exampleAwsDatasyncAgent.arn}",])
///                     .build_struct(),
///             )
///             .server_hostname("nfs.example.com")
///             .subdirectory("/exported/path")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datasync_location_nfs` using the DataSync Task Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:datasync/nfsLocation:NfsLocation example arn:aws:datasync:us-east-1:123456789012:location/loc-12345678901234567
/// ```
pub mod nfs_location {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NfsLocationArgs {
        /// Configuration block containing mount options used by DataSync to access the NFS Server.
        #[builder(into, default)]
        pub mount_options: pulumi_wasm_rust::Output<
            Option<super::super::types::datasync::NfsLocationMountOptions>,
        >,
        /// Configuration block containing information for connecting to the NFS File System.
        #[builder(into)]
        pub on_prem_config: pulumi_wasm_rust::Output<
            super::super::types::datasync::NfsLocationOnPremConfig,
        >,
        /// Specifies the IP address or DNS name of the NFS server. The DataSync Agent(s) use this to mount the NFS server.
        #[builder(into)]
        pub server_hostname: pulumi_wasm_rust::Output<String>,
        /// Subdirectory to perform actions as source or destination. Should be exported by the NFS server.
        #[builder(into)]
        pub subdirectory: pulumi_wasm_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NfsLocationResult {
        /// Amazon Resource Name (ARN) of the DataSync Location.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block containing mount options used by DataSync to access the NFS Server.
        pub mount_options: pulumi_wasm_rust::Output<
            Option<super::super::types::datasync::NfsLocationMountOptions>,
        >,
        /// Configuration block containing information for connecting to the NFS File System.
        pub on_prem_config: pulumi_wasm_rust::Output<
            super::super::types::datasync::NfsLocationOnPremConfig,
        >,
        /// Specifies the IP address or DNS name of the NFS server. The DataSync Agent(s) use this to mount the NFS server.
        pub server_hostname: pulumi_wasm_rust::Output<String>,
        /// Subdirectory to perform actions as source or destination. Should be exported by the NFS server.
        pub subdirectory: pulumi_wasm_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub uri: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NfsLocationArgs) -> NfsLocationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let mount_options_binding = args.mount_options.get_inner();
        let on_prem_config_binding = args.on_prem_config.get_inner();
        let server_hostname_binding = args.server_hostname.get_inner();
        let subdirectory_binding = args.subdirectory.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datasync/nfsLocation:NfsLocation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mountOptions".into(),
                    value: &mount_options_binding,
                },
                register_interface::ObjectField {
                    name: "onPremConfig".into(),
                    value: &on_prem_config_binding,
                },
                register_interface::ObjectField {
                    name: "serverHostname".into(),
                    value: &server_hostname_binding,
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
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "mountOptions".into(),
                },
                register_interface::ResultField {
                    name: "onPremConfig".into(),
                },
                register_interface::ResultField {
                    name: "serverHostname".into(),
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
        NfsLocationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            mount_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mountOptions").unwrap(),
            ),
            on_prem_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("onPremConfig").unwrap(),
            ),
            server_hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverHostname").unwrap(),
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
