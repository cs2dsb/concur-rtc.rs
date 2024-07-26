[![Crate](https://img.shields.io/crates/v/concur-rtc.svg)](https://crates.io/crates/concur-rtc)
[![Documentation](https://docs.rs/concur-rtc/badge.svg)](https://docs.rs/concur-rtc)
[![Workflow Status](https://github.com/cs2dsb/concur-rtc.rs/actions/workflows/build.yml/badge.svg)](https://github.com/cs2dsb/concur-rtc.rs/actions?query=workflow%3A%22build%22)
![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)

# concur-rtc

WebRTC datachannels with the `perfect negotiation` pattern

## Features

* `tracing` - enables the [`tracing`] crate and logs everything it's doing
* `state-events` - changes the Item type of the stream to be an enum that is either a message or
  a status change Both are enabled by default

## Docs

[Code docs](https://cs2dsb.github.io/concur-rtc.rs)

## License

[MIT](LICENSE)