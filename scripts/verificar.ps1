$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')
New-Item -ItemType Directory -Force evidencias | Out-Null
function Comprobar-Codigo {
    if ($LASTEXITCODE -ne 0) { throw "Falló la verificación anterior: código $LASTEXITCODE" }
}
cargo fmt --check *> evidencias/formato.txt
Comprobar-Codigo
cargo clippy --all-targets --all-features -- -D warnings *> evidencias/clippy.txt
Comprobar-Codigo
cargo test --locked *> evidencias/pruebas.txt
Comprobar-Codigo
cargo build --release --locked *> evidencias/compilacion.txt
Comprobar-Codigo
& .\target\release\simulador-mt.exe maquinas/reconocedora.yaml --traza evidencias/reconocedora.txt | Out-Null
Comprobar-Codigo
& .\target\release\simulador-mt.exe maquinas/alteradora.yaml --traza evidencias/alteradora.txt | Out-Null
Comprobar-Codigo
& .\target\release\simulador-mt.exe maquinas/reconocedora.yaml --entrada abc --limite 1 --traza evidencias/limite.txt | Out-Null
Comprobar-Codigo
rustc --version > evidencias/herramientas.txt
Comprobar-Codigo
cargo --version >> evidencias/herramientas.txt
Comprobar-Codigo
Write-Output 'Las cuatro verificaciones y las tres ejecuciones finalizaron correctamente.'
