/// Provides an Elastic File System (EFS) access point.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = access_point::create(
///         "test",
///         AccessPointArgs::builder().file_system_id("${foo.id}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the EFS access points using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:efs/accessPoint:AccessPoint test fsap-52a643fb
/// ```
pub mod access_point {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessPointArgs {
        /// ID of the file system for which the access point is intended.
        #[builder(into)]
        pub file_system_id: pulumi_wasm_rust::Output<String>,
        /// Operating system user and group applied to all file system requests made using the access point. Detailed below.
        #[builder(into, default)]
        pub posix_user: pulumi_wasm_rust::Output<
            Option<super::super::types::efs::AccessPointPosixUser>,
        >,
        /// Directory on the Amazon EFS file system that the access point provides access to. Detailed below.
        #[builder(into, default)]
        pub root_directory: pulumi_wasm_rust::Output<
            Option<super::super::types::efs::AccessPointRootDirectory>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccessPointResult {
        /// ARN of the access point.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the file system.
        pub file_system_arn: pulumi_wasm_rust::Output<String>,
        /// ID of the file system for which the access point is intended.
        pub file_system_id: pulumi_wasm_rust::Output<String>,
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Operating system user and group applied to all file system requests made using the access point. Detailed below.
        pub posix_user: pulumi_wasm_rust::Output<
            Option<super::super::types::efs::AccessPointPosixUser>,
        >,
        /// Directory on the Amazon EFS file system that the access point provides access to. Detailed below.
        pub root_directory: pulumi_wasm_rust::Output<
            super::super::types::efs::AccessPointRootDirectory,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccessPointArgs) -> AccessPointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let file_system_id_binding = args.file_system_id.get_inner();
        let posix_user_binding = args.posix_user.get_inner();
        let root_directory_binding = args.root_directory.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:efs/accessPoint:AccessPoint".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "fileSystemId".into(),
                    value: &file_system_id_binding,
                },
                register_interface::ObjectField {
                    name: "posixUser".into(),
                    value: &posix_user_binding,
                },
                register_interface::ObjectField {
                    name: "rootDirectory".into(),
                    value: &root_directory_binding,
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
                    name: "fileSystemArn".into(),
                },
                register_interface::ResultField {
                    name: "fileSystemId".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "posixUser".into(),
                },
                register_interface::ResultField {
                    name: "rootDirectory".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessPointResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            file_system_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemArn").unwrap(),
            ),
            file_system_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemId").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            posix_user: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("posixUser").unwrap(),
            ),
            root_directory: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootDirectory").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}