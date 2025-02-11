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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceArgs {
        /// Amazon Resource Name (ARN) of the resource.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Flag to enable AWS LakeFormation hybrid access permission mode.
        ///
        /// > **NOTE:** AWS does not support registering an S3 location with an IAM role and subsequently updating the S3 location registration to a service-linked role.
        #[builder(into, default)]
        pub hybrid_access_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Role that has read/write access to the resource.
        #[builder(into, default)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Designates an AWS Identity and Access Management (IAM) service-linked role by registering this role with the Data Catalog.
        #[builder(into, default)]
        pub use_service_linked_role: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub with_federation: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ResourceResult {
        /// Amazon Resource Name (ARN) of the resource.
        ///
        /// The following arguments are optional:
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Flag to enable AWS LakeFormation hybrid access permission mode.
        ///
        /// > **NOTE:** AWS does not support registering an S3 location with an IAM role and subsequently updating the S3 location registration to a service-linked role.
        pub hybrid_access_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Date and time the resource was last modified in [RFC 3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub last_modified: pulumi_gestalt_rust::Output<String>,
        /// Role that has read/write access to the resource.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Designates an AWS Identity and Access Management (IAM) service-linked role by registering this role with the Data Catalog.
        pub use_service_linked_role: pulumi_gestalt_rust::Output<Option<bool>>,
        pub with_federation: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceArgs,
    ) -> ResourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let hybrid_access_enabled_binding = args
            .hybrid_access_enabled
            .get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let use_service_linked_role_binding = args
            .use_service_linked_role
            .get_output(context);
        let with_federation_binding = args.with_federation.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lakeformation/resource:Resource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hybridAccessEnabled".into(),
                    value: &hybrid_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "useServiceLinkedRole".into(),
                    value: &use_service_linked_role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "withFederation".into(),
                    value: &with_federation_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceResult {
            arn: o.get_field("arn"),
            hybrid_access_enabled: o.get_field("hybridAccessEnabled"),
            last_modified: o.get_field("lastModified"),
            role_arn: o.get_field("roleArn"),
            use_service_linked_role: o.get_field("useServiceLinkedRole"),
            with_federation: o.get_field("withFederation"),
        }
    }
}
