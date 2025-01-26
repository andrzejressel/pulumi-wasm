/// Registers a Lake Formation resource (e.g., S3 bucket) as managed by the Data Catalog. In other words, the S3 path is added to the data lake.
///
/// Choose a role that has read/write access to the chosen Amazon S3 path or use the service-linked role.
/// When you register the S3 path, the service-linked role and a new inline policy are created on your behalf.
/// Lake Formation adds the first path to the inline policy and attaches it to the service-linked role.
/// When you register subsequent paths, Lake Formation adds the path to the existing policy.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResource:
///     type: aws:lakeformation:Resource
///     name: example
///     properties:
///       arn: ${example.arn}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:s3:getBucket
///       arguments:
///         bucket: an-example-bucket
/// ```
pub mod resource {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceArgs {
        /// Amazon Resource Name (ARN) of the resource.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Flag to enable AWS LakeFormation hybrid access permission mode.
        ///
        /// > **NOTE:** AWS does not support registering an S3 location with an IAM role and subsequently updating the S3 location registration to a service-linked role.
        #[builder(into, default)]
        pub hybrid_access_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Role that has read/write access to the resource.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Designates an AWS Identity and Access Management (IAM) service-linked role by registering this role with the Data Catalog.
        #[builder(into, default)]
        pub use_service_linked_role: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub with_federation: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ResourceResult {
        /// Amazon Resource Name (ARN) of the resource.
        ///
        /// The following arguments are optional:
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Flag to enable AWS LakeFormation hybrid access permission mode.
        ///
        /// > **NOTE:** AWS does not support registering an S3 location with an IAM role and subsequently updating the S3 location registration to a service-linked role.
        pub hybrid_access_enabled: pulumi_wasm_rust::Output<bool>,
        /// Date and time the resource was last modified in [RFC 3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub last_modified: pulumi_wasm_rust::Output<String>,
        /// Role that has read/write access to the resource.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Designates an AWS Identity and Access Management (IAM) service-linked role by registering this role with the Data Catalog.
        pub use_service_linked_role: pulumi_wasm_rust::Output<Option<bool>>,
        pub with_federation: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ResourceArgs,
    ) -> ResourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let hybrid_access_enabled_binding = args
            .hybrid_access_enabled
            .get_output(context)
            .get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let use_service_linked_role_binding = args
            .use_service_linked_role
            .get_output(context)
            .get_inner();
        let with_federation_binding = args
            .with_federation
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lakeformation/resource:Resource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "hybridAccessEnabled".into(),
                    value: &hybrid_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "useServiceLinkedRole".into(),
                    value: &use_service_linked_role_binding,
                },
                register_interface::ObjectField {
                    name: "withFederation".into(),
                    value: &with_federation_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "hybridAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "lastModified".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "useServiceLinkedRole".into(),
                },
                register_interface::ResultField {
                    name: "withFederation".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            hybrid_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hybridAccessEnabled").unwrap(),
            ),
            last_modified: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModified").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            use_service_linked_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("useServiceLinkedRole").unwrap(),
            ),
            with_federation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("withFederation").unwrap(),
            ),
        }
    }
}
