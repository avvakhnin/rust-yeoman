use rltk::Rect;

pub trait QuasiRect {
    fn envelop_rect(&self) -> Rect;
}
impl QuasiRect for Rect {
    fn envelop_rect(&self) -> Rect {
        Rect {
            x1: self.x1.min(self.x2),
            x2: self.x1.max(self.x2) + 1,
            y1: self.y1.min(self.y2),
            y2: self.y1.max(self.y2) + 1,
        }
    }
}
