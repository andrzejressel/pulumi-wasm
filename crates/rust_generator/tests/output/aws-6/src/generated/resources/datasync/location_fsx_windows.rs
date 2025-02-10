/// Manages an AWS DataSync FSx Windows Location.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = location_fsx_windows::create(
///         "example",
///         LocationFsxWindowsArgs::builder()
///             .fsx_filesystem_arn("${exampleAwsFsxWindowsFileSystem.arn}")
///             .password("SuperSecretPassw0rd")
///             .security_group_arns(vec!["${exampleAwsSecurityGroup.arn}",])
///             .user("SomeUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datasync_location_fsx_windows_file_system` using the `DataSync-ARN#FSx-Windows-ARN`. For example:
///
/// ```sh
/// $ pulumi import aws:datasync/locationFsxWindows:LocationFsxWindows example arn:aws:datasync:us-west-2:123456789012:location/loc-12345678901234567#arn:aws:fsx:us-west-2:476956259333:file-system/fs-08e04cd442c1bb94a
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod location_fsx_windows {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocationFsxWindowsArgs {
        /// The name of the Windows domain that the FSx for Windows server belongs to.
        #[builder(into, default)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) for the FSx for Windows file system.
        #[builder(into)]
        pub fsx_filesystem_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The password of the user who has the permissions to access files and folders in the FSx for Windows file system.
        #[builder(into)]
        pub password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Names (ARNs) of the security groups that are to use to configure the FSx for Windows file system.
        #[builder(into)]
        pub security_group_arns: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Subdirectory to perform actions as source or destination.
        #[builder(into, default)]
        pub subdirectory: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The user who has the permissions to access files and folders in the FSx for Windows file system.
        #[builder(into)]
        pub user: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LocationFsxWindowsResult {
        /// Amazon Resource Name (ARN) of the DataSync Location.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The time that the FSx for Windows location was created.
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// The name of the Windows domain that the FSx for Windows server belongs to.
        pub domain: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) for the FSx for Windows file system.
        pub fsx_filesystem_arn: pulumi_gestalt_rust::Output<String>,
        /// The password of the user who has the permissions to access files and folders in the FSx for Windows file system.
        pub password: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Names (ARNs) of the security groups that are to use to configure the FSx for Windows file system.
        pub security_group_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Subdirectory to perform actions as source or destination.
        pub subdirectory: pulumi_gestalt_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The URL of the FSx for Windows location that was described.
        pub uri: pulumi_gestalt_rust::Output<String>,
        /// The user who has the permissions to access files and folders in the FSx for Windows file system.
        pub user: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocationFsxWindowsArgs,
    ) -> LocationFsxWindowsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_binding = args.domain.get_output(context);
        let fsx_filesystem_arn_binding = args.fsx_filesystem_arn.get_output(context);
        let password_binding = args.password.get_output(context);
        let security_group_arns_binding = args.security_group_arns.get_output(context);
        let subdirectory_binding = args.subdirectory.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_binding = args.user.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:datasync/locationFsxWindows:LocationFsxWindows".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fsxFilesystemArn".into(),
                    value: fsx_filesystem_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupArns".into(),
                    value: security_group_arns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subdirectory".into(),
                    value: subdirectory_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "user".into(),
                    value: user_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LocationFsxWindowsResult {
            arn: o.get_field("arn"),
            creation_time: o.get_field("creationTime"),
            domain: o.get_field("domain"),
            fsx_filesystem_arn: o.get_field("fsxFilesystemArn"),
            password: o.get_field("password"),
            security_group_arns: o.get_field("securityGroupArns"),
            subdirectory: o.get_field("subdirectory"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            uri: o.get_field("uri"),
            user: o.get_field("user"),
        }
    }
}
