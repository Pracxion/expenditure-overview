#!/bin/bash

if ! command -v cargo &> /dev/null; then
    echo "Cargo not found. Installing Rust and Cargo .."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
else
    echo "Cargo is already installed."
fi

if ! command -v sqlx &> /dev/null; then
    echo "Sqlx not found. Installing Sqlx .."
    cargo install sqlx-cli
else
    echo "Sqlx is already installed."
fi 

sqlx db create
sqlx migrate run