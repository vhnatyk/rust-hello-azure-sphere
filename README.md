# Rust hello from C for Azure Sphere

Sample project to demonstrate development with Rust on Azure Sphere using Visual Studio and it's Azure Sphere C projects.

### Prerequisites

* Azure Sphere SDK (2+Beta1905) installed to default path (`C:\Program Files (x86)\Microsoft Azure Sphere SDK`)

* In `%userprofile%\.cargo\config` add following lines for **armv7-unknown-linux-musleabihf** target

        [target.armv7-unknown-linux-musleabihf]
        linker = "gcc"
        # uncomment this to not add to cargo command each time
        # rustflags = ["-C", "link-args=-fno-use-linker-plugin"]

    **Note**: alternatively you can create symbolic link `cc` to your `gcc.exe` in `C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\2+Beta1905\tools\gcc`

### Project structure

* **rust-sphere-hello.sln** consists of
  * single **rust-sphere-hello.vcxproj** that is linked using static library output of
  * Rust library in `rust` folder, that is importing
    * another sample Rust library `other-lib-common` (the one that can be replaced by your library or some other crate etc)
