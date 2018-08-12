# Running the tests

The library also contains unit and integration tests which exercise the D3D9 API.
If you are a developer, or a user looking to verify their install works,
you might want to run these tests.

**Note**: currently, the tests can only be built and run for 64-bit targets,
due to [this bug with Rust](https://github.com/rust-lang/rust/issues/47493).

Building and running the tests is quite straightforward.

```sh
# It's a good idea to disable logging for the other libraries
# in order to better see the test output.
export WINEDEBUG=-all
export DXVK_LOG_LEVEL=none

cargo test --all --target x86_64-pc-windows-gnu
```

**Note**: this assumes you are running on Windows, or that you have Wine installed
and you can run Windows executables just like normal Linux ones.
