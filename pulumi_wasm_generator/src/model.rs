pub(crate) enum TypeType {
    Boolean,
    Integer,
    Number,
    String,
    Array,
    Object,
}

pub(crate) enum TypeOrRef {
    Type(TypeType),
    Ref(String),
}

pub(crate) struct InputProperty {
    pub(crate) name: String,
    // pub(crate) r#type: TypeOrRef,
    // pub(crate) description: Option<String>,
    pub(crate) required: bool,
}

pub(crate) struct OutputProperty {
    pub(crate) name: String,
    // pub(crate) r#type: TypeOrRef,
    // pub(crate) description: Option<String>,
    pub(crate) required: bool,
}

pub(crate) struct Resource {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) input_properties: Vec<InputProperty>,
    pub(crate) output_properties: Vec<OutputProperty>
}

pub(crate) struct Package {
    pub(crate) name: String,
    pub(crate) display_name: Option<String>,
    pub(crate) version: String,
    pub(crate) resources: Vec<Resource>,
}