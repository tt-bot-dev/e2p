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

    export function resizeImage(image: Image, width: number): Image;
    export function compositeImage(image: Image, image2: Image, x: number, y: number): Image;
    export function encodeAPNG(frames: Image[]): Buffer;
    export function encodeGIF(frames: Image[]): Buffer;
    export function decodeGIF(buffer: Buffer): AnimatedImage[];
    export function decodePNG(buffer: Buffer): AnimatedImage[];
}

export = e2p;