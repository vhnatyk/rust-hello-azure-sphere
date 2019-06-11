@echo off
setlocal
set AZSPHERETOOLS=c:\Program Files (x86)\Microsoft Azure Sphere SDK\Tools\
set PATH=%PATH%;%AZSPHERETOOLS%

azsphere device sideload delete
REM azsphere device sideload deploy -p target\manual.imagepackage  --force
azsphere device sideload deploy -p target\manual.imagepackage
IF %ERRORLEVEL% NEQ 0 (
  echo sideload deploy command execution failed.
)
