use std::{
    path::Path,
    io::{self, Error, ErrorKind},
};
use image::{
    Pixel,
    ImageResult,
    RgbImage,
    Rgb,
};
use crate::k_means::{
    k_means_cluster,
    assign_centroid,
};

// thanks https://lospec.com/palette-list/funkyfuture-8
const COLOR_PALLETE: [Rgb<u8>; 8] = [
    Rgb([0x2b, 0x0f, 0x54]),
    Rgb([0xab, 0x1f, 0x65]),
    Rgb([0xff, 0x4f, 0x69]),
    Rgb([0xff, 0xf7, 0xf8]),
    Rgb([0xff, 0x81, 0x42]),
    Rgb([0xff, 0xda, 0x45]),
    Rgb([0x33, 0x68, 0xdc]),
    Rgb([0x49, 0xe7, 0xec]),
];
const DARK_THRESHOLD: u8 = 0x40;
const ITERATION: u32 = 20;

pub fn blend_io(k: usize, input: impl AsRef<Path>, output: impl AsRef<Path>) -> io::Result<()> {
    blend(k, input, output)
        .map_err(|e| Error::new(ErrorKind::Other, e))
}

/// workflow of this function:
/// 1. load image in greyscale
/// 2. calculate k centroids of all the dark pixels
/// 3. blend each pixel with some color based on their nearest centroid
/// 4. save to output path
fn blend(k: usize, input: impl AsRef<Path>, output: impl AsRef<Path>) -> ImageResult<()> {
    let img = image::open(input)?.to_luma();

    let dark_pixels: Vec<_> = img.enumerate_pixels()
        .filter_map(|(x, y, p)| {
            let luma = p[0];
            if luma <= DARK_THRESHOLD {
                Some((x as f64, y as f64))
            } else {
                None
            }
        })
        .collect();
    
    let centroids = k_means_cluster(&dark_pixels, k, ITERATION);

    let mut blended = RgbImage::new(img.width(), img.height());
    for (x, y, p) in img.enumerate_pixels() {
        let i = assign_centroid(&centroids, (x as f64, y as f64));
        let mask = COLOR_PALLETE[i % COLOR_PALLETE.len()];
        blended.put_pixel(x, y, mix_rgb(p.to_rgb(), mask));
    }

    blended.save(output)?;
    Ok(())
}

fn mix_rgb(orig: Rgb<u8>, mask: Rgb<u8>) -> Rgb<u8> {
    let mut channels = [0; 3];
    for (i, channel) in channels.iter_mut().enumerate() {
        *channel = ((orig[i] as u16 + mask[i] as u16) / 2) as u8;
    }
    Rgb(channels)
}