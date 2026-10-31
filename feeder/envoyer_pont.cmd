@echo off
rem Usage : envoyer_pont.cmd <pct5h> <reset5h_unix> <pct7j> <reset7j_unix>   (quatre nombres entiers)
rem Fenetre de 5 h absente (plan sans limite de 5 h, ex. Ultra, 31/10/2026) : envoyer_pont.cmd - - <pct7j> <reset7j_unix>  (deux tirets a la place du couple 5 h ; la fenetre 5 h n'est alors pas envoyee, rien n'est invente).
rem Envoie au pont (conso_token.exe --pont) les chiffres du forfait, au format de la ligne d'etat de Claude Code.
rem Version GENERALE : aucun chemin ni nom d'utilisateur ecrit ici.
rem - le programme est cherche dans le dossier PARENT de outils\ (ex. C:\ConsoToken\conso_token.exe) ;
rem - le dossier de donnees est %USERPROFILE%\AppData\Local\ConsoToken, le meme que celui lu par la fenetre.
rem   (l'appli Claude est un paquet Windows : AppData\Local y est redirige, pas USERPROFILE ; ne pas utiliser LOCALAPPDATA ici).
rem Chaque argument est verifie SEPAREMENT (non vide, chiffres seulement) avant d'etre utilise ; le resultat de chaque appel est note dans dernier_envoi.txt.
setlocal
set "SORTIE=%~dp0dernier_envoi.txt"
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
if not exist "%~dp0..\conso_token.exe" goto :sans_programme
set "CONSO_TOKEN_DATA=%USERPROFILE%\AppData\Local\ConsoToken"
> "%~dp0entree.json" echo {"rate_limits":{%CINQ%"seven_day":{"used_percentage":%~3,"resets_at":%~4}}}
"%~dp0..\conso_token.exe" --pont < "%~dp0entree.json"
set "RC=%ERRORLEVEL%"
> "%SORTIE%" echo %DATE% %TIME% envoye 5h=%~1 7j=%~3 rc=%RC% donnees=%CONSO_TOKEN_DATA%
exit /b %RC%

:refus
> "%SORTIE%" echo %DATE% %TIME% REFUSE argument invalide
exit /b 3

:sans_programme
> "%SORTIE%" echo %DATE% %TIME% REFUSE conso_token.exe introuvable dans le dossier parent de outils
exit /b 4

:chiffres
if "%~1"=="" exit /b 1
for /f "delims=0123456789" %%Z in ("%~1") do exit /b 1
exit /b 0
