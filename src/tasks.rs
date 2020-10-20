/**
 * Copyright (C) 2020 tt.bot dev team
 * 
 * This file is part of @tt-bot-dev/e2p.
 * 
 * @tt-bot-dev/e2p is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * @tt-bot-dev/e2p is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with @tt-bot-dev/e2p.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::js_image::*;
use apng::load_dynamic_image;
use gif::Encoder as GifEncoder;
use image::{
    gif::GifDecoder,
    imageops::{overlay, FilterType},
    png::PngDecoder,
    AnimationDecoder, DynamicImage, RgbaImage,
};
use napi::*;
use std::io::Cursor;

pub(crate) struct ResizeTask(pub(crate) JsImage, pub(crate) u32);

impl Task for ResizeTask {
    type Output = DynamicImage;
    type JsValue = JsObject;

    fn compute(&mut self) -> Result<Self::Output> {
        let image = RgbaImage::from_raw(self.0.width, self.0.height, self.0.data.clone());

        if image.is_none() {
            return Err(Error::from_reason("Invalid image".to_owned()));
        }

        let dyn_image = DynamicImage::ImageRgba8(image.unwrap());

        Ok(dyn_image.resize(self.1, self.1, FilterType::Triangle))
    }

    fn resolve(&self, env: &mut Env, output: Self::Output) -> Result<Self::JsValue> {
        JsImage::dyn_image_into_js_object(env, output)
    }
}

pub(crate) struct CompositeTask(
    pub(crate) JsImage,
    pub(crate) JsImage,
    pub(crate) u32,
    pub(crate) u32,
);

impl Task for CompositeTask {
    type Output = RgbaImage;
    type JsValue = JsObject;

    fn compute(&mut self) -> Result<Self::Output> {
        let image1 = RgbaImage::from_raw(self.0.width, self.0.height, self.0.data.clone());

        if image1.is_none() {
            return Err(Error::from_reason("Invalid bottom image".to_owned()));
        }

        let image2 = RgbaImage::from_raw(self.1.width, self.1.height, self.1.data.clone());

        if image2.is_none() {
            return Err(Error::from_reason("Invalid top image".to_owned()));
        }

        let mut image1 = image1.unwrap();
        let image2 = image2.unwrap();

        overlay(&mut image1, &image2, self.2, self.3);
        Ok(image1)
    }

    fn resolve(&self, env: &mut Env, output: Self::Output) -> Result<Self::JsValue> {
        JsImage::image_into_js_object(env, output)
    }
}

pub(crate) struct GenerateAPNGTask(pub(crate) Vec<JsImage>);

impl Task for GenerateAPNGTask {
    type Output = Vec<u8>;
    type JsValue = JsBuffer;

    fn compute(&mut self) -> Result<Self::Output> {
        let mut vec = Vec::new();

        let images = self.0.iter().map(|v| {
            let img = RgbaImage::from_raw(v.width, v.height, v.data.clone());
            if img.is_none() {
                return Err(Error::from_reason("Invalid image data".to_owned()));
            }
            let dyn_img = load_dynamic_image(image::DynamicImage::ImageRgba8(img.unwrap()));
            if dyn_img.is_err() {
                return Err(Error::from_reason("Invalid image data".to_owned()));
            }

            Ok(dyn_img.unwrap())
        });
        let mut more_images: Vec<_> = Vec::new();

        for img in images {
            if let Err(e) = img {
                return Err(e);
            } else {
                more_images.push(img.unwrap())
            }
        }

        {
            let config = apng::create_config(&more_images, None);
            if let Err(_) = config {
                return Err(Error::from_reason("Cannot encode image".to_owned()));
            }

            let enc = apng::Encoder::new(&mut vec, config.unwrap());
            if let Err(_) = enc {
                return Err(Error::from_reason("Cannot encode image".to_owned()));
            }

            let mut enc = enc.unwrap();

            let frame = apng::Frame {
                delay_num: Some(20),
                delay_den: Some(1000),
                ..Default::default()
            };

            if let Err(_) = enc.encode_all(more_images, Some(&frame)) {
                return Err(Error::from_reason("Couldn't encode APNG".to_owned()));
            }
        }

        Ok(vec)
    }

    fn resolve(&self, env: &mut Env, output: Self::Output) -> Result<Self::JsValue> {
        env.create_buffer_with_data(output)
    }
}

pub(crate) struct GenerateGIFTask(pub(crate) Vec<JsImage>);

impl Task for GenerateGIFTask {
    type Output = Vec<u8>;
    type JsValue = JsBuffer;

    fn compute(&mut self) -> Result<Self::Output> {
        let mut vec = Vec::new();

        let images = self.0.iter().map(|v| {
            let img = RgbaImage::from_raw(v.width, v.height, v.data.clone());
            if img.is_none() {
                return Err(Error::from_reason("Invalid image data".to_owned()));
            }

            Ok(img.unwrap())
        });
        let mut more_images: Vec<_> = Vec::new();

        for img in images {
            if let Err(e) = img {
                return Err(e);
            } else {
                more_images.push(img.unwrap())
            }
        }
        {
            let enc = GifEncoder::new(
                &mut vec,
                more_images[0].width() as u16,
                more_images[0].height() as u16,
                &[],
            );

            if let Err(_) = enc {
                return Err(Error::from_reason("Cannot encode image".to_owned()));
            }

            let mut enc = enc.unwrap();

            for image in more_images {
                let mut frame = gif::Frame::from_rgba_speed(
                    image.width() as u16,
                    image.height() as u16,
                    &mut image.to_vec(),
                    10,
                );

                frame.delay = 2; // 2 * 10 ms
                frame.dispose = gif::DisposalMethod::Background;
                if let Err(_) = enc.write_frame(&frame) {
                    return Err(Error::from_reason("Couldn't encode GIF".to_owned()));
                }
            }
            if let Err(_) = enc.set_repeat(gif::Repeat::Infinite) {
                return Err(Error::from_reason("Couldn't encode GIF".to_owned()));
            }
        }

        Ok(vec)
    }

    fn resolve(&self, env: &mut Env, output: Self::Output) -> Result<Self::JsValue> {
        env.create_buffer_with_data(output)
    }
}

pub(crate) struct DecodeGIFTask(pub(crate) Vec<u8>);

impl Task for DecodeGIFTask {
    type Output = Vec<Result<JsAnimatedImage>>;
    type JsValue = JsObject;

    fn compute(&mut self) -> Result<Self::Output> {
        let cur = Cursor::new(&self.0);
        let dec = GifDecoder::new(cur);

        if let Err(_) = dec {
            return Err(Error::from_reason("Invalid GIF image".to_owned()));
        }

        let dec = dec.unwrap();

        let frames = dec.into_frames();

        Ok(frames
            .map(|i| {
                if let Err(_) = i {
                    return Err(Error::from_reason("Invalid image data".to_owned()));
                }

                let f = i.unwrap();
                let image = JsAnimatedImage {
                    data: f.buffer().to_vec(),
                    delay: f.delay().numer_denom_ms(),
                    x: f.left(),
                    y: f.top(),
                    width: f.buffer().width(),
                    height: f.buffer().height(),
                };
                Ok(image)
            })
            .collect::<Self::Output>())
    }

    fn resolve(&self, env: &mut Env, output: Self::Output) -> Result<Self::JsValue> {
        let mut arr = env.create_array_with_length(output.len())?;

        for i in 0..output.len() {
            let buf = output[i].as_ref();
            if let Err(e) = buf {
                return Err(e.clone());
            }

            let buf = buf.unwrap();
            let mut out = env.create_object()?;

            out.set_named_property("data", env.create_buffer_with_data(buf.data.clone())?)?;
            out.set_named_property("width", env.create_uint32(buf.width)?)?;
            out.set_named_property("height", env.create_uint32(buf.height)?)?;
            out.set_named_property("delay", env.create_uint32(buf.delay.0 / buf.delay.1)?)?;
            out.set_named_property("x", env.create_uint32(buf.x)?)?;
            out.set_named_property("y", env.create_uint32(buf.y)?)?;
            arr.set_index(i, out)?;
        }

        Ok(arr)
    }
}

pub(crate) struct DecodePNGTask(pub(crate) Vec<u8>);

impl Task for DecodePNGTask {
    type Output = DynamicImage;
    type JsValue = JsObject;

    fn compute(&mut self) -> Result<Self::Output> {
        let cur = Cursor::new(&self.0);
        let dec = PngDecoder::new(cur);

        if let Err(_) = dec {
            return Err(Error::from_reason("Invalid PNG image".to_owned()));
        }

        let dec = dec.unwrap();
        let dyn_img = DynamicImage::from_decoder(dec);

        if let Err(_) = dyn_img {
            return Err(Error::from_reason("Invalid PNG image".to_owned()));
        }

        let dyn_img = dyn_img.unwrap();
        Ok(dyn_img)
    }

    fn resolve(&self, env: &mut Env, output: Self::Output) -> Result<Self::JsValue> {
        JsImage::dyn_image_into_js_object(env, output)
    }
}
