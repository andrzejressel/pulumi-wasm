#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageArgs {
        /// Arn of the image being searched for. Cannot be used with name_regex or name.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean that if it is set to true and there are multiple images returned the most recent will be returned. If it is set to false and there are multiple images return the datasource will error.
        #[builder(into, default)]
        pub most_recent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the image being searched for. Cannot be used with name_regex or arn.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Regular expression name of the image being searched for. Cannot be used with arn or name.
        #[builder(into, default)]
        pub name_regex: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of image which must be (PUBLIC, PRIVATE, or SHARED).
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetImageResult {
        pub applications: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appstream::GetImageApplication>,
        >,
        /// Version of the AppStream 2.0 agent to use for instances that are launched from this image. Has a maximum length of 100 characters.
        pub appstream_agent_version: pulumi_gestalt_rust::Output<String>,
        /// ARN of the image.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the image from which the image was created.
        pub base_image_arn: pulumi_gestalt_rust::Output<String>,
        /// Time at which this image was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// Description of image.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Image name to display.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name of the image builder that was used to created the private image. If the image is sharedthen the value is null.
        pub image_builder_name: pulumi_gestalt_rust::Output<String>,
        /// Boolean to indicate whether an image builder can be launched from this image.
        /// * `image error` - Resource error object that describes the error containing the following:
        pub image_builder_supported: pulumi_gestalt_rust::Output<bool>,
        /// List of strings describing the image permissions containing the following:
        pub image_permissions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appstream::GetImageImagePermission>,
        >,
        pub most_recent: pulumi_gestalt_rust::Output<Option<bool>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub name_regex: pulumi_gestalt_rust::Output<Option<String>>,
        /// Operating system platform of the image. Values will be from: WINDOWS | WINDOWS_SERVER_2016 | WINDOWS_SERVER_2019 | WINDOWS_SERVER_2022 | AMAZON_LINUX2
        pub platform: pulumi_gestalt_rust::Output<String>,
        pub public_base_image_released_date: pulumi_gestalt_rust::Output<String>,
        /// Current state of image. Image starts in PENDING state which changes to AVAILABLE if creation passes and FAILED if it fails. Values will be from: PENDING | AVAILABLE | FAILED | COPYING | DELETING | CREATING | IMPORTING.
        pub state: pulumi_gestalt_rust::Output<String>,
        pub state_change_reasons: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appstream::GetImageStateChangeReason>,
        >,
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetImageArgs,
    ) -> GetImageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let most_recent_binding = args.most_recent.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_regex_binding = args.name_regex.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:appstream/getImage:getImage".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nameRegex".into(),
                    value: &name_regex_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetImageResult {
            applications: o.get_field("applications"),
            appstream_agent_version: o.get_field("appstreamAgentVersion"),
            arn: o.get_field("arn"),
            base_image_arn: o.get_field("baseImageArn"),
            created_time: o.get_field("createdTime"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            image_builder_name: o.get_field("imageBuilderName"),
            image_builder_supported: o.get_field("imageBuilderSupported"),
            image_permissions: o.get_field("imagePermissions"),
            most_recent: o.get_field("mostRecent"),
            name: o.get_field("name"),
            name_regex: o.get_field("nameRegex"),
            platform: o.get_field("platform"),
            public_base_image_released_date: o.get_field("publicBaseImageReleasedDate"),
            state: o.get_field("state"),
            state_change_reasons: o.get_field("stateChangeReasons"),
            type_: o.get_field("type"),
        }
    }
}
