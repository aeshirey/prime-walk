# prime-walk
After reading [this reddit post](https://www.reddit.com/r/dataisbeautiful/comments/jdmxby/oc_prime_numbers_whenever_n_was_a_prime_number/), I thought it would be interesting to reproduce this in my language du jour, Rust. I found this surprisingly easy by using the [`imageproc` crate](https://github.com/image-rs/imageproc). It also uses [`reikna`](https://phillip-h.github.io/reikna/docs/reikna/index.html)'s Segmented Seive of Eratosthenes to quickly generate primes.

This project is a standalone binary you run from the command prompt. You can pass in `-pMAX_NUMBER` where it will first generate a list of all prime numbers up to `MAX_NUMBER`. This may take a while depending on the value you specify and your machine. (On an i7 @ 2.2GHz with 24GB RAM in WSL2 Ubuntu, prime generation takes about 1m10s and the main process takes 30s.) After the file exists, it will generate an image according to the following process:

1. Start at coordiantes (0, 0), facing North. (Direction can be configured.)
2. Move forward until the next prime number is encountered, at which point, turn Right. (Angle to turn can be configured.)
3. Continue walking forward and turning for every prime number until the input primes are exhausted.

The current implementation is a bit hacky. I have no current plans to continue, but I'll accept pull requests if you feel like improving it. Notable TODOs are:

* [ ] Cleanup the `PrimeWalk` struct to more reasonably split the two steps into their logical processes.
* [ ] Specify the output image size up-front (rather than in prime\_walk.rs) and generate the `scale` value at runtime. The current output isn't a properly cropped image.

Here's the output for 1B records and the default settings (North, turn Right), manually cropped:

![Primes below one billion](https://raw.githubusercontent.com/aeshirey/prime-walk/main/primes.png)
