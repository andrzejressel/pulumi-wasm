/// Provides a resource to manage an S3 Access Grants instance, which serves as a logical grouping for access grants.
/// You can have one S3 Access Grants instance per Region in your account.
///
/// ## Example Usage
///
/// ### Basic Usage
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
/// }
/// ```
///
/// ### AWS IAM Identity Center
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = access_grants_instance::create(
///         "example",
///         AccessGrantsInstanceArgs::builder()
///             .identity_center_arn("arn:aws:sso:::instance/ssoins-890759e9c7bfdc1d")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Access Grants instances using the `account_id`. For example:
///
/// ```sh
/// $ pulumi import aws:s3control/accessGrantsInstance:AccessGrantsInstance example 123456789012
/// ```
pub mod access_grants_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessGrantsInstanceArgs {
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the AWS IAM Identity Center instance associated with the S3 Access Grants instance.
        #[builder(into, default)]
        pub identity_center_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccessGrantsInstanceResult {
        /// Amazon Resource Name (ARN) of the S3 Access Grants instance.
        pub access_grants_instance_arn: pulumi_wasm_rust::Output<String>,
        /// Unique ID of the S3 Access Grants instance.
        pub access_grants_instance_id: pulumi_wasm_rust::Output<String>,
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The ARN of the AWS IAM Identity Center instance application; a subresource of the original Identity Center instance.
        pub identity_center_application_arn: pulumi_wasm_rust::Output<String>,
        /// The ARN of the AWS IAM Identity Center instance associated with the S3 Access Grants instance.
        pub identity_center_arn: pulumi_wasm_rust::Output<Option<String>>,
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
        args: AccessGrantsInstanceArgs,
    ) -> AccessGrantsInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let identity_center_arn_binding = args.identity_center_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3control/accessGrantsInstance:AccessGrantsInstance".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "identityCenterArn".into(),
                    value: &identity_center_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessGrantsInstanceArn".into(),
                },
                register_interface::ResultField {
                    name: "accessGrantsInstanceId".into(),
                },
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "identityCenterApplicationArn".into(),
                },
                register_interface::ResultField {
                    name: "identityCenterArn".into(),
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
        AccessGrantsInstanceResult {
            access_grants_instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessGrantsInstanceArn").unwrap(),
            ),
            access_grants_instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessGrantsInstanceId").unwrap(),
            ),
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            identity_center_application_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityCenterApplicationArn").unwrap(),
            ),
            identity_center_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityCenterArn").unwrap(),
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