pub mod get_access_point {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccessPointArgs {
        /// ID that identifies the file system.
        #[builder(into)]
        pub access_point_id: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetAccessPointResult {
        pub access_point_id: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name of the file system.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name of the file system.
        pub file_system_arn: pulumi_wasm_rust::Output<String>,
        /// ID of the file system for which the access point is intended.
        pub file_system_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Single element list containing operating system user and group applied to all file system requests made using the access point.
        pub posix_users: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::efs::GetAccessPointPosixUser>,
        >,
        /// Single element list containing information on the directory on the Amazon EFS file system that the access point provides access to.
        pub root_directories: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::efs::GetAccessPointRootDirectory>,
        >,
        /// Key-value mapping of resource tags.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAccessPointArgs) -> GetAccessPointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_point_id_binding = args.access_point_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:efs/getAccessPoint:getAccessPoint".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessPointId".into(),
                    value: &access_point_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessPointId".into(),
                },
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "posixUsers".into(),
                },
                register_interface::ResultField {
                    name: "rootDirectories".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAccessPointResult {
            access_point_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessPointId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            file_system_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemArn").unwrap(),
            ),
            file_system_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            posix_users: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("posixUsers").unwrap(),
            ),
            root_directories: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootDirectories").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
