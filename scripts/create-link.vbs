Set oWS = WScript.CreateObject("WScript.Shell")
sLinkFile = oWS.ExpandEnvironmentStrings("%AppData%\Microsoft\Windows\Start Menu\Programs\Startup\hwc.lnk")
Set oLink = oWS.CreateShortcut(sLinkFile)
oLink.TargetPath = oWS.CurrentDirectory & "\hwc.exe"
oLink.WorkingDirectory = oWS.CurrentDirectory
oLink.Description = "change wallpaper on hot-keys"
oLink.Save