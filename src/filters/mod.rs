pub mod brighten;
pub mod grayscale;
pub mod rotate;

use image::DynamicImage;

pub trait Filter: Send + Sync {
    fn apply(&self, img: DynamicImage) -> DynamicImage;
}
