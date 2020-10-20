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

extern crate napi;
#[macro_use]
extern crate napi_derive;

mod js_image;
mod tasks;
use js_image::*;
use tasks::*;

use napi::*;
use std::convert::TryInto;

#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(windows)]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

// (image1: Image, width: u32) => Image
#[js_function(2)]
fn resize_image(ctx: CallContext) -> Result<JsObject> {
    let image: JsObject = ctx.get(0)?;
    let image = JsImage::from(image)?;
    let target_width = ctx.get::<JsNumber>(1)?.try_into()?;
    ctx.env.spawn(ResizeTask(image, target_width))
}

// (image1: Image, image2: Image, x: u32, y: u32) => Image
#[js_function(4)]
fn composite_image(ctx: CallContext) -> Result<JsObject> {
    let image1: JsObject = ctx.get(0)?;
    let image1 = JsImage::from(image1)?;

    let image2 = ctx.get(1)?;
    let image2 = JsImage::from(image2)?;

    let x = ctx.get::<JsNumber>(2)?.try_into()?;
    let y = ctx.get::<JsNumber>(3)?.try_into()?;

    ctx.env.spawn(CompositeTask(image1, image2, x, y))
}

// (frames: Image[]) => Buffer
#[js_function(1)]
fn generate_apng(ctx: CallContext) -> Result<JsObject> {
    let images: JsObject = ctx.get(0)?;
    if !images.is_array()? {
        return Err(Error::from_reason("Invalid image data".to_owned()));
    }

    let len = images.get_array_length()?;
    let mut vec: Vec<JsObject> = Vec::with_capacity(len as usize);

    for i in 0..len {
        vec.push(images.get_index(i)?)
    }

    let images = vec.into_iter().map(|val| JsImage::from(val));

    let images: Vec<_> = images.collect();
    let mut more_images: Vec<_> = Vec::new();

    for img in images {
        if let Err(e) = img {
            return Err(e);
        } else {
            more_images.push(img.unwrap())
        }
    }

    ctx.env.spawn(GenerateAPNGTask(more_images))
}

#[js_function(1)]
fn generate_gif(ctx: CallContext) -> Result<JsObject> {
    let images: JsObject = ctx.get(0)?;
    if !images.is_array()? {
        return Err(Error::from_reason("Invalid image data".to_owned()));
    }

    let len = images.get_array_length()?;
    let mut vec: Vec<JsObject> = Vec::with_capacity(len as usize);

    for i in 0..len {
        vec.push(images.get_index(i)?)
    }

    let images = vec.into_iter().map(|val| JsImage::from(val));

    let images: Vec<_> = images.collect();
    let mut more_images: Vec<_> = Vec::new();

    for img in images {
        if let Err(e) = img {
            return Err(e);
        } else {
            more_images.push(img.unwrap())
        }
    }

    ctx.env.spawn(GenerateGIFTask(more_images))
}

// (data: Buffer) => AnimatedImage
#[js_function(1)]
fn decode_gif(ctx: CallContext) -> Result<JsObject> {
    let byte_array: JsBuffer = ctx.get(0)?;

    ctx.env
        .spawn(DecodeGIFTask(Vec::from(&byte_array as &[u8])))
}

// (data: Buffer) => Image
#[js_function(1)]
fn decode_png(ctx: CallContext) -> Result<JsObject> {
    let byte_array: JsBuffer = ctx.get(0)?;

    ctx.env
        .spawn(DecodePNGTask(Vec::from(&byte_array as &[u8])))
}

register_module!(tt_bot_e2p, init);

fn init(module: &mut Module) -> Result<()> {
    module.create_named_method("resizeImage", resize_image)?;
    module.create_named_method("compositeImage", composite_image)?;
    module.create_named_method("encodeAPNG", generate_apng)?;
    module.create_named_method("encodeGIF", generate_gif)?;
    module.create_named_method("decodeGIF", decode_gif)?;
    module.create_named_method("decodePNG", decode_png)?;

    Ok(())
}
