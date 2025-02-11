/// Resource for managing a AWS Shield Proactive Engagement.
/// Proactive engagement authorizes the Shield Response Team (SRT) to use email and phone to notify contacts about escalations to the SRT and to initiate proactive customer support.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:shield:ProactiveEngagement
///     properties:
///       enabled: true
///       emergencyContacts:
///         - contactNotes: Notes
///           emailAddress: contact1@example.com
///           phoneNumber: '+12358132134'
///         - contactNotes: Notes 2
///           emailAddress: contact2@example.com
///           phoneNumber: '+12358132134'
///     options:
///       dependsOn:
///         - ${exampleDrtAccessRoleArnAssociation}
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example-role
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Sid: ""
///               Effect: Allow
///               Principal:
///                 Service: drt.shield.amazonaws.com
///               Action: sts:AssumeRole
///   exampleRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: example
///     properties:
///       role: ${exampleRole.name}
///       policyArn: arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy
///   exampleDrtAccessRoleArnAssociation:
///     type: aws:shield:DrtAccessRoleArnAssociation
///     name: example
///     properties:
///       roleArn: ${exampleRole.arn}
///   exampleProtectionGroup:
///     type: aws:shield:ProtectionGroup
///     name: example
///     properties:
///       protectionGroupId: example
///       aggregation: MAX
///       pattern: ALL
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Shield proactive engagement using the AWS account ID. For example:
///
/// ```sh
/// $ pulumi import aws:shield/proactiveEngagement:ProactiveEngagement example 123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod proactive_engagement {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProactiveEngagementArgs {
        /// One or more emergency contacts. You must provide at least one phone number in the emergency contact list. See `emergency_contacts`.
        #[builder(into, default)]
        pub emergency_contacts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::shield::ProactiveEngagementEmergencyContact>>,
        >,
        /// Boolean value indicating if Proactive Engagement should be enabled or not.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
    }
    #[allow(dead_code)]
    pub struct ProactiveEngagementResult {
        /// One or more emergency contacts. You must provide at least one phone number in the emergency contact list. See `emergency_contacts`.
        pub emergency_contacts: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::shield::ProactiveEngagementEmergencyContact>>,
        >,
        /// Boolean value indicating if Proactive Engagement should be enabled or not.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProactiveEngagementArgs,
    ) -> ProactiveEngagementResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let emergency_contacts_binding = args.emergency_contacts.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:shield/proactiveEngagement:ProactiveEngagement".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emergencyContacts".into(),
                    value: &emergency_contacts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProactiveEngagementResult {
            emergency_contacts: o.get_field("emergencyContacts"),
            enabled: o.get_field("enabled"),
        }
    }
}
