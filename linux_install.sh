#!/usr/bin/bash
echo cleaning...
cargo clean
echo building...
cargo build --release


printf "\n# portscraper path\n" >> ~/.bashrc
echo export PATH=\"$(pwd)/target/release:\$PATH\" >> ~/.bashrc

echo done :]
exec "$SHELL"
