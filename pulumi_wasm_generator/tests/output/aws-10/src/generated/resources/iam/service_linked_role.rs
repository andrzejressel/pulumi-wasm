/// Provides an [IAM service-linked role](https://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let elasticbeanstalk = service_linked_role::create(
///         "elasticbeanstalk",
///         ServiceLinkedRoleArgs::builder()
///             .aws_service_name("elasticbeanstalk.amazonaws.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM service-linked roles using role ARN. For example:
///
/// ```sh
/// $ pulumi import aws:iam/serviceLinkedRole:ServiceLinkedRole elasticbeanstalk arn:aws:iam::123456789012:role/aws-service-role/elasticbeanstalk.amazonaws.com/AWSServiceRoleForElasticBeanstalk
/// ```
pub mod service_linked_role {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceLinkedRoleArgs {
        /// The AWS service to which this role is attached. You use a string similar to a URL but without the `http://` in front. For example: `elasticbeanstalk.amazonaws.com`. To find the full list of services that support service-linked roles, check [the docs](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-services-that-work-with-iam.html).
        #[builder(into)]
        pub aws_service_name: pulumi_wasm_rust::Output<String>,
        /// Additional string appended to the role name. Not all AWS services support custom suffixes.
        #[builder(into, default)]
        pub custom_suffix: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of the role.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of tags for the IAM role. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceLinkedRoleResult {
        /// The Amazon Resource Name (ARN) specifying the role.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The AWS service to which this role is attached. You use a string similar to a URL but without the `http://` in front. For example: `elasticbeanstalk.amazonaws.com`. To find the full list of services that support service-linked roles, check [the docs](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-services-that-work-with-iam.html).
        pub aws_service_name: pulumi_wasm_rust::Output<String>,
        /// The creation date of the IAM role.
        pub create_date: pulumi_wasm_rust::Output<String>,
        /// Additional string appended to the role name. Not all AWS services support custom suffixes.
        pub custom_suffix: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of the role.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the role.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The path of the role.
        pub path: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of tags for the IAM role. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The stable and unique string identifying the role.
        pub unique_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceLinkedRoleArgs) -> ServiceLinkedRoleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_service_name_binding = args.aws_service_name.get_inner();
        let custom_suffix_binding = args.custom_suffix.get_inner();
        let description_binding = args.description.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/serviceLinkedRole:ServiceLinkedRole".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsServiceName".into(),
                    value: &aws_service_name_binding,
                },
                register_interface::ObjectField {
                    name: "customSuffix".into(),
                    value: &custom_suffix_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "awsServiceName".into(),
                },
                register_interface::ResultField {
                    name: "createDate".into(),
                },
                register_interface::ResultField {
                    name: "customSuffix".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "path".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "uniqueId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceLinkedRoleResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsServiceName").unwrap(),
            ),
            create_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createDate").unwrap(),
            ),
            custom_suffix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customSuffix").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("path").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            unique_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uniqueId").unwrap(),
            ),
        }
    }
}
