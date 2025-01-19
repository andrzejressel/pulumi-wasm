/// Provides a SageMaker Space resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = space::create(
///         "example",
///         SpaceArgs::builder().domain_id("${test.id}").space_name("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Spaces using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/space:Space test_space arn:aws:sagemaker:us-west-2:123456789012:space/domain-id/space-name
/// ```
pub mod space {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpaceArgs {
        /// The ID of the associated Domain.
        #[builder(into)]
        pub domain_id: pulumi_wasm_rust::Output<String>,
        /// A collection of ownership settings. Required if `space_sharing_settings` is set. See `ownership_settings` Block below.
        #[builder(into, default)]
        pub ownership_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::SpaceOwnershipSettings>,
        >,
        /// The name of the space that appears in the SageMaker Studio UI.
        #[builder(into, default)]
        pub space_display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the space.
        #[builder(into)]
        pub space_name: pulumi_wasm_rust::Output<String>,
        /// A collection of space settings. See `space_settings` Block below.
        #[builder(into, default)]
        pub space_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::SpaceSpaceSettings>,
        >,
        /// A collection of space sharing settings. Required if `ownership_settings` is set. See `space_sharing_settings` Block below.
        #[builder(into, default)]
        pub space_sharing_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::SpaceSpaceSharingSettings>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SpaceResult {
        /// The space's Amazon Resource Name (ARN).
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the associated Domain.
        pub domain_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the space's profile in the Amazon Elastic File System volume.
        pub home_efs_file_system_uid: pulumi_wasm_rust::Output<String>,
        /// A collection of ownership settings. Required if `space_sharing_settings` is set. See `ownership_settings` Block below.
        pub ownership_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::SpaceOwnershipSettings>,
        >,
        /// The name of the space that appears in the SageMaker Studio UI.
        pub space_display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the space.
        pub space_name: pulumi_wasm_rust::Output<String>,
        /// A collection of space settings. See `space_settings` Block below.
        pub space_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::SpaceSpaceSettings>,
        >,
        /// A collection of space sharing settings. Required if `ownership_settings` is set. See `space_sharing_settings` Block below.
        pub space_sharing_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::SpaceSpaceSharingSettings>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Returns the URL of the space. If the space is created with Amazon Web Services IAM Identity Center (Successor to Amazon Web Services Single Sign-On) authentication, users can navigate to the URL after appending the respective redirect parameter for the application type to be federated through Amazon Web Services IAM Identity Center.
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SpaceArgs) -> SpaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_id_binding = args.domain_id.get_inner();
        let ownership_settings_binding = args.ownership_settings.get_inner();
        let space_display_name_binding = args.space_display_name.get_inner();
        let space_name_binding = args.space_name.get_inner();
        let space_settings_binding = args.space_settings.get_inner();
        let space_sharing_settings_binding = args.space_sharing_settings.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/space:Space".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainId".into(),
                    value: &domain_id_binding,
                },
                register_interface::ObjectField {
                    name: "ownershipSettings".into(),
                    value: &ownership_settings_binding,
                },
                register_interface::ObjectField {
                    name: "spaceDisplayName".into(),
                    value: &space_display_name_binding,
                },
                register_interface::ObjectField {
                    name: "spaceName".into(),
                    value: &space_name_binding,
                },
                register_interface::ObjectField {
                    name: "spaceSettings".into(),
                    value: &space_settings_binding,
                },
                register_interface::ObjectField {
                    name: "spaceSharingSettings".into(),
                    value: &space_sharing_settings_binding,
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
                    name: "domainId".into(),
                },
                register_interface::ResultField {
                    name: "homeEfsFileSystemUid".into(),
                },
                register_interface::ResultField {
                    name: "ownershipSettings".into(),
                },
                register_interface::ResultField {
                    name: "spaceDisplayName".into(),
                },
                register_interface::ResultField {
                    name: "spaceName".into(),
                },
                register_interface::ResultField {
                    name: "spaceSettings".into(),
                },
                register_interface::ResultField {
                    name: "spaceSharingSettings".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpaceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            domain_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainId").unwrap(),
            ),
            home_efs_file_system_uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("homeEfsFileSystemUid").unwrap(),
            ),
            ownership_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownershipSettings").unwrap(),
            ),
            space_display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spaceDisplayName").unwrap(),
            ),
            space_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spaceName").unwrap(),
            ),
            space_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spaceSettings").unwrap(),
            ),
            space_sharing_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spaceSharingSettings").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
