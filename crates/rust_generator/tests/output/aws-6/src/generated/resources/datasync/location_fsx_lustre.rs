/// Manages an AWS DataSync FSx Lustre Location.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = location_fsx_lustre::create(
///         "example",
///         LocationFsxLustreArgs::builder()
///             .fsx_filesystem_arn("${exampleAwsFsxLustreFileSystem.arn}")
///             .security_group_arns(vec!["${exampleAwsSecurityGroup.arn}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datasync_location_fsx_lustre_file_system` using the `DataSync-ARN#FSx-Lustre-ARN`. For example:
///
/// ```sh
/// $ pulumi import aws:datasync/locationFsxLustre:LocationFsxLustre example arn:aws:datasync:us-west-2:123456789012:location/loc-12345678901234567#arn:aws:fsx:us-west-2:476956259333:file-system/fs-08e04cd442c1bb94a
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod location_fsx_lustre {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocationFsxLustreArgs {
        /// The Amazon Resource Name (ARN) for the FSx for Lustre file system.
        #[builder(into)]
        pub fsx_filesystem_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Names (ARNs) of the security groups that are to use to configure the FSx for Lustre file system.
        #[builder(into)]
        pub security_group_arns: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Subdirectory to perform actions as source or destination.
        #[builder(into, default)]
        pub subdirectory: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LocationFsxLustreResult {
        /// Amazon Resource Name (ARN) of the DataSync Location.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The time that the FSx for Lustre location was created.
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the FSx for Lustre file system.
        pub fsx_filesystem_arn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Names (ARNs) of the security groups that are to use to configure the FSx for Lustre file system.
        pub security_group_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Subdirectory to perform actions as source or destination.
        pub subdirectory: pulumi_gestalt_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The URL of the FSx for Lustre location that was described.
        pub uri: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocationFsxLustreArgs,
    ) -> LocationFsxLustreResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let fsx_filesystem_arn_binding = args.fsx_filesystem_arn.get_output(context);
        let security_group_arns_binding = args.security_group_arns.get_output(context);
        let subdirectory_binding = args.subdirectory.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:datasync/locationFsxLustre:LocationFsxLustre".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fsxFilesystemArn".into(),
                    value: fsx_filesystem_arn_binding.get_id(),
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
            ],
        };
        let o = context.register_resource(request);
        LocationFsxLustreResult {
            arn: o.get_field("arn"),
            creation_time: o.get_field("creationTime"),
            fsx_filesystem_arn: o.get_field("fsxFilesystemArn"),
            security_group_arns: o.get_field("securityGroupArns"),
            subdirectory: o.get_field("subdirectory"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            uri: o.get_field("uri"),
        }
    }
}
