use super::{DebugAttributes, HasBase};
use crate::{impl_default_make, prelude::*};
use skia_bindings as sb;

pub type Flood = RCHandle<sb::SkSVGFeFlood>;

impl NativeRefCountedBase for sb::SkSVGFeFlood {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeFlood {
    type Base = sb::SkSVGFe;
}

impl_default_make!(Flood, sb::C_SkSVGFeFlood_Make);

impl DebugAttributes for Flood {
    const NAME: &'static str = "FeFlood";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(builder);
    }
}
