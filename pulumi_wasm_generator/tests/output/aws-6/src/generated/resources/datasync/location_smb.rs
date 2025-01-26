/// Manages a SMB Location within AWS DataSync.
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
///     let example = location_smb::create(
///         "example",
///         LocationSmbArgs::builder()
///             .agent_arns(vec!["${exampleAwsDatasyncAgent.arn}",])
///             .password("ANotGreatPassword")
///             .server_hostname("smb.example.com")
///             .subdirectory("/exported/path")
///             .user("Guest")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datasync_location_smb` using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:datasync/locationSmb:LocationSmb example arn:aws:datasync:us-east-1:123456789012:location/loc-12345678901234567
/// ```
pub mod location_smb {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocationSmbArgs {
        /// A list of DataSync Agent ARNs with which this location will be associated.
        #[builder(into)]
        pub agent_arns: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The name of the Windows domain the SMB server belongs to.
        #[builder(into, default)]
        pub domain: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block containing mount options used by DataSync to access the SMB Server. Can be `AUTOMATIC`, `SMB2`, or `SMB3`.
        #[builder(into, default)]
        pub mount_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::datasync::LocationSmbMountOptions>,
        >,
        /// The password of the user who can mount the share and has file permissions in the SMB.
        #[builder(into)]
        pub password: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the IP address or DNS name of the SMB server. The DataSync Agent(s) use this to mount the SMB share.
        #[builder(into)]
        pub server_hostname: pulumi_wasm_rust::InputOrOutput<String>,
        /// Subdirectory to perform actions as source or destination. Should be exported by the NFS server.
        #[builder(into)]
        pub subdirectory: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The user who can mount the share and has file and folder permissions in the SMB share.
        #[builder(into)]
        pub user: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LocationSmbResult {
        /// A list of DataSync Agent ARNs with which this location will be associated.
        pub agent_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// Amazon Resource Name (ARN) of the DataSync Location.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the Windows domain the SMB server belongs to.
        pub domain: pulumi_wasm_rust::Output<String>,
        /// Configuration block containing mount options used by DataSync to access the SMB Server. Can be `AUTOMATIC`, `SMB2`, or `SMB3`.
        pub mount_options: pulumi_wasm_rust::Output<
            Option<super::super::types::datasync::LocationSmbMountOptions>,
        >,
        /// The password of the user who can mount the share and has file permissions in the SMB.
        pub password: pulumi_wasm_rust::Output<String>,
        /// Specifies the IP address or DNS name of the SMB server. The DataSync Agent(s) use this to mount the SMB share.
        pub server_hostname: pulumi_wasm_rust::Output<String>,
        /// Subdirectory to perform actions as source or destination. Should be exported by the NFS server.
        pub subdirectory: pulumi_wasm_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub uri: pulumi_wasm_rust::Output<String>,
        /// The user who can mount the share and has file and folder permissions in the SMB share.
        pub user: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LocationSmbArgs,
    ) -> LocationSmbResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let agent_arns_binding = args.agent_arns.get_output(context).get_inner();
        let domain_binding = args.domain.get_output(context).get_inner();
        let mount_options_binding = args.mount_options.get_output(context).get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let server_hostname_binding = args
            .server_hostname
            .get_output(context)
            .get_inner();
        let subdirectory_binding = args.subdirectory.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let user_binding = args.user.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datasync/locationSmb:LocationSmb".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "agentArns".into(),
                    value: &agent_arns_binding,
                },
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "mountOptions".into(),
                    value: &mount_options_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
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
                register_interface::ObjectField {
                    name: "user".into(),
                    value: &user_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LocationSmbResult {
            agent_arns: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("agentArns"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            domain: pulumi_wasm_rust::__private::into_domain(o.extract_field("domain")),
            mount_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mountOptions"),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            server_hostname: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serverHostname"),
            ),
            subdirectory: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subdirectory"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            uri: pulumi_wasm_rust::__private::into_domain(o.extract_field("uri")),
            user: pulumi_wasm_rust::__private::into_domain(o.extract_field("user")),
        }
    }
}
