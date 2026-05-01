use crate::filters::Filter;
use image::DynamicImage;

pub struct Rotate {
    pub degree: u32,
}

impl Filter for Rotate {
    fn apply(&self, img: DynamicImage) -> DynamicImage {
        match self.degree {
            90 => img.rotate90(),
            180 => img.rotate180(),
            270 => img.rotate270(),
            _ => img,
        }
    }
}
