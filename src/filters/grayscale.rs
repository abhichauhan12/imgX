use crate::filters::Filter;
use image::DynamicImage;

pub struct Grayscale;

impl Filter for Grayscale {
    fn apply(&self, img: DynamicImage) -> DynamicImage {
        img.grayscale()
    }
}
