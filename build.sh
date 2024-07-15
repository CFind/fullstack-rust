#!/bin/bash

if cargo build --release; then
    echo 'Rust build successful'
else    
    echo 'Rust build failed'
    exit 1
fi

