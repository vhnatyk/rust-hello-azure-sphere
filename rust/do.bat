REM /*
REM 	Rust hello from C for Azure Sphere
REM
REM 	Copyright 2019 by Vitaliy Hnatyk @ Eruptiq
REM	
REM 	@license CC BY 4.0+ <https://creativecommons.org/licenses/by/4.0/legalcode.txt>
REM */

rem sample to control output file size
rem 1367040
cargo rustc --release --bin bn1 -- -C opt-level=z

rem 1492992
REM cargo rustc --release --bin bn1 -- -C lto -C link-args=-static

set "filename=target\release\bn1.exe"

strip %filename%

for %%A in (%filename%) do echo.Size of "%%A" is %%~zA bytes