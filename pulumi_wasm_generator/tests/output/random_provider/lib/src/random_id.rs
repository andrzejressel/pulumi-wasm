

pub struct RandomIdArgs<'a> {
    pub name: &'a str,
    pub byteLength: int,
    pub keepers: Object,
    pub prefix: String,

}