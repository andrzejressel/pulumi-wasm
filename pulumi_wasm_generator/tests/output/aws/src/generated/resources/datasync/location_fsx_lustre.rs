/// Manages an AWS DataSync FSx Lustre Location.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod location_fsx_lustre {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocationFsxLustreArgs {
        /// The Amazon Resource Name (ARN) for the FSx for Lustre file system.
        #[builder(into)]
        pub fsx_filesystem_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Names (ARNs) of the security groups that are to use to configure the FSx for Lustre file system.
        #[builder(into)]
        pub security_group_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// Subdirectory to perform actions as source or destination.
        #[builder(into, default)]
        pub subdirectory: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LocationFsxLustreResult {
        /// Amazon Resource Name (ARN) of the DataSync Location.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The time that the FSx for Lustre location was created.
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the FSx for Lustre file system.
        pub fsx_filesystem_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Names (ARNs) of the security groups that are to use to configure the FSx for Lustre file system.
        pub security_group_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// Subdirectory to perform actions as source or destination.
        pub subdirectory: pulumi_wasm_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The URL of the FSx for Lustre location that was described.
        pub uri: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LocationFsxLustreArgs) -> LocationFsxLustreResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let fsx_filesystem_arn_binding = args.fsx_filesystem_arn.get_inner();
        let security_group_arns_binding = args.security_group_arns.get_inner();
        let subdirectory_binding = args.subdirectory.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datasync/locationFsxLustre:LocationFsxLustre".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "fsxFilesystemArn".into(),
                    value: &fsx_filesystem_arn_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupArns".into(),
                    value: &security_group_arns_binding,
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
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "fsxFilesystemArn".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupArns".into(),
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
        LocationFsxLustreResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            fsx_filesystem_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fsxFilesystemArn").unwrap(),
            ),
            security_group_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupArns").unwrap(),
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