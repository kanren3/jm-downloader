use base64::prelude::*;
use image::{DynamicImage, GenericImage, GenericImageView};
use md5;
use std::str::FromStr;

fn get_num(aid_b64: &str, pid_b64: &str) -> i32 {
    let aid_str = String::from_utf8(BASE64_STANDARD.decode(aid_b64).unwrap()).unwrap();
    let pid_str = String::from_utf8(BASE64_STANDARD.decode(pid_b64).unwrap()).unwrap();

    let combine_str = aid_str.clone() + &pid_str;

    let digest = md5::compute(combine_str.as_bytes());
    let combine_str_md5 = format!("{:x}", digest);

    let last_char = combine_str_md5.chars().last().unwrap();
    let mut last_number = last_char as u8;

    let aid: i32 = aid_str.parse().unwrap();

    if aid >= 268850 && aid <= 421925 {
        last_number %= 10;
    } else if aid >= 421926 {
        last_number %= 8;
    }

    let mut a = 10;
    match last_number {
        0 => a = 2,
        1 => a = 4,
        2 => a = 6,
        3 => a = 8,
        4 => a = 10,
        5 => a = 12,
        6 => a = 14,
        7 => a = 16,
        8 => a = 18,
        9 => a = 20,
        _ => (),
    }
    a
}

pub fn restore_image(input_image_path: &str, output_image_path: &str, aid: &str, pid: &str) {
    let mut input_image = image::open(input_image_path).unwrap();

    if aid.parse::<i32>().unwrap() < 220971 {
        input_image.save(output_image_path).unwrap();
        return;
    }

    let (image_width, image_height) = input_image.dimensions();

    let s = get_num(
        BASE64_STANDARD
            .encode(String::from_str(aid).unwrap())
            .as_str(),
        BASE64_STANDARD
            .encode(String::from_str(pid).unwrap())
            .as_str(),
    );

    let r = image_height % s as u32;

    let mut output_image = DynamicImage::new_rgb8(image_width, image_height);

    for m in 0..s {
        let c = image_height / s as u32;
        let g = c * m as u32;

        let mut h = image_height - c * (m + 1) as u32 - r;

        if m == 0 {
            h += r;
        }

        let section = input_image.crop(0, h, image_width, h + c);
        output_image.copy_from(&section, 0, g).unwrap();
    }

    output_image.save(output_image_path).unwrap();
}