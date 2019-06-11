

rem 1367040
cargo rustc --release --bin bn1 -- -C opt-level=z

rem 1492992
REM cargo rustc --release --bin bn1 -- -C lto -C link-args=-static

set "filename=target\release\bn1.exe"

D:\msys64_\mingw64\bin\strip.exe %filename%


for %%A in (%filename%) do echo.Size of "%%A" is %%~zA bytes