mod circle;
mod ellipse;
mod line;
mod path;
mod poly;
mod rect;

pub use self::{circle::Circle, ellipse::Ellipse, line::Line, path::Path, poly::Poly, rect::Rect};

use super::{DebugAttributes, HasBase};
use crate::prelude::*;
use skia_bindings as sb;

pub type Shape = RCHandle<sb::SkSVGShape>;

impl NativeRefCountedBase for sb::SkSVGShape {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGShape {
    type Base = sb::SkSVGTransformableNode;
}

impl DebugAttributes for Shape {
    const NAME: &'static str = "Shape";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(builder);
    }
}
