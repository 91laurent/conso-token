' envoyer_pont_cache.vbs -- lanceur sans fenetre (15/10/2026).
' La tache « Conso Token feed » lancait envoyer_pont.cmd par cmd.exe (via WMI) : une console visible s'ouvrait
' (Windows Terminal) a chaque passage, toutes les 15 minutes. Ce lanceur fait la meme chose SANS fenetre :
' WScript.Shell.Run(..., 0, True) cache la console des le depart.
' Usage : wscript.exe //B //Nologo envoyer_pont_cache.vbs <pct5h> <reset5h_unix> <pct7j> <reset7j_unix>
' Chaque argument doit etre un entier (chiffres seulement), verifie ICI avant que cmd ne lise la ligne.
Option Explicit
Dim sh, fso, dossier, ligne, i, re, rc, debut
If WScript.Arguments.Count <> 4 Then WScript.Quit 3
Set re = New RegExp
re.Pattern = "^[0-9]+$"
' Fenetre de 5 h absente (31/10/2026, plan sans limite de 5 h) : les deux premiers arguments valent "-" et "-" (les deux ou aucun).
debut = 0
If WScript.Arguments(0) = "-" Then
  If WScript.Arguments(1) <> "-" Then WScript.Quit 3
  debut = 2
End If
For i = debut To 3
  If Not re.Test(WScript.Arguments(i)) Then WScript.Quit 3
Next
Set sh = CreateObject("WScript.Shell")
Set fso = CreateObject("Scripting.FileSystemObject")
dossier = fso.GetParentFolderName(WScript.ScriptFullName)
ligne = "cmd.exe /c """"" & dossier & "\envoyer_pont.cmd"""
For i = 0 To 3
  ligne = ligne & " " & WScript.Arguments(i)
Next
ligne = ligne & """"
rc = sh.Run(ligne, 0, True)
WScript.Quit rc
