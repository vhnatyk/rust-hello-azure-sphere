REM /*
REM 	Rust hello from C for Azure Sphere
REM
REM 	Copyright 2019 by Vitaliy Hnatyk @ Eruptiq
REM	
REM 	@license CC BY 4.0+ <https://creativecommons.org/licenses/by/4.0/legalcode.txt>
REM */

REM
REM In 
REM 
REM %userprofile%\.cargo\config
REM 
REM add following lines
REM 
REM [target.armv7-unknown-linux-musleabihf]
REM linker = "gcc"
REM # uncomment this to not add to cargo command each time
REM # rustflags = ["-C", "link-args=-fno-use-linker-plugin"]

@echo off
echo off
SET AZS_SDK=2+Beta1905
REM SET AZS_SDK=1+Beta1902
SET "azure_sdk_root_path=c:/program files (x86)/microsoft azure sphere sdk"

SET "SYSROOTS=%azure_sdk_root_path%/Sysroots"
SET "CURRENT_SYSROOT=%SYSROOTS%/%AZS_SDK%"

REM CPATH
SET "C_INCLUDE_PATH=%CURRENT_SYSROOT%/usr/include"
REM echo %C_INCLUDE_PATH%
REM CPLUS_INCLUDE_PATH
REM OBJC_INCLUDE_PATH


SET AZSPHERETOOLS=%azure_sdk_root_path%\Tools\
SET SYSROOT=%SYSROOTS%/%AZS_SDK%
SET SYSROOT_TOOLS=%AZSPHERETOOLS%
SET CCPATH=%SYSROOT_TOOLS%\gcc
SET PATH=%AZSPHERETOOLS%;%CCPATH%;%PATH%;
SET app_root=target\%target%\approot
SET out_path=%app_root%\bin

SET binname=bn1
SET libname=rust_hello
SET config=%1
if "%config%"=="" SET config=debug
REM SET config=release
REM SET config=debug

REM SET target=arm-v7-none-eabi
SET target=armv7-unknown-linux-musleabihf
REM SET target=armv7-unknown-linux-gnueabihf
SET PATH=%userprofile%\.cargo\bin;%PATH%
