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

pub(crate) struct Package {

}