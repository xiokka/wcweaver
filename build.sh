echo "Building for Linux (x86_64-unknown-linux-gnu)..."
cargo build --release --target x86_64-unknown-linux-gnu

echo "Building for Windows (x86_64-pc-windows-gnu)..."
cargo build --release --target x86_64-pc-windows-gnu
