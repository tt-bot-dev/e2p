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
declare namespace e2p {
    export interface Image {
        data: Buffer;
        height: number;
        width: number;
    }

    export interface AnimatedImage extends Image {
        delay: number;
        x: number;
        y: number;
    }

    export function resizeImage(image: Image, width: number): Promise<Image>;
    export function compositeImage(image: Image, image2: Image, x: number, y: number): Promise<Image>;
    export function encodeAPNG(frames: Image[]): Promise<Buffer>;
    export function encodeGIF(frames: Image[]): Promise<Buffer>;
    export function decodeGIF(buffer: Buffer): Promise<AnimatedImage[]>;
    export function decodePNG(buffer: Buffer): Promise<AnimatedImage[]>;
}

export = e2p;
