# Dyn platform targets

## New target

example

```shell
rustc +nightly -Z unstable-options --target=aarch64-unknown-none-softfloat --print target-spec-json > tools/plat-dyn/targets/aarch64-dyn-none-softfloat.json
```

build_args := \
  -Z unstable-options \
  --target /home/zhourui/arceos/arceos-vm-dev-dyn/aarch64-dyn-none-softfloat.json \
  -Z build-std=core,alloc \
  --target-dir $(TARGET_DIR) \
  $(build_args-$(MODE)) \
  $(verbose)