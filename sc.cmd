set "azure_sdk_root_path=c:\program files (x86)\microsoft azure sphere sdk"

set "SYSROOTS=%azure_sdk_root_path%\Sysroots"
set "CURRENT_SYSROOT=%SYSROOTS%\1+Beta1902"

REM CPATH
SET "C_INCLUDE_PATH=%CURRENT_SYSROOT%\usr\include"
echo %C_INCLUDE_PATH%
REM CPLUS_INCLUDE_PATH
REM OBJC_INCLUDE_PATH


call "%azure_sdk_root_path%\initializecommandprompt.cmd"