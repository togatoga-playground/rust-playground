#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EnumValue {
    True,
    False,
    UnDef,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StructValue(pub u8);
impl StructValue {
    pub const TRUE: StructValue = StructValue(0);
    pub const FALSE: StructValue = StructValue(1);
    pub const UNDEF: StructValue = StructValue(2);
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Value {
    True,
    False,
}
