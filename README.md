# Ray tracer challenge in Rust.

Based on a book [The Ray Tracer Challenge](https://pragprog.com/book/jbtracer/the-ray-tracer-challenge) by Jamis Buck

## Motivation

I wanted to do this challenge for a while but couldn't really decide on a language. I wanted something robust, with good standard library, built in testing. Something that is relatively fast. Before this I have had a single rust project that was a complete failure. It had two external crates, very long compile time and big binary size. I was very disappointed in Rust. But after taking a short break from it this, I decided to give it a go. And here I am, giving it another go.

I have coded quite a bit of it, but decided to upload each chapter in a clean way to github.

## Dependencies

* `image-rs` - aka "I simply hate working with PPM files." And you can't really flex with PPM's
* `rayon` - image rendering improvements. (I am stupid and have no idea how to actually make this code threaded, so instead I am trying to clobber this using parallelism. Shockingly performance improvements are incredible. World rendering with rayon is only 15s for the canvas of 100/100 while without it the average render time is 40s. In the release with the canvas of 1000/1000 pixels the rendering without rayon is 120s average, with rayon it averages to 50s.