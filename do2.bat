call setenv

REM set "AZURE_SDK_PATH=C:\Program Files (x86)\Microsoft Azure Sphere SDK"
REM call "%AZURE_SDK_PATH%\InitializeCommandPrompt.cmd"
REM echo %deb_ug%
REM doskey "LLVM32=C:\Progra~2\LLVM"
REM doskey "LLVM_BIN=%LLVM32%\bin"
REM doskey "PATH=%PATH%;%LLVM_BIN%""

REM doskey "CC=%LLVM_BIN%\clang.exe"
REM doskey "CXX=%LLVM_BIN%\clang++.exe"
REM doskey "AR=%LLVM_BIN%\llvm-ar.exe"

REM doskey cc=clang $*
REM doskey cxx=clang++ $*
REM doskey ar=llvm-ar $*

REM doskey CC=cl $*
REM doskey CXX=clang++ $*
REM doskey AR=llvm-ar $*

REM doskey cc=gcc $*
REM doskey cxx=g++ $*
REM REM doskey ar=ar

rem echo cc and %cc%

REM arch_lib_dir="/usr/lib/$trgt/$api_num$compil_type"

REM set SYSROOT="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$host_var/sysroot$arch_lib_dir"
REM set AARCH64_LINUX_ANDROID_OPENSSL_LIB_DIR=$SYSROOT
REM set OPENSSL_DIR="/usr/" #depends on your OS and OpenSSL setup

REM ## setting some additional vars may be required
REM # set PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig"
REM # set PATH="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$host_var/bin":$PATH
REM set LIBRARY_PATH=$LIBRARY_PATH:"/usr/lib/x86_64-linux-gnu:/usr/local/lib:$SYSROOT"

REM for C libs compilation
set "TARGET_CFLAGS=-march=armv7ve -mcpu=cortex-a7 -mthumb -mfpu=neon -mfloat-abi=hard"

rem rem verbosity level default
set verb_lev=
REM set "verb_lev=--verbose --verbose --verbose" 

rem show trace if crashes
set RUST_BACKTRACE=1 

REM set action=test
REM set action=bench
set action=build

set pargo=cargo
REM set pargo=xargo

if %config%=="release" set configp=--%config%


rem cleanup
del /Q target\%target%\%config%\lib%libname%.*

rem command itself
REM cargo %action% --%config% --target wasm32-unknown-unknown %verb_lev% 
REM xargo build --target armv7-linux-eabi %verb_lev%
REM xargo build --%config% --target arm-v7-none-eabi %verb_lev%
%pargo% %action% --target=%target% %configp% %verb_lev%

rem C:\WINDOWS\system32\cmd.exe /k "C:\Program Files\Emscripten\emsdk_env.bat"

IF %ERRORLEVEL% NEQ 0 (
    @echo "build command execution failed."
) else (
    @echo "build command execution success."
    rd /S /Q target\out\%config%\
    mkdir target\out\%config%\
    copy target\%target%\%config%\lib%libname%.* target\out\%config%\
)

REM call package
REM IF %ERRORLEVEL% NEQ 0 (
REM   echo "package command execution failed."
REM ) else ( 
REM     call deploy
REM         IF %ERRORLEVEL% NEQ 0 (
REM         echo "deploy command execution failed."
REM     ) else (
REM         call debug
REM         IF %ERRORLEVEL% NEQ 0 (
REM         echo "debug command execution failed."
REM     )))
