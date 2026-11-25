@echo off
rem Usage : envoyer_pont.cmd <pct5h> <reset5h_unix> <pct7j> <reset7j_unix>   (quatre nombres entiers)
rem Fenetre de 5 h absente (plan sans limite de 5 h) : envoyer_pont.cmd - - <pct7j> <reset7j_unix>  (deux tirets a la place du couple 5 h ; la fenetre 5 h n'est alors pas envoyee, rien n'est invente).
rem Envoie au pont (conso_token.exe --pont) les chiffres du forfait, au format de la ligne d'etat de Claude Code.
rem Version GENERALE (1.3.7) : aucun chemin ni nom d'utilisateur ecrit ici.
rem - le programme est cherche dans le MEME dossier que ce fichier, sinon dans le dossier PARENT (ex. C:\ConsoToken\outils\ -> C:\ConsoToken\conso_token.exe) ;
rem - le dossier de donnees est %USERPROFILE%\AppData\Local\ConsoToken, le meme que celui lu par la fenetre.
rem   (l'appli Claude est un paquet Windows : AppData\Local y est redirige, pas USERPROFILE ; ne pas utiliser LOCALAPPDATA ici).
rem - le resultat de chaque appel (dernier_envoi.txt) et le fichier de passage (entree.json) sont ecrits dans le DOSSIER DE DONNEES,
rem   jamais a cote du programme : le programme peut donc etre dans un dossier ou l'utilisateur n'a pas le droit d'ecrire.
rem Chaque argument est verifie SEPAREMENT (non vide, chiffres seulement) avant d'etre utilise.
setlocal
set "CONSO_TOKEN_DATA=%USERPROFILE%\AppData\Local\ConsoToken"
if not exist "%CONSO_TOKEN_DATA%\" mkdir "%CONSO_TOKEN_DATA%" 2>nul
set "SORTIE=%CONSO_TOKEN_DATA%\dernier_envoi.txt"
set "ENTREE=%CONSO_TOKEN_DATA%\entree.json"
set "CINQ="
if "%~1"=="-" goto :sans_cinq
call :chiffres "%~1" || goto :refus
call :chiffres "%~2" || goto :refus
set CINQ="five_hour":{"used_percentage":%~1,"resets_at":%~2},
goto :suite
:sans_cinq
if not "%~2"=="-" goto :refus
:suite
call :chiffres "%~3" || goto :refus
call :chiffres "%~4" || goto :refus
set "PROGRAMME="
if exist "%~dp0conso_token.exe" set "PROGRAMME=%~dp0conso_token.exe"
if not defined PROGRAMME if exist "%~dp0..\conso_token.exe" set "PROGRAMME=%~dp0..\conso_token.exe"
if not defined PROGRAMME goto :sans_programme
> "%ENTREE%" echo {"rate_limits":{%CINQ%"seven_day":{"used_percentage":%~3,"resets_at":%~4}}}
"%PROGRAMME%" --pont < "%ENTREE%"
set "RC=%ERRORLEVEL%"
> "%SORTIE%" echo %DATE% %TIME% envoye 5h=%~1 7j=%~3 rc=%RC% donnees=%CONSO_TOKEN_DATA%
exit /b %RC%

:refus
> "%SORTIE%" echo %DATE% %TIME% REFUSE argument invalide
exit /b 3

:sans_programme
> "%SORTIE%" echo %DATE% %TIME% REFUSE conso_token.exe introuvable (ni a cote de ce fichier, ni dans le dossier parent)
exit /b 4

:chiffres
if "%~1"=="" exit /b 1
for /f "delims=0123456789" %%Z in ("%~1") do exit /b 1
exit /b 0
