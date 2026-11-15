#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p evidencias
cargo fmt --check > evidencias/formato.txt 2>&1
cargo clippy --all-targets --all-features -- -D warnings > evidencias/clippy.txt 2>&1
cargo test --locked > evidencias/pruebas.txt 2>&1
cargo build --release --locked > evidencias/compilacion.txt 2>&1
./target/release/simulador-mt maquinas/reconocedora.yaml --traza evidencias/reconocedora.txt > /dev/null
./target/release/simulador-mt maquinas/alteradora.yaml --traza evidencias/alteradora.txt > /dev/null
./target/release/simulador-mt maquinas/reconocedora.yaml --entrada abc --limite 1 --traza evidencias/limite.txt > /dev/null
rustc --version > evidencias/herramientas.txt
cargo --version >> evidencias/herramientas.txt
printf '%s\n' 'Las cuatro verificaciones y las tres ejecuciones finalizaron correctamente.'
