@echo off
REM setlocal

rd /S /Q %out_path%
mkdir %out_path%
copy target\%target%\%config%\%binName% %out_path%\app

"%CCPATH%\strip" %out_path%\app

copy app_manifest.json %app_root%

azsphere image package-application --input %app_root% --output target\manual.imagepackage --sysroot %AZS_SDK% -v

echo Now do this:
echo azsphere device sideload delete
echo azsphere device sideload deploy -p target\manual.imagepackage
echo.

