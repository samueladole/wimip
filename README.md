# wimip

A simple Rust CLI tool to fetch your public IP address using [ifconfig.io](https://ifconfig.io), print it to the console (just like `curl ifconfig.io`), and copy it to your clipboard on macOS, Windows, or Linux.

## Features

- Fetches your public IP address via `https://ifconfig.io`
- Prints only the IP address (no extra text)
- Copies the IP to your clipboard:
  - Uses `pbcopy` on macOS
  - Uses `clip` on Windows
  - Uses `xclip` or `xsel` on Linux (if available)

## Requirements

- Rust (edition 2021)
- For clipboard support:
  - macOS: `pbcopy` (default on macOS)
  - Windows: `clip` (default on Windows)
  - Linux: `xclip` or `xsel` (install via your package manager)

## Usage


https://github.com/user-attachments/assets/d54ce47e-3760-4b4a-9a84-2c79faadc0cd


```sh
cargo run --release
```

## Example output

```sh
Your IP: 203.0.113.42
```

The IP is also copied to your clipboard if a supported clipboard tool is available.

## Tests

Unit tests are included for the IP parsing logic. Run them with:

```sh
cargo test
```

## Code Overview

Uses `std::process::Command` to call `curl` and clipboard tools.
Uses `tokio` for async main (though the code is synchronous).
Unit tests simulate `curl` output and check IP parsing.

## License

MIT

Author
Samuel Adole
