# Ray tracer challenge in Rust.

Based on a book [The Ray Tracer Challenge](https://pragprog.com/book/jbtracer/the-ray-tracer-challenge) by Jamis Buck

## Motivation

I wanted to do this challenge for a while but couldn't really decide on a language. I wanted something robust, with good standard library, built in testing. Something that is relatively fast. Before this I have had a single rust project that was a complete failure. It had two external crates, very long compile time and big binary size. I was very disappointed in Rust. But after taking a short break from it this, I decided to give it a go. And here I am, giving it another go.

I have coded quite a bit of it, but decided to upload each chapter in a clean way to github.

## Dependencies

I only have a single depency that is required for build - `image-rs`. Because rust Standard Library lacks any kind of image processing and I dislike .ppm files.
