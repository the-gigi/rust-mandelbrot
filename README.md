# rust-mandelbrot
Draw the [Mandelbrot Set](https://en.wikipedia.org/wiki/Mandelbrot_set) using Rust

This is a Rust learning project. The goal isn't to produce an amazing and efficient rendering of the Mandelbrot set.

It is a very rudimentary and basic implementation.

Its claim to fame is that it is slowly renders the pixels in a spiral expanding from the center.

I implemented a simple Complex struct (see what I did there?) just because. Don't use it for anything else as Rust has
a proper Complex crate.

# Requirements

The code uses the [Rust-SDL2](https://github.com/Rust-SDL2/rust-sdl2) crate. 
It requires installing the sdl2 library. 

## MacOS

Follow the instructions here:
https://github.com/Rust-SDL2/rust-sdl2#homebrew

## Windows

Follow the instructions here:
https://github.com/Rust-SDL2/rust-sdl2#windows-msvc
 

# Reference

Here is another simple Rust implementations:

https://rotgers.io/posts/mandelbrot-rust/


