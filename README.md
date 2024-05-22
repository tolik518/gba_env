# GBA env

Crate for detecting the environment for Game Boy Advance ROMs (GBA/NDS/mGBA/VBA/gpSP/etc)

You can find the documentation for gba_env [here](https://docs.rs/tolik518/latest/gba_env/).

## Usage

You would need to use the nightly rust version and add this to your `Cargo.toml`:

```toml
[dependencies]
gba_env = "1.0"
```

Then you can use the crate in your code:

```rust
use gba_env;
use gba_env::Environment;

fn main() {
    let env = gba_env::get_env();
    if env == Environment::GpSp {
        println!("Sorry, but this ROM is not supported on gpSP.");
    } 
}
```

## License
This project is licensed under the GNU GPLv3 or MIT or Apache-2.0 License.
Just pick the one that fits your needs.

## Appendix
All the information about the GBA environment detection were taken from the [gbadev](https://gbadev.net/) Discord server. So a huge thanks to the gbadev community!
