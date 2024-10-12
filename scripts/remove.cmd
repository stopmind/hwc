@echo off

del "%AppData%\Microsoft\Windows\Start Menu\Programs\Startup\hwc.lnk"
taskkill /f /t /im hwc.exe

pause