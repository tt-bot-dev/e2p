/**
 * Copyright (C) 2021 tt.bot dev team
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
use napi::bindgen_prelude::*;

#[napi]
fn resize_image(env: Env, image: Object, width: u32) -> Result<Object> {
    let image = JsImage::from(image)?;
    Ok(env.spawn(ResizeTask(image, width))?.promise_object())
}

#[napi]
fn composite_image(env: Env, image1: Object, image2: Object, x: u32, y: u32) -> Result<Object> {
    let image1 = JsImage::from(image1)?;
    let image2 = JsImage::from(image2)?;

    Ok(env.spawn(CompositeTask(image1, image2, x, y))?.promise_object())
}

#[napi(js_name = "encodeAPNG")]
fn generate_apng(env: Env, frames: Vec<Object>) -> Result<Object> {
    let images: Result<Vec<_>> = frames.into_iter().map(|val| JsImage::from(val)).collect();

    Ok(env.spawn(GenerateAPNGTask(images?))?.promise_object())
}

#[napi(js_name = "encodeGIF")]
fn generate_gif(env: Env, frames: Vec<Object>) -> Result<Object> {
    let images: Result<Vec<_>> = frames.into_iter().map(|val| JsImage::from(val)).collect();

    Ok(env.spawn(GenerateGIFTask(images?))?.promise_object())
}

#[napi(js_name = "decodeGIF")]
fn decode_gif(env: Env, data: Uint8Array) -> Result<Object> {
    Ok(env.spawn(DecodeGIFTask(Vec::from(&data as &[u8])))?.promise_object())
}

#[napi(js_name = "decodePNG")]
fn decode_png(env: Env, data: Uint8Array) -> Result<Object> {
    Ok(env.spawn(DecodePNGTask(Vec::from(&data as &[u8])))?.promise_object())
}
