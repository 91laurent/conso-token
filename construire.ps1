# construire.ps1 : compile Conso Token sans laisser les chemins du poste dans le programme.
#
# Usage (depuis le dossier des sources) :
#   pwsh -File construire.ps1 -Cible <dossier de compilation COURT> [-Action build|test|test-release]
#
# - Le dossier de compilation doit être court (un chemin de plus de 260 caractères fait échouer l'éditeur de liens).
# - Ce script calcule les chemins À REMPLACER à partir de la machine où il tourne (profil de l'utilisateur,
#   dossiers de Cargo et de Rust, dossier des sources, dossier de compilation) : il ne contient aucun chemin.
# - Il règle les options de compilation par la variable CARGO_ENCODED_RUSTFLAGS, qui l'emporte sur
#   .cargo\config.toml : la ligne « crt-static » est donc répétée ici.
param(
    [Parameter(Mandatory)][string]$Cible,
    [ValidateSet('build', 'test', 'test-release')][string]$Action = 'build'
)
$ErrorActionPreference = 'Stop'
$sources = $PSScriptRoot
$cible = [IO.Path]::GetFullPath($Cible)
if ($cible.Length -gt 60) { Write-Warning "Dossier de compilation long ($($cible.Length) caractères) : risque d'échec de l'éditeur de liens." }

# Du plus général au plus précis : quand deux préfixes conviennent, le DERNIER l'emporte.
$prefixes = New-Object System.Collections.Generic.List[string]
function Ajouter([string]$chemin, [string]$nom, [switch]$MemeAbsent) {
    if ($chemin -and ($MemeAbsent -or (Test-Path -LiteralPath $chemin))) {
        $c = [IO.Path]::GetFullPath($chemin).TrimEnd('\')
        if ($c.Length -gt 3) { $prefixes.Add("--remap-path-prefix=$c=$nom") }
    }
}
Ajouter $env:USERPROFILE 'utilisateur'
Ajouter $env:CARGO_HOME 'cargo'
Ajouter $env:RUSTUP_HOME 'rustup'
Ajouter (Split-Path $sources -Parent) 'dossier-parent'
Ajouter $sources 'projet'
Ajouter (Split-Path $cible -Parent) 'construction' -MemeAbsent
Ajouter $cible 'cible' -MemeAbsent

$drapeaux = @('-C', 'target-feature=+crt-static') + $prefixes
$env:CARGO_ENCODED_RUSTFLAGS = ($drapeaux -join [char]0x1f)
Write-Host "Chemins remplacés :"; $prefixes | ForEach-Object { Write-Host "  $_" }

Push-Location $sources
try {
    switch ($Action) {
        'build'        { cargo build --release --offline --locked -p conso_token --target-dir $cible }
        'test'         { cargo test --offline --locked -p noyau --target-dir $cible }
        'test-release' { cargo test --release --offline --locked -p conso_token --target-dir $cible }
    }
    if ($LASTEXITCODE -ne 0) { throw "cargo a échoué (code $LASTEXITCODE)" }
} finally { Pop-Location }
