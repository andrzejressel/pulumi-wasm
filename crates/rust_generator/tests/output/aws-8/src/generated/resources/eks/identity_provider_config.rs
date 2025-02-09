/// Manages an EKS Identity Provider Configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = identity_provider_config::create(
///         "example",
///         IdentityProviderConfigArgs::builder()
///             .cluster_name("${exampleAwsEksCluster.name}")
///             .oidc(
///                 IdentityProviderConfigOidc::builder()
///                     .clientId("your client_id")
///                     .identityProviderConfigName("example")
///                     .issuerUrl("your issuer_url")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EKS Identity Provider Configurations using the `cluster_name` and `identity_provider_config_name` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:eks/identityProviderConfig:IdentityProviderConfig my_identity_provider_config my_cluster:my_identity_provider_config
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod identity_provider_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityProviderConfigArgs {
        /// Name of the EKS Cluster.
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Nested attribute containing [OpenID Connect](https://openid.net/connect/) identity provider information for the cluster. Detailed below.
        #[builder(into)]
        pub oidc: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::eks::IdentityProviderConfigOidc,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IdentityProviderConfigResult {
        /// Amazon Resource Name (ARN) of the EKS Identity Provider Configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the EKS Cluster.
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// Nested attribute containing [OpenID Connect](https://openid.net/connect/) identity provider information for the cluster. Detailed below.
        pub oidc: pulumi_gestalt_rust::Output<
            super::super::types::eks::IdentityProviderConfigOidc,
        >,
        /// Status of the EKS Identity Provider Configuration.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: IdentityProviderConfigArgs,
    ) -> IdentityProviderConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding_1 = args.cluster_name.get_output(context);
        let cluster_name_binding = cluster_name_binding_1.get_inner();
        let oidc_binding_1 = args.oidc.get_output(context);
        let oidc_binding = oidc_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:eks/identityProviderConfig:IdentityProviderConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "oidc".into(),
                    value: &oidc_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IdentityProviderConfigResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            cluster_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            oidc: pulumi_gestalt_rust::__private::into_domain(o.extract_field("oidc")),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
