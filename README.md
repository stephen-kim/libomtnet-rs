# libomtnet-rs

Rust implementation of the Open Media Transport (OMT) network protocol library.

Shared library used by [omt-encoder](https://github.com/stephen-kim/omt-encoder) and [omt-decoder](https://github.com/stephen-kim/omt-decoder).

## Features

- OMT frame codec (encode/decode over TCP)
- TCP client and server with optimized socket buffers
- Frame types: Video, Audio, Metadata
- Video codecs: VMX1, H.264, H.265, BGRA, NV12, UYVY, YUY2, etc.
- Per-client codec and quality negotiation
- mDNS service constants and URL handling

## Usage

Add as a git submodule or path dependency:

```toml
[dependencies]
libomtnet = { path = "../libomtnet" }
```

## License

MIT License. See [LICENSE.txt](LICENSE.txt) for details.
