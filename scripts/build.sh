#!/usr/bin/env bash
# Builds a self-contained, production-ready bundle into ./dist:
# release binary + minified site assets (JS/WASM/CSS) + public/ + uploads/ + .env
# + nginx site config + systemd unit + install.sh (server-side setup script).
#
# Usage: ./scripts/build.sh
# Deploy: rsync -avz --exclude 'uploads' dist/ root@VPS:/opt/feri-irawansyah/
#         then: ssh root@VPS 'cd /opt/feri-irawansyah && sudo bash install.sh'

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BIN_NAME="feri-irawansyah"
DIST_DIR="$ROOT_DIR/dist"

echo "==> Building release bundle (cargo-leptos minifies CSS and runs wasm-opt on the WASM automatically in release mode)"
cargo leptos build --release

echo "==> Resetting dist/"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

echo "==> Copying server binary"
cp "target/release/$BIN_NAME" "$DIST_DIR/$BIN_NAME"
chmod +x "$DIST_DIR/$BIN_NAME"

echo "==> Copying site assets (target/site -> dist/site)"
cp -r target/site "$DIST_DIR/site"

echo "==> Copying public/ and preparing uploads/"
cp -r public "$DIST_DIR/public"
cp scripts/templates/404.html "$DIST_DIR/public/404.html"
cp scripts/templates/429.html "$DIST_DIR/public/429.html"
cp scripts/templates/502.html "$DIST_DIR/public/502.html"
mkdir -p "$DIST_DIR/uploads"

echo "==> Precompressing static assets (brotli, for nginx's brotli_static)"
# actix's middleware::Compress already compresses on the fly for anything
# proxied to the app, but nginx serves /pkg and /public directly off disk
# (see nginx.conf) — brotli_static there needs a sibling *.br next to each
# file, not on-the-fly compression, so we produce them here at build time.
COMPRESSIBLE=(-name '*.js' -o -name '*.css' -o -name '*.wasm' -o -name '*.svg'
              -o -name '*.json' -o -name '*.xml' -o -name '*.txt')

if command -v brotli >/dev/null 2>&1; then
    find "$DIST_DIR/site" "$DIST_DIR/public" -type f \( "${COMPRESSIBLE[@]}" \) \
        -exec brotli -q 11 -k -f {} \;
else
    echo "!! brotli not found — skipping .br precompression (apt install brotli), nginx's brotli_static will just miss" >&2
fi

echo "==> Bundling nginx site, systemd unit, valkey config, and server-side install script"
mkdir -p "$DIST_DIR/nginx" "$DIST_DIR/systemd" "$DIST_DIR/valkey"
cp scripts/templates/nginx.conf "$DIST_DIR/nginx/feri-irawansyah.conf"
cp scripts/templates/feri-irawansyah.service "$DIST_DIR/systemd/feri-irawansyah.service"
cp scripts/templates/valkey.conf "$DIST_DIR/valkey/valkey.conf"
cp scripts/templates/install.sh "$DIST_DIR/install.sh"
chmod +x "$DIST_DIR/install.sh"

echo "==> Writing dist/.env"
if [ -f .env ]; then
    cp .env "$DIST_DIR/.env"
else
    echo "!! No .env at project root — dist/.env will only have the runtime defaults below." >&2
    touch "$DIST_DIR/.env"
fi
{
    echo ""
    echo "# --- appended by scripts/build-dist.sh for the production runtime ---"
    echo "LEPTOS_SITE_ROOT=site"
    echo "LEPTOS_SITE_PKG_DIR=pkg"
    # 127.0.0.1, not 0.0.0.0 — this is meant to sit behind a reverse proxy
    # (Nginx etc.) on the same machine, not be reachable directly from outside.
    echo "LEPTOS_SITE_ADDR=127.0.0.1:3000"
    echo "LEPTOS_ENV=PROD"
} >> "$DIST_DIR/.env"

echo ""
echo "==> Done:"
du -sh "$DIST_DIR"/* | sed "s|$DIST_DIR/||"
echo ""
echo "Run it locally with:"
echo "  cd dist && ./$BIN_NAME"
echo ""
echo "Deploy to the VPS:"
echo "  rsync -avz --exclude 'uploads' dist/ root@VPS_IP:/opt/feri-irawansyah/"
echo "  ssh root@VPS_IP 'cd /opt/feri-irawansyah && sudo bash install.sh'"
