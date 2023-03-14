# Autoterm-rs

## Ubuntu 20.04.5 LTS Desktop VM

```bash
$ rm target/*.png
$ HOST="rome"; dpkg --get-selections |sort > assets/packages_$(date +'%Y%m%d')_$HOST.txt
```
## Build
``` bash
$ apt install -y build-essential g++ wget curl git unzip
$ apt install -y clang libclang-dev libxcb1 libxcb1-dev
$ apt install -y libgtk2.0-dev libavcodec-dev libavformat-dev libswscale-dev 
$ apt install -y cmake pkgconf librust-vcpkg-dev

# libgtkglext1-dev - OpenGL Extension to GTK+ (development files)
# libgflags-dev - commandline flags module for C++ (development files)
# glogg - Smart interactive log explorer using Qt
# hdf5-tools - HDF5 - Runtime tools
sudo apt install -y libgtkglext1-dev libgflags-dev glogg hdf5-tools

$ mkdir /code/opencv_4.7.0_contrib && cd /code/opencv_4.7.0_contrib
$ git clone https://github.com/opencv/opencv.git
$ git clone https://github.com/opencv/opencv_contrib.git

$ cd opencv && mkdir -p build && cd build

$ cmake -D CMAKE_BUILD_TYPE=RELEASE \
    -D CMAKE_INSTALL_PREFIX=/usr/local \
    -D INSTALL_C_EXAMPLES=OFF \
    -D INSTALL_PYTHON_EXAMPLES=OFF \
    -D OPENCV_GENERATE_PKGCONFIG=ON \
    -D OPENCV_EXTRA_MODULES_PATH=../../opencv_contrib/modules \
    -D BUILD_EXAMPLES=OFF ..
    
    >> output.txt

$ make -j$(nproc)
$ sudo make install

$ pkg-config --modversion opencv4
    4.7.0
```
## Errors
``` bash
    error: linking with `cc` failed: exit status: 1
    ...
    = note: /usr/bin/ld: cannot find -lxcb: No such file or directory
            collect2: error: ld returned 1 exit status

# libxcb1 - X C Binding
# libxcb1-dev - X C Binding, development files

$ apt install -y libxcb1 libxcb1-dev
```

``` bash
Error: "Failed to find installed OpenCV package using probes: environment, pkg_config, cmake, vcpkg_cmake, vcpkg, refer to https://github.com/twistedfall/opencv-rust#getting-opencv for help"

# cmake - cross-platform, open-source make system
# pkgconf - manage compile and link flags for libraries
# librust-vcpkg-dev - Find native dependencies in a vcpkg tree at build time - Rust source code
$ apt install -y cmake pkgconf librust-vcpkg-dev
```
``` bash
target/debug/screenshots_example: error while loading shared libraries: libopencv_gapi.so.407: cannot open shared object file: No such file or directory

$ sudo find / -name libopencv_gapi.so
    /usr/local/lib/libopencv_gapi.so

$ nano ~/.bashrc
    export LD_LIBRARY_PATH=/usr/local/lib


```

