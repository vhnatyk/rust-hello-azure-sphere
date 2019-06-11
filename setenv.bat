@echo off
echo off
SET AZS_SDK=2+Beta1905
REM SET AZS_SDK=1+Beta1902
set "azure_sdk_root_path=c:/program files (x86)/microsoft azure sphere sdk"

set "SYSROOTS=%azure_sdk_root_path%/Sysroots"
set "CURRENT_SYSROOT=%SYSROOTS%/%AZS_SDK%"

REM CPATH
SET "C_INCLUDE_PATH=%CURRENT_SYSROOT%/usr/include"
REM echo %C_INCLUDE_PATH%
REM CPLUS_INCLUDE_PATH
REM OBJC_INCLUDE_PATH


set AZSPHERETOOLS=c:\Program Files (x86)\Microsoft Azure Sphere SDK\Tools\
set SYSROOT=C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\%AZS_SDK%
set SYSROOT_TOOLS=%SYSROOT%\tools
set CCPATH=%SYSROOT_TOOLS%\gcc
set PATH=%PATH%;%AZSPHERETOOLS%;%CCPATH%
set app_root=target\%target%\approot
set out_path=%app_root%\bin

set RUST_TARGET_PATH=%cd%
REM set gcc=gcc_wrapper

set binname=bn1
set libname=rust_hello
REM set config=release
set config=debug

REM set target=arm-v7-none-eabi
set target=armv7-unknown-linux-musleabihf
REM set target=armv7-unknown-linux-gnueabihf
set PATH=C:\Users\utech\.cargo\bin;%PATH%
