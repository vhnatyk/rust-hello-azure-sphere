

cargo build --release
rem 4252858
REM cargo rustc --release -- -C opt-level=z -C prefer-dynamic
REM rem 4252858
REM cargo rustc --release -- -C opt-level=z



rem 1492992
REM cargo rustc --release --bin bn1 -- -C lto -C link-args=-static

set "filename=target\release\bn1.exe"

D:\msys64_\mingw64\bin\strip.exe %filename%


for %%A in (%filename%) do echo.Size of "%%A" is %%~zA bytes