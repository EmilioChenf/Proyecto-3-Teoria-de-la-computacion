$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')
New-Item -ItemType Directory -Force target/verificacion | Out-Null
function Comprobar-Codigo {
    if ($LASTEXITCODE -ne 0) { throw "Falló la verificación anterior: código $LASTEXITCODE" }
}
cargo fmt --check *> target/verificacion/formato.txt
Comprobar-Codigo
cargo clippy --all-targets --all-features -- -D warnings *> target/verificacion/clippy.txt
Comprobar-Codigo
cargo test --locked *> target/verificacion/pruebas.txt
Comprobar-Codigo
cargo build --release --locked *> target/verificacion/compilacion.txt
Comprobar-Codigo
& .\target\release\simulador-mt.exe maquinas/reconocedora.yaml --traza target/verificacion/reconocedora.txt | Out-Null
Comprobar-Codigo
& .\target\release\simulador-mt.exe maquinas/alteradora.yaml --traza target/verificacion/alteradora.txt | Out-Null
Comprobar-Codigo
& .\target\release\simulador-mt.exe maquinas/reconocedora.yaml --entrada abc --limite 1 --traza target/verificacion/limite.txt | Out-Null
Comprobar-Codigo
rustc --version > target/verificacion/herramientas.txt
Comprobar-Codigo
cargo --version >> target/verificacion/herramientas.txt
Comprobar-Codigo
Write-Output 'Las cuatro verificaciones y las tres ejecuciones finalizaron correctamente.'
