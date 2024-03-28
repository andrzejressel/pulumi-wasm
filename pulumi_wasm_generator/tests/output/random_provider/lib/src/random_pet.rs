

pub struct RandomPetArgs<'a> {
    pub name: &'a str,
    pub keepers: Object,
    pub length: int,
    pub prefix: String,
    pub separator: String,

}