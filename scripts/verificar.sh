#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p target/verificacion
cargo fmt --check > target/verificacion/formato.txt 2>&1
cargo clippy --all-targets --all-features -- -D warnings > target/verificacion/clippy.txt 2>&1
cargo test --locked > target/verificacion/pruebas.txt 2>&1
cargo build --release --locked > target/verificacion/compilacion.txt 2>&1
./target/release/simulador-mt maquinas/reconocedora.yaml --traza target/verificacion/reconocedora.txt > /dev/null
./target/release/simulador-mt maquinas/alteradora.yaml --traza target/verificacion/alteradora.txt > /dev/null
./target/release/simulador-mt maquinas/reconocedora.yaml --entrada abc --limite 1 --traza target/verificacion/limite.txt > /dev/null
rustc --version > target/verificacion/herramientas.txt
cargo --version >> target/verificacion/herramientas.txt
printf '%s\n' 'Las cuatro verificaciones y las tres ejecuciones finalizaron correctamente.'
