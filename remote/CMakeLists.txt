cmake_minimum_required(VERSION 3.0)
project(Remote)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)
#$ENV{HOME}/.cache/cargo-xwin/xwin/sdk/include/cppwinrt
include_directories(SYSTEM
	OpenXR-SDK/include
	.
	SampleShared
)

add_subdirectory(OpenXR-SDK)
#add_library(openxr_loader STATIC IMPORTED)
set_property(TARGET openxr_loader PROPERTY IMPORTED_LOCATION "OpenXR-SDK/src/loader/openxr_loader")

#add_library(remote STATIC DxUtility.cpp App.cpp SampleShared/CommandLineUtility.cpp SampleShared/SampleWindowWin32.cpp OpenXrProgram.cpp CubeGraphics.cpp)
#target_link_libraries(remote user32 gdi32 advapi32 d3dcompiler d3d11 dxgi OpenXR-SDK/src/loader/openxr_loader)

add_executable(app DxUtility.cpp App.cpp SampleShared/CommandLineUtility.cpp SampleShared/SampleWindowWin32.cpp OpenXrProgram.cpp CubeGraphics.cpp)
target_link_libraries(app user32 gdi32 advapi32 d3dcompiler d3d11 dxgi openxr_loader)

#Microsoft.Holographic.AppRemoting.OpenXr

#install(TARGETS remote DESTINATION lib)
