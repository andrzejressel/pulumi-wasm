pub mod get_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageArgs {
        /// Arn of the image being searched for. Cannot be used with name_regex or name.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean that if it is set to true and there are multiple images returned the most recent will be returned. If it is set to false and there are multiple images return the datasource will error.
        #[builder(into, default)]
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the image being searched for. Cannot be used with name_regex or arn.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Regular expression name of the image being searched for. Cannot be used with arn or name.
        #[builder(into, default)]
        pub name_regex: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of image which must be (PUBLIC, PRIVATE, or SHARED).
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetImageResult {
        pub applications: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appstream::GetImageApplication>,
        >,
        /// Version of the AppStream 2.0 agent to use for instances that are launched from this image. Has a maximum length of 100 characters.
        pub appstream_agent_version: pulumi_wasm_rust::Output<String>,
        /// ARN of the image.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the image from which the image was created.
        pub base_image_arn: pulumi_wasm_rust::Output<String>,
        /// Time at which this image was created.
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// Description of image.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Image name to display.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The name of the image builder that was used to created the private image. If the image is sharedthen the value is null.
        pub image_builder_name: pulumi_wasm_rust::Output<String>,
        /// Boolean to indicate whether an image builder can be launched from this image.
        /// * `image error` - Resource error object that describes the error containing the following:
        pub image_builder_supported: pulumi_wasm_rust::Output<bool>,
        /// List of strings describing the image permissions containing the following:
        pub image_permissions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appstream::GetImageImagePermission>,
        >,
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub name_regex: pulumi_wasm_rust::Output<Option<String>>,
        /// Operating system platform of the image. Values will be from: WINDOWS | WINDOWS_SERVER_2016 | WINDOWS_SERVER_2019 | WINDOWS_SERVER_2022 | AMAZON_LINUX2
        pub platform: pulumi_wasm_rust::Output<String>,
        pub public_base_image_released_date: pulumi_wasm_rust::Output<String>,
        /// Current state of image. Image starts in PENDING state which changes to AVAILABLE if creation passes and FAILED if it fails. Values will be from: PENDING | AVAILABLE | FAILED | COPYING | DELETING | CREATING | IMPORTING.
        pub state: pulumi_wasm_rust::Output<String>,
        pub state_change_reasons: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appstream::GetImageStateChangeReason>,
        >,
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetImageArgs) -> GetImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let most_recent_binding = args.most_recent.get_inner();
        let name_binding = args.name.get_inner();
        let name_regex_binding = args.name_regex.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:appstream/getImage:getImage".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nameRegex".into(),
                    value: &name_regex_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applications".into(),
                },
                register_interface::ResultField {
                    name: "appstreamAgentVersion".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "baseImageArn".into(),
                },
                register_interface::ResultField {
                    name: "createdTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "imageBuilderName".into(),
                },
                register_interface::ResultField {
                    name: "imageBuilderSupported".into(),
                },
                register_interface::ResultField {
                    name: "imagePermissions".into(),
                },
                register_interface::ResultField {
                    name: "mostRecent".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nameRegex".into(),
                },
                register_interface::ResultField {
                    name: "platform".into(),
                },
                register_interface::ResultField {
                    name: "publicBaseImageReleasedDate".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "stateChangeReasons".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetImageResult {
            applications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applications").unwrap(),
            ),
            appstream_agent_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appstreamAgentVersion").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            base_image_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseImageArn").unwrap(),
            ),
            created_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            image_builder_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageBuilderName").unwrap(),
            ),
            image_builder_supported: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageBuilderSupported").unwrap(),
            ),
            image_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imagePermissions").unwrap(),
            ),
            most_recent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mostRecent").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_regex: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nameRegex").unwrap(),
            ),
            platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platform").unwrap(),
            ),
            public_base_image_released_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicBaseImageReleasedDate").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            state_change_reasons: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stateChangeReasons").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}