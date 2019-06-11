IF %ERRORLEVEL% NEQ 0 (
  goto:eof
)
set ERRORLEVEL=0
@echo off
setlocal
set AZSPHERETOOLS=c:/Program Files (x86)/Microsoft Azure Sphere SDK/Tools/
set PATH=%PATH%;%AZSPHERETOOLS%

azsphere device sideload stop -i ccc29b2a-1ac1-43cc-a734-00770b9789b9
azsphere device sideload start -d -i ccc29b2a-1ac1-43cc-a734-00770b9789b9
IF %ERRORLEVEL% NEQ 0 (
  echo "sideload start command execution failed."
) else (
    start telnet 192.168.35.2 2342

    echo try this:
    echo first connect:
    echo target remote 192.168.35.2:2345
    echo.
    echo run the application:
    echo c
    echo.

    "%CURRENT_SYSROOT%/tools/gcc/arm-poky-linux-musleabi-gdb.exe" target/%target%/%config%/%binName%
)