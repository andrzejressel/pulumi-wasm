/// Provides a SageMaker Workforce resource.
///
/// ## Example Usage
///
/// ### Cognito Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workforce::create(
///         "example",
///         WorkforceArgs::builder()
///             .cognito_config(
///                 WorkforceCognitoConfig::builder()
///                     .clientId("${exampleUserPoolClient.id}")
///                     .userPool("${exampleUserPoolDomain.userPoolId}")
///                     .build_struct(),
///             )
///             .workforce_name("example")
///             .build_struct(),
///     );
///     let exampleUserPool = user_pool::create(
///         "exampleUserPool",
///         UserPoolArgs::builder().name("example").build_struct(),
///     );
///     let exampleUserPoolClient = user_pool_client::create(
///         "exampleUserPoolClient",
///         UserPoolClientArgs::builder()
///             .generate_secret(true)
///             .name("example")
///             .user_pool_id("${exampleUserPool.id}")
///             .build_struct(),
///     );
///     let exampleUserPoolDomain = user_pool_domain::create(
///         "exampleUserPoolDomain",
///         UserPoolDomainArgs::builder()
///             .domain("example")
///             .user_pool_id("${exampleUserPool.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Oidc Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workforce::create(
///         "example",
///         WorkforceArgs::builder()
///             .oidc_config(
///                 WorkforceOidcConfig::builder()
///                     .authorizationEndpoint("https://example.com")
///                     .clientId("example")
///                     .clientSecret("example")
///                     .issuer("https://example.com")
///                     .jwksUri("https://example.com")
///                     .logoutEndpoint("https://example.com")
///                     .tokenEndpoint("https://example.com")
///                     .userInfoEndpoint("https://example.com")
///                     .build_struct(),
///             )
///             .workforce_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Workforces using the `workforce_name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/workforce:Workforce example example
/// ```
pub mod workforce {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkforceArgs {
        /// Use this parameter to configure an Amazon Cognito private workforce. A single Cognito workforce is created using and corresponds to a single Amazon Cognito user pool. Conflicts with `oidc_config`. see Cognito Config details below.
        #[builder(into, default)]
        pub cognito_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::WorkforceCognitoConfig>,
        >,
        /// Use this parameter to configure a private workforce using your own OIDC Identity Provider. Conflicts with `cognito_config`. see OIDC Config details below.
        #[builder(into, default)]
        pub oidc_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::WorkforceOidcConfig>,
        >,
        /// A list of IP address ranges Used to create an allow list of IP addresses for a private workforce. By default, a workforce isn't restricted to specific IP addresses. see Source Ip Config details below.
        #[builder(into, default)]
        pub source_ip_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::WorkforceSourceIpConfig>,
        >,
        /// The name of the Workforce (must be unique).
        #[builder(into)]
        pub workforce_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// configure a workforce using VPC. see Workforce VPC Config details below.
        #[builder(into, default)]
        pub workforce_vpc_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::WorkforceWorkforceVpcConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkforceResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Workforce.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Use this parameter to configure an Amazon Cognito private workforce. A single Cognito workforce is created using and corresponds to a single Amazon Cognito user pool. Conflicts with `oidc_config`. see Cognito Config details below.
        pub cognito_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::WorkforceCognitoConfig>,
        >,
        /// Use this parameter to configure a private workforce using your own OIDC Identity Provider. Conflicts with `cognito_config`. see OIDC Config details below.
        pub oidc_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::WorkforceOidcConfig>,
        >,
        /// A list of IP address ranges Used to create an allow list of IP addresses for a private workforce. By default, a workforce isn't restricted to specific IP addresses. see Source Ip Config details below.
        pub source_ip_config: pulumi_gestalt_rust::Output<
            super::super::types::sagemaker::WorkforceSourceIpConfig,
        >,
        /// The subdomain for your OIDC Identity Provider.
        pub subdomain: pulumi_gestalt_rust::Output<String>,
        /// The name of the Workforce (must be unique).
        pub workforce_name: pulumi_gestalt_rust::Output<String>,
        /// configure a workforce using VPC. see Workforce VPC Config details below.
        pub workforce_vpc_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::WorkforceWorkforceVpcConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WorkforceArgs,
    ) -> WorkforceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cognito_config_binding = args.cognito_config.get_output(context).get_inner();
        let oidc_config_binding = args.oidc_config.get_output(context).get_inner();
        let source_ip_config_binding = args
            .source_ip_config
            .get_output(context)
            .get_inner();
        let workforce_name_binding = args.workforce_name.get_output(context).get_inner();
        let workforce_vpc_config_binding = args
            .workforce_vpc_config
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/workforce:Workforce".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cognitoConfig".into(),
                    value: &cognito_config_binding,
                },
                register_interface::ObjectField {
                    name: "oidcConfig".into(),
                    value: &oidc_config_binding,
                },
                register_interface::ObjectField {
                    name: "sourceIpConfig".into(),
                    value: &source_ip_config_binding,
                },
                register_interface::ObjectField {
                    name: "workforceName".into(),
                    value: &workforce_name_binding,
                },
                register_interface::ObjectField {
                    name: "workforceVpcConfig".into(),
                    value: &workforce_vpc_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkforceResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            cognito_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cognitoConfig"),
            ),
            oidc_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("oidcConfig"),
            ),
            source_ip_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceIpConfig"),
            ),
            subdomain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subdomain"),
            ),
            workforce_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workforceName"),
            ),
            workforce_vpc_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workforceVpcConfig"),
            ),
        }
    }
}
