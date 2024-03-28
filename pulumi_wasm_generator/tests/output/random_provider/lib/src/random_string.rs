

pub struct RandomStringArgs<'a> {
    pub name: &'a str,
    pub keepers: Object,
    pub length: int,
    pub lower: boolean,
    pub minLower: int,
    pub minNumeric: int,
    pub minSpecial: int,
    pub minUpper: int,
    pub number: boolean,
    pub numeric: boolean,
    pub overrideSpecial: String,
    pub special: boolean,
    pub upper: boolean,

}