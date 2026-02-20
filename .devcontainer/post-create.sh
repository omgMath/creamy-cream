apt update -y
apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  clang \
  cmake \
  wget \
  file \
  npm \
  sqlite3 \
  libxdo-dev \
  libssl-dev \
  libnlopt-dev \
  libayatana-appindicator3-dev \
  libsqlite3-dev \
  pkg-config \
  librsvg2-dev -y

rustup toolchain install stable
rustup component add rustfmt
rustup target add wasm32-unknown-unknown
cargo install --locked prek
cargo install cargo-binstall --disable-telemetry --no-confirm
cargo binstall dioxus-cli --no-confirm
prek install
