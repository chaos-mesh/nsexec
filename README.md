# nsexec

Just like `nsenter`, `nsexec` run program in different namespaces.

The most significant benefit of `nsexec` is that it can run binaries that don't exist in the target namespace. For example, if the target namespace is a `distroless` image, the `ls`, `bash` and `cat` don't exist.

## Example

```bash
nsexec -m /proc/xxxx/ns/mnt -l /bin/bash
```

The option `--library-path` should be set to the path of `libnsenter.so`. If you compile `nsexec` with `cargo build --release --all`, you can find it in `./target/release/libnsenter.so`. The default `library-path` is `/usr/local/lib/libnsenter.so`

The option `-l, --local` means to load the binaries from the current mnt namespace (but not target namespace).

## Implementation

`nsexec` has a lot of limitations. The best way to understand the limitation is to know the implementation and choose whether to use it in your situation.

Without `-l, --local`, the implementation is calling `setns` directly and spawn a new child process, which is nearly the same as `nsenter`.

With `-l, --local`, we will not call `setns` to set mount namespace in `nsexec`. Instead, `nsexec` will modify the `LD_PRELOAD` environment variable of the child process to preload a dynamic library. The library, as shown in `/nsenter/src/lib.rs`, will call `setns` to set mount namespace in the constructor.

## Note

`nsexec` will also bypass some signals such as `SIGTERM` and `SIGINT` to the child process, for the convenience of killing processes.
