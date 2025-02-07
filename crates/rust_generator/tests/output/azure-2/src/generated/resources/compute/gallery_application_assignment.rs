/// Manages a Virtual Machine Gallery Application Assignment.
///
/// > **Note:** Gallery Application Assignments can be defined either directly on `azure.compute.LinuxVirtualMachine` and `azure.compute.WindowsVirtualMachine` resources, or using the `azure.compute.GalleryApplicationAssignment` resource - but the two approaches cannot be used together. If both are used with the same Virtual Machine, spurious changes will occur. It's recommended to use `ignore_changes` for the `gallery_application` block on the associated virtual machine resources, to avoid a persistent diff when using this resource.
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleSharedImageGallery:
///     type: azure:compute:SharedImageGallery
///     name: example
///     properties:
///       name: examplegallery
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///   exampleGalleryApplication:
///     type: azure:compute:GalleryApplication
///     name: example
///     properties:
///       name: example-app
///       galleryId: ${exampleSharedImageGallery.id}
///       location: ${exampleResourceGroup.location}
///       supportedOsType: Linux
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestorage
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: example-container
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: blob
///   exampleBlob:
///     type: azure:storage:Blob
///     name: example
///     properties:
///       name: scripts
///       storageAccountName: ${exampleAccount.name}
///       storageContainerName: ${exampleContainer.name}
///       type: Block
///       sourceContent: '[scripts file content]'
///   exampleGalleryApplicationVersion:
///     type: azure:compute:GalleryApplicationVersion
///     name: example
///     properties:
///       name: 0.0.1
///       galleryApplicationId: ${exampleGalleryApplication.id}
///       location: ${exampleGalleryApplication.location}
///       manageAction:
///         install: '[install command]'
///         remove: '[remove command]'
///       source:
///         mediaLink: ${exampleBlob.id}
///       targetRegions:
///         - name: ${exampleGalleryApplication.location}
///           regionalReplicaCount: 1
///   exampleGalleryApplicationAssignment:
///     type: azure:compute:GalleryApplicationAssignment
///     name: example
///     properties:
///       galleryApplicationVersionId: ${exampleGalleryApplicationVersion.id}
///       virtualMachineId: ${example.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:compute:getVirtualMachine
///       arguments:
///         name: example-vm
///         resourceGroupName: example-resources-vm
/// ```
///
/// ## Import
///
/// Virtual Machine Gallery Application Assignments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/galleryApplicationAssignment:GalleryApplicationAssignment example subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/virtualMachines/machine1|/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Compute/galleries/gallery1/applications/galleryApplication1/versions/galleryApplicationVersion1
/// ```
///
pub mod gallery_application_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GalleryApplicationAssignmentArgs {
        /// Specifies the URI to an Azure Blob that will replace the default configuration for the package if provided. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub configuration_blob_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Gallery Application Version. Changing this forces a new resource to be created.
        #[builder(into)]
        pub gallery_application_version_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the order in which the packages have to be installed. Possible values are between `0` and `2147483647`. Defaults to `0`.
        #[builder(into, default)]
        pub order: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies a passthrough value for more generic context. This field can be any valid `string` value. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub tag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_machine_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GalleryApplicationAssignmentResult {
        /// Specifies the URI to an Azure Blob that will replace the default configuration for the package if provided. Changing this forces a new resource to be created.
        pub configuration_blob_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Gallery Application Version. Changing this forces a new resource to be created.
        pub gallery_application_version_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the order in which the packages have to be installed. Possible values are between `0` and `2147483647`. Defaults to `0`.
        pub order: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies a passthrough value for more generic context. This field can be any valid `string` value. Changing this forces a new resource to be created.
        pub tag: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Virtual Machine. Changing this forces a new resource to be created.
        pub virtual_machine_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GalleryApplicationAssignmentArgs,
    ) -> GalleryApplicationAssignmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let configuration_blob_uri_binding = args
            .configuration_blob_uri
            .get_output(context)
            .get_inner();
        let gallery_application_version_id_binding = args
            .gallery_application_version_id
            .get_output(context)
            .get_inner();
        let order_binding = args.order.get_output(context).get_inner();
        let tag_binding = args.tag.get_output(context).get_inner();
        let virtual_machine_id_binding = args
            .virtual_machine_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/galleryApplicationAssignment:GalleryApplicationAssignment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurationBlobUri".into(),
                    value: &configuration_blob_uri_binding,
                },
                register_interface::ObjectField {
                    name: "galleryApplicationVersionId".into(),
                    value: &gallery_application_version_id_binding,
                },
                register_interface::ObjectField {
                    name: "order".into(),
                    value: &order_binding,
                },
                register_interface::ObjectField {
                    name: "tag".into(),
                    value: &tag_binding,
                },
                register_interface::ObjectField {
                    name: "virtualMachineId".into(),
                    value: &virtual_machine_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GalleryApplicationAssignmentResult {
            configuration_blob_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurationBlobUri"),
            ),
            gallery_application_version_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("galleryApplicationVersionId"),
            ),
            order: pulumi_gestalt_rust::__private::into_domain(o.extract_field("order")),
            tag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tag")),
            virtual_machine_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualMachineId"),
            ),
        }
    }
}
