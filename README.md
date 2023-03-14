# Autoterm-rs

Ubuntu 20.04.5 LTS Desktop VM

```bash
$ dpkg --get-selections |sort > assets/packages_$(date +'%Y%m%d').txt
```

``` bash
$ apt install -y build-essential g++ wget curl git unzip
$ apt install -y clang libclang-dev libxcb1 libxcb1-dev
$ apt install -y libgtk2.0-dev libavcodec-dev libavformat-dev libswscale-dev 
$ apt install -y cmake pkgconf librust-vcpkg-dev

libgtkglext1-dev - OpenGL Extension to GTK+ (development files)
libgflags-dev - commandline flags module for C++ (development files)
glogg - Smart interactive log explorer using Qt
hdf5-tools - HDF5 - Runtime tools

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

error occurred: Command "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-gdwarf-4" "-fno-omit-frame-pointer" "-m64" "-I"

sudo find / -name "libopencv_core.so.*"
    /usr/local/lib/libopencv_core.so.4.7.0

Then I got the result: /usr/local/lib/libopencv_core.so.3.2.
2. Create a file called /etc/ld.so.conf.d/opencv.conf and write to it the path to the folder where the binary is stored.For example, I wrote /usr/local/lib/ to my opencv.conf file.
3. Run the command line as follows.

sudo echo "/usr/local/lib/" >> /etc/ld.so.conf.d/opencv.conf

sudo ldconfig -v

```



## OpenCV
Install opencv 4 on ubuntu 20.04
``` bash
# libopencv-dev - development files for opencv
# python3-opencv - Python 3 bindings for the computer vision library
$ apt install -y libopencv-dev clang libclang-dev python3-opencv
$ python3 -c "import cv2; print(cv2.__version__)"
    4.2.0
$ pkg-config --modversion opencv4
    4.2.0
```

OpenCV looks for codecs that are supplied by the OS 
``` bash
# libjpeg-dev - Independent JPEG Group's JPEG runtime library (dependency package)
# libpng-dev - PNG library - development (version 1.6)
# libtiff-dev - Tag Image File Format library (TIFF), development files
$ apt install -y libjpeg-dev libpng-dev libtiff-dev 

libgtk-3-dev \
libv4l-dev \
libxvidcore-dev libx264-dev \
gfortran openexr libatlas-base-dev
libtbb2 libtbb-dev libdc1394-22-dev libopenexr-dev \
libgstreamer-plugins-base1.0-dev libgstreamer1.0-dev

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


