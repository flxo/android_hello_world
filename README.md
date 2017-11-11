# Super simple android hello world

Please check `build.rs`!

Install NDK via

```
brew cask install android-ndk
```

Setup `NDK_HOME` accoring to the `brew` output during installation or adjust the path below.

Prepare a standalone toolchain:

```
$NDK_HOME/build/tools/make-standalone-toolchain.sh --install-dir=~/bin/ndk-toolchain
```

If `install-dir` is something else it has to be adjusted in `build.rs`.
