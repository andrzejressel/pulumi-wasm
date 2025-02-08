/// Provides a Cognito Risk Configuration resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = risk_configuration::create(
///         "example",
///         RiskConfigurationArgs::builder()
///             .risk_exception_configuration(
///                 RiskConfigurationRiskExceptionConfiguration::builder()
///                     .blockedIpRangeLists(vec!["10.10.10.10/32",])
///                     .build_struct(),
///             )
///             .user_pool_id("${exampleAwsCognitoUserPool.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Import using the user pool ID and Client ID separated by a `:`:
///
/// __Using `pulumi import` to import__ Cognito Risk Configurations using the user pool ID or the user pool ID and Client Id separated by a `:`. For example:
///
/// Import using the user pool ID:
///
/// ```sh
/// $ pulumi import aws:cognito/riskConfiguration:RiskConfiguration main example
/// ```
/// Import using the user pool ID and Client ID separated by a `:`:
///
/// ```sh
/// $ pulumi import aws:cognito/riskConfiguration:RiskConfiguration main example:example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod risk_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RiskConfigurationArgs {
        /// The account takeover risk configuration. See details below.
        #[builder(into, default)]
        pub account_takeover_risk_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfiguration,
            >,
        >,
        /// The app client ID. When the client ID is not provided, the same risk configuration is applied to all the clients in the User Pool.
        #[builder(into, default)]
        pub client_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The compromised credentials risk configuration. See details below.
        #[builder(into, default)]
        pub compromised_credentials_risk_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::cognito::RiskConfigurationCompromisedCredentialsRiskConfiguration,
            >,
        >,
        /// The configuration to override the risk decision. See details below.
        #[builder(into, default)]
        pub risk_exception_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::cognito::RiskConfigurationRiskExceptionConfiguration,
            >,
        >,
        /// The user pool ID.
        #[builder(into)]
        pub user_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RiskConfigurationResult {
        /// The account takeover risk configuration. See details below.
        pub account_takeover_risk_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfiguration,
            >,
        >,
        /// The app client ID. When the client ID is not provided, the same risk configuration is applied to all the clients in the User Pool.
        pub client_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The compromised credentials risk configuration. See details below.
        pub compromised_credentials_risk_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::cognito::RiskConfigurationCompromisedCredentialsRiskConfiguration,
            >,
        >,
        /// The configuration to override the risk decision. See details below.
        pub risk_exception_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::cognito::RiskConfigurationRiskExceptionConfiguration,
            >,
        >,
        /// The user pool ID.
        pub user_pool_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RiskConfigurationArgs,
    ) -> RiskConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_takeover_risk_configuration_binding = args
            .account_takeover_risk_configuration
            .get_output(context)
            .get_inner();
        let client_id_binding = args.client_id.get_output(context).get_inner();
        let compromised_credentials_risk_configuration_binding = args
            .compromised_credentials_risk_configuration
            .get_output(context)
            .get_inner();
        let risk_exception_configuration_binding = args
            .risk_exception_configuration
            .get_output(context)
            .get_inner();
        let user_pool_id_binding = args.user_pool_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cognito/riskConfiguration:RiskConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountTakeoverRiskConfiguration".into(),
                    value: &account_takeover_risk_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "clientId".into(),
                    value: &client_id_binding,
                },
                register_interface::ObjectField {
                    name: "compromisedCredentialsRiskConfiguration".into(),
                    value: &compromised_credentials_risk_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "riskExceptionConfiguration".into(),
                    value: &risk_exception_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RiskConfigurationResult {
            account_takeover_risk_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountTakeoverRiskConfiguration"),
            ),
            client_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientId"),
            ),
            compromised_credentials_risk_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("compromisedCredentialsRiskConfiguration"),
            ),
            risk_exception_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("riskExceptionConfiguration"),
            ),
            user_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userPoolId"),
            ),
        }
    }
}
