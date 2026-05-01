use crate::filters::Filter;
use image::DynamicImage;

pub struct Brighten {
    pub value: i32,
}

impl Filter for Brighten {
    fn apply(&self, img: DynamicImage) -> DynamicImage {
        img.brighten(self.value)
    }
}
