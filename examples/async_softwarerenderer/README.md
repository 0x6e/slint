<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: MIT -->
# Async Software Renderer Demo

This is a demonstration of the async software renderer API.

Run with `SLINT_BACKEND=winit-software` and `SLINT_ASYNC_RENDERING=1` to use the experimental API.

## Run the demo:

### Simulator

```sh
cargo run -p async-softwarerenderer --release
```

### STM32H735G-DK

Using [probe-rs](https://probe.rs).

```sh
CARGO_PROFILE_RELEASE_OPT_LEVEL=s CARGO_TARGET_THUMBV7EM_NONE_EABIHF_RUNNER="probe-rs run --chip STM32H735IGKx" cargo run -p async-softwarerenderer --no-default-features  --features=mcu-board-support/stm32h735g,renderer-experimental-software --target=thumbv7em-none-eabihf --release
```
