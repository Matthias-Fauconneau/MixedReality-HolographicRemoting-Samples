#cargo xwin build --release
TARGET=x86_64-pc-windows-msvc
PROFILE=release
CRATE=(basename $PWD)
BUILD=$CARGO_TARGET_DIR/$TARGET/$PROFILE/build/$CRATE-*/out/build
cmake --toolchain ~/.cache/cargo-xwin/cmake/$TARGET-toolchain.cmake -Sremote -B $BUILD
make -C $BUILD
cp RemotingXR.json $BUILD/
#wine reg add "HKLM\Software\Khronos\OpenXR\1" /v ActiveRuntime /d RemotingXR.json
#wine reg query "HKLM\Software\Khronos\OpenXR\1"
XR_LOADER_DEBUG=all wine $BUILD/app.exe
rm *.log
