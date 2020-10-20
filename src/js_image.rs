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

use napi::*;
use std::convert::TryInto;

#[derive(Debug)]
pub(crate) struct JsImage {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) data: Vec<u8>,
}

#[derive(Debug)]
pub(crate) struct JsAnimatedImage {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) data: Vec<u8>,
    pub(crate) delay: (u32, u32),
    pub(crate) x: u32,
    pub(crate) y: u32
}


impl JsImage {
    pub(crate) fn from(obj: JsObject) -> Result<Self> {
        let buf: JsBuffer = obj.get_named_property("data")?;
        let width: u32 = obj.get_named_property::<JsNumber>("width")?.try_into()?;
        let height: u32 = obj.get_named_property::<JsNumber>("height")?.try_into()?;

        let typed_array = Vec::from(&buf as &[u8]);
        Ok(Self {
            width: width,
            height: height,
            data: typed_array,
        })
    }

    pub(crate) fn dyn_image_into_js_object(env: &Env, dyn_image: image::DynamicImage) -> Result<JsObject> {
        JsImage::image_into_js_object(env, dyn_image.to_rgba())
    }

    pub(crate) fn image_into_js_object(env: &Env, image: image::RgbaImage) -> Result<JsObject> {
        let mut out = env.create_object()?;
        
        let data = image.to_vec();
        out.set_named_property(
            "data",
            env.create_buffer_with_data(data)?,
        )?;

        out.set_named_property("width", env.create_uint32(image.width())?)?;
        out.set_named_property(
            "height",
            env.create_uint32(image.height())?,
        )?;

        Ok(out)
    }

    pub(crate) fn image_into_js_object_with_ctx(ctx: CallContext, image: image::RgbaImage) -> Result<JsObject> {
        JsImage::image_into_js_object(ctx.env, image)
    }
}
