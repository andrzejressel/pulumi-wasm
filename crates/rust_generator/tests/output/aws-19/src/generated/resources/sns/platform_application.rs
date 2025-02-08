/// Provides an SNS platform application resource
///
/// ## Example Usage
///
/// ### Apple Push Notification Service (APNS) using certificate-based authentication
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let apnsApplication = platform_application::create(
///         "apnsApplication",
///         PlatformApplicationArgs::builder()
///             .name("apns_application")
///             .platform("APNS")
///             .platform_credential("<APNS PRIVATE KEY>")
///             .platform_principal("<APNS CERTIFICATE>")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Apple Push Notification Service (APNS) using token-based authentication
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let apnsApplication = platform_application::create(
///         "apnsApplication",
///         PlatformApplicationArgs::builder()
///             .apple_platform_bundle_id("<APPLE BUNDLE ID>")
///             .apple_platform_team_id("<APPLE TEAM ID>")
///             .name("apns_application")
///             .platform("APNS")
///             .platform_credential("<APNS SIGNING KEY>")
///             .platform_principal("<APNS SIGNING KEY ID>")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Google Cloud Messaging (GCM)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let gcmApplication = platform_application::create(
///         "gcmApplication",
///         PlatformApplicationArgs::builder()
///             .name("gcm_application")
///             .platform("GCM")
///             .platform_credential("<GCM API KEY>")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SNS platform applications using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:sns/platformApplication:PlatformApplication gcm_application arn:aws:sns:us-west-2:123456789012:app/GCM/gcm_application
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod platform_application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PlatformApplicationArgs {
        /// The bundle identifier that's assigned to your iOS app. May only include alphanumeric characters, hyphens (-), and periods (.).
        #[builder(into, default)]
        pub apple_platform_bundle_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The identifier that's assigned to your Apple developer account team. Must be 10 alphanumeric characters.
        #[builder(into, default)]
        pub apple_platform_team_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the SNS Topic triggered when a delivery to any of the platform endpoints associated with your platform application encounters a permanent failure.
        #[builder(into, default)]
        pub event_delivery_failure_topic_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ARN of the SNS Topic triggered when a new platform endpoint is added to your platform application.
        #[builder(into, default)]
        pub event_endpoint_created_topic_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ARN of the SNS Topic triggered when an existing platform endpoint is deleted from your platform application.
        #[builder(into, default)]
        pub event_endpoint_deleted_topic_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ARN of the SNS Topic triggered when an existing platform endpoint is changed from your platform application.
        #[builder(into, default)]
        pub event_endpoint_updated_topic_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The IAM role ARN permitted to receive failure feedback for this application and give SNS write access to use CloudWatch logs on your behalf.
        #[builder(into, default)]
        pub failure_feedback_role_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The friendly name for the SNS platform application
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The platform that the app is registered with. See [Platform](http://docs.aws.amazon.com/sns/latest/dg/mobile-push-send-register.html) for supported platforms.
        #[builder(into)]
        pub platform: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Application Platform credential. See [Credential](http://docs.aws.amazon.com/sns/latest/dg/mobile-push-send-register.html) for type of credential required for platform. The value of this attribute when stored into the state is only a hash of the real value, so therefore it is not practical to use this as an attribute for other resources.
        #[builder(into)]
        pub platform_credential: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Application Platform principal. See [Principal](http://docs.aws.amazon.com/sns/latest/api/API_CreatePlatformApplication.html) for type of principal required for platform. The value of this attribute when stored into the state is only a hash of the real value, so therefore it is not practical to use this as an attribute for other resources.
        #[builder(into, default)]
        pub platform_principal: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IAM role ARN permitted to receive success feedback for this application and give SNS write access to use CloudWatch logs on your behalf.
        #[builder(into, default)]
        pub success_feedback_role_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The sample rate percentage (0-100) of successfully delivered messages.
        ///
        /// The following attributes are needed only when using APNS token credentials:
        #[builder(into, default)]
        pub success_feedback_sample_rate: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct PlatformApplicationResult {
        /// The bundle identifier that's assigned to your iOS app. May only include alphanumeric characters, hyphens (-), and periods (.).
        pub apple_platform_bundle_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The identifier that's assigned to your Apple developer account team. Must be 10 alphanumeric characters.
        pub apple_platform_team_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN of the SNS platform application
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the SNS Topic triggered when a delivery to any of the platform endpoints associated with your platform application encounters a permanent failure.
        pub event_delivery_failure_topic_arn: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The ARN of the SNS Topic triggered when a new platform endpoint is added to your platform application.
        pub event_endpoint_created_topic_arn: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The ARN of the SNS Topic triggered when an existing platform endpoint is deleted from your platform application.
        pub event_endpoint_deleted_topic_arn: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The ARN of the SNS Topic triggered when an existing platform endpoint is changed from your platform application.
        pub event_endpoint_updated_topic_arn: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The IAM role ARN permitted to receive failure feedback for this application and give SNS write access to use CloudWatch logs on your behalf.
        pub failure_feedback_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The friendly name for the SNS platform application
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The platform that the app is registered with. See [Platform](http://docs.aws.amazon.com/sns/latest/dg/mobile-push-send-register.html) for supported platforms.
        pub platform: pulumi_gestalt_rust::Output<String>,
        /// Application Platform credential. See [Credential](http://docs.aws.amazon.com/sns/latest/dg/mobile-push-send-register.html) for type of credential required for platform. The value of this attribute when stored into the state is only a hash of the real value, so therefore it is not practical to use this as an attribute for other resources.
        pub platform_credential: pulumi_gestalt_rust::Output<String>,
        /// Application Platform principal. See [Principal](http://docs.aws.amazon.com/sns/latest/api/API_CreatePlatformApplication.html) for type of principal required for platform. The value of this attribute when stored into the state is only a hash of the real value, so therefore it is not practical to use this as an attribute for other resources.
        pub platform_principal: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IAM role ARN permitted to receive success feedback for this application and give SNS write access to use CloudWatch logs on your behalf.
        pub success_feedback_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The sample rate percentage (0-100) of successfully delivered messages.
        ///
        /// The following attributes are needed only when using APNS token credentials:
        pub success_feedback_sample_rate: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PlatformApplicationArgs,
    ) -> PlatformApplicationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let apple_platform_bundle_id_binding = args
            .apple_platform_bundle_id
            .get_output(context)
            .get_inner();
        let apple_platform_team_id_binding = args
            .apple_platform_team_id
            .get_output(context)
            .get_inner();
        let event_delivery_failure_topic_arn_binding = args
            .event_delivery_failure_topic_arn
            .get_output(context)
            .get_inner();
        let event_endpoint_created_topic_arn_binding = args
            .event_endpoint_created_topic_arn
            .get_output(context)
            .get_inner();
        let event_endpoint_deleted_topic_arn_binding = args
            .event_endpoint_deleted_topic_arn
            .get_output(context)
            .get_inner();
        let event_endpoint_updated_topic_arn_binding = args
            .event_endpoint_updated_topic_arn
            .get_output(context)
            .get_inner();
        let failure_feedback_role_arn_binding = args
            .failure_feedback_role_arn
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let platform_binding = args.platform.get_output(context).get_inner();
        let platform_credential_binding = args
            .platform_credential
            .get_output(context)
            .get_inner();
        let platform_principal_binding = args
            .platform_principal
            .get_output(context)
            .get_inner();
        let success_feedback_role_arn_binding = args
            .success_feedback_role_arn
            .get_output(context)
            .get_inner();
        let success_feedback_sample_rate_binding = args
            .success_feedback_sample_rate
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sns/platformApplication:PlatformApplication".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applePlatformBundleId".into(),
                    value: &apple_platform_bundle_id_binding,
                },
                register_interface::ObjectField {
                    name: "applePlatformTeamId".into(),
                    value: &apple_platform_team_id_binding,
                },
                register_interface::ObjectField {
                    name: "eventDeliveryFailureTopicArn".into(),
                    value: &event_delivery_failure_topic_arn_binding,
                },
                register_interface::ObjectField {
                    name: "eventEndpointCreatedTopicArn".into(),
                    value: &event_endpoint_created_topic_arn_binding,
                },
                register_interface::ObjectField {
                    name: "eventEndpointDeletedTopicArn".into(),
                    value: &event_endpoint_deleted_topic_arn_binding,
                },
                register_interface::ObjectField {
                    name: "eventEndpointUpdatedTopicArn".into(),
                    value: &event_endpoint_updated_topic_arn_binding,
                },
                register_interface::ObjectField {
                    name: "failureFeedbackRoleArn".into(),
                    value: &failure_feedback_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "platform".into(),
                    value: &platform_binding,
                },
                register_interface::ObjectField {
                    name: "platformCredential".into(),
                    value: &platform_credential_binding,
                },
                register_interface::ObjectField {
                    name: "platformPrincipal".into(),
                    value: &platform_principal_binding,
                },
                register_interface::ObjectField {
                    name: "successFeedbackRoleArn".into(),
                    value: &success_feedback_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "successFeedbackSampleRate".into(),
                    value: &success_feedback_sample_rate_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PlatformApplicationResult {
            apple_platform_bundle_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applePlatformBundleId"),
            ),
            apple_platform_team_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applePlatformTeamId"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            event_delivery_failure_topic_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventDeliveryFailureTopicArn"),
            ),
            event_endpoint_created_topic_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventEndpointCreatedTopicArn"),
            ),
            event_endpoint_deleted_topic_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventEndpointDeletedTopicArn"),
            ),
            event_endpoint_updated_topic_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventEndpointUpdatedTopicArn"),
            ),
            failure_feedback_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("failureFeedbackRoleArn"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            platform: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platform"),
            ),
            platform_credential: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platformCredential"),
            ),
            platform_principal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platformPrincipal"),
            ),
            success_feedback_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("successFeedbackRoleArn"),
            ),
            success_feedback_sample_rate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("successFeedbackSampleRate"),
            ),
        }
    }
}
