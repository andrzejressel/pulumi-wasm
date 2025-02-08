/// Manages an NFS Location within AWS DataSync.
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
#[allow(clippy::doc_lazy_continuation)]
pub mod nfs_location {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NfsLocationArgs {
        /// Configuration block containing mount options used by DataSync to access the NFS Server.
        #[builder(into, default)]
        pub mount_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datasync::NfsLocationMountOptions>,
        >,
        /// Configuration block containing information for connecting to the NFS File System.
        #[builder(into)]
        pub on_prem_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::datasync::NfsLocationOnPremConfig,
        >,
        /// Specifies the IP address or DNS name of the NFS server. The DataSync Agent(s) use this to mount the NFS server.
        #[builder(into)]
        pub server_hostname: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Subdirectory to perform actions as source or destination. Should be exported by the NFS server.
        #[builder(into)]
        pub subdirectory: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NfsLocationResult {
        /// Amazon Resource Name (ARN) of the DataSync Location.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block containing mount options used by DataSync to access the NFS Server.
        pub mount_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::datasync::NfsLocationMountOptions>,
        >,
        /// Configuration block containing information for connecting to the NFS File System.
        pub on_prem_config: pulumi_gestalt_rust::Output<
            super::super::types::datasync::NfsLocationOnPremConfig,
        >,
        /// Specifies the IP address or DNS name of the NFS server. The DataSync Agent(s) use this to mount the NFS server.
        pub server_hostname: pulumi_gestalt_rust::Output<String>,
        /// Subdirectory to perform actions as source or destination. Should be exported by the NFS server.
        pub subdirectory: pulumi_gestalt_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: NfsLocationArgs,
    ) -> NfsLocationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let mount_options_binding = args.mount_options.get_output(context).get_inner();
        let on_prem_config_binding = args.on_prem_config.get_output(context).get_inner();
        let server_hostname_binding = args
            .server_hostname
            .get_output(context)
            .get_inner();
        let subdirectory_binding = args.subdirectory.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datasync/nfsLocation:NfsLocation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        NfsLocationResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            mount_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mountOptions"),
            ),
            on_prem_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("onPremConfig"),
            ),
            server_hostname: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverHostname"),
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
