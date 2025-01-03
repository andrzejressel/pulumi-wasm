/// Provides a resource to manage an S3 Access Grants location.
/// A location is an S3 resource (bucket or prefix) in a permission grant that the grantee can access.
/// The S3 data must be in the same Region as your S3 Access Grants instance.
/// When you register a location, you must include the IAM role that has permission to manage the S3 location that you are registering.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = access_grants_instance::create(
///         "example",
///         AccessGrantsInstanceArgs::builder().build_struct(),
///     );
///     let exampleAccessGrantsLocation = access_grants_location::create(
///         "exampleAccessGrantsLocation",
///         AccessGrantsLocationArgs::builder()
///             .iam_role_arn("${exampleAwsIamRole.arn}")
///             .location_scope("s3://")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Access Grants locations using the `account_id` and `access_grants_location_id`, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/accessGrantsLocation:AccessGrantsLocation example 123456789012,default
/// ```
pub mod access_grants_location {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessGrantsLocationArgs {
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the IAM role that S3 Access Grants should use when fulfilling runtime access
        /// requests to the location.
        #[builder(into)]
        pub iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// The default S3 URI `s3://` or the URI to a custom location, a specific bucket or prefix.
        #[builder(into)]
        pub location_scope: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccessGrantsLocationResult {
        /// Amazon Resource Name (ARN) of the S3 Access Grants location.
        pub access_grants_location_arn: pulumi_wasm_rust::Output<String>,
        /// Unique ID of the S3 Access Grants location.
        pub access_grants_location_id: pulumi_wasm_rust::Output<String>,
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The ARN of the IAM role that S3 Access Grants should use when fulfilling runtime access
        /// requests to the location.
        pub iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// The default S3 URI `s3://` or the URI to a custom location, a specific bucket or prefix.
        pub location_scope: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AccessGrantsLocationArgs,
    ) -> AccessGrantsLocationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let iam_role_arn_binding = args.iam_role_arn.get_inner();
        let location_scope_binding = args.location_scope.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3control/accessGrantsLocation:AccessGrantsLocation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "iamRoleArn".into(),
                    value: &iam_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "locationScope".into(),
                    value: &location_scope_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessGrantsLocationArn".into(),
                },
                register_interface::ResultField {
                    name: "accessGrantsLocationId".into(),
                },
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "iamRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "locationScope".into(),
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
        AccessGrantsLocationResult {
            access_grants_location_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessGrantsLocationArn").unwrap(),
            ),
            access_grants_location_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessGrantsLocationId").unwrap(),
            ),
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamRoleArn").unwrap(),
            ),
            location_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locationScope").unwrap(),
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
