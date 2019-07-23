REM /*
REM 	Rust hello from C for Azure Sphere
REM
REM 	Copyright 2019 by Vitaliy Hnatyk @ Eruptiq
REM	
REM 	@license CC BY 4.0+ <https://creativecommons.org/licenses/by/4.0/legalcode.txt>
REM */

call setenv %1

rem verbosity level default
set verb_lev=
REM set "verb_lev=--verbose --verbose --verbose" 

rem show trace if crashes
set RUST_BACKTRACE=1 

REM set action=test
REM set action=bench
REM set action=build --lib
set action=--lib

set build_tool=cargo
REM set build_tool=xargo


if /I "%config%"=="release" (set configp=--release) else (set configp=)

rem cleanup
del /Q target\%target%\%config%\lib%libname%.*

rem command itself
set "comm=%build_tool% rustc %action% --target=%target% %configp% %verb_lev% -- -C link-args=-fno-use-linker-plugin"
echo %comm%
%comm%

IF %ERRORLEVEL% NEQ 0 (
    @echo "build command execution failed."
) else (
    @echo "build command execution success."
    rd /S /Q target\out\%config%\
    mkdir target\out\%config%\
    copy target\%target%\%config%\lib%libname%.* target\out\%config%\
)