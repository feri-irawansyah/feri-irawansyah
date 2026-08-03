#!/usr/bin/env bash
# One-shot (and safe to re-run) server-side setup: system user, ownership,
# systemd service, nginx site. Bundled into dist/ by scripts/build.sh.
#
# Run this ON THE VPS, with sudo, from *inside* the deployed folder — i.e.
# after `dist/` has already been rsynced to /opt/feri-irawansyah:
#
#   sudo bash install.sh
#
# Safe to re-run after every redeploy: it only creates the user/service/site
# if missing, and otherwise just refreshes ownership + reloads.

set -euo pipefail

APP_DIR="/opt/feri-irawansyah"
APP_USER="feriapp"
DOMAIN="feri-irawansyah.my.id"

if [ "$(id -u)" -ne 0 ]; then
    echo "Run this with sudo (needs to write to /etc/systemd and /etc/nginx)." >&2
    exit 1
fi

if [ "$(pwd)" != "$APP_DIR" ]; then
    echo "!! Expected to run from $APP_DIR — this script assumes dist/ has already" >&2
    echo "!! been rsynced there. Current dir: $(pwd)" >&2
fi

echo "==> Ensuring nginx is installed"
command -v nginx >/dev/null || { apt-get update && apt-get install -y nginx; }

echo "==> Ensuring system user '$APP_USER' exists"
id -u "$APP_USER" >/dev/null 2>&1 || useradd --system --no-create-home --shell /usr/sbin/nologin "$APP_USER"

echo "==> Fixing ownership/permissions"
mkdir -p "$APP_DIR/uploads"
chown -R "$APP_USER:$APP_USER" "$APP_DIR"
chmod 600 "$APP_DIR/.env"
chmod +x "$APP_DIR/feri-irawansyah"

echo "==> Installing systemd service"
cp "$APP_DIR/systemd/feri-irawansyah.service" /etc/systemd/system/feri-irawansyah.service
systemctl daemon-reload
systemctl enable feri-irawansyah
systemctl restart feri-irawansyah

echo "==> Installing nginx site"
NGINX_DEST="/etc/nginx/sites-available/$DOMAIN"
if [ -f "$NGINX_DEST" ] && grep -q "listen 443" "$NGINX_DEST"; then
    echo "!! $NGINX_DEST already has HTTPS configured (certbot edits this file in"
    echo "!! place to add its 'listen 443 ssl' block). NOT overwriting it automatically"
    echo "!! — that would silently drop the HTTPS config on this redeploy."
    echo "!! If you changed scripts/templates/nginx.conf, merge the changes into"
    echo "!! $NGINX_DEST by hand, then run: nginx -t && systemctl reload nginx"
else
    cp "$APP_DIR/nginx/feri-irawansyah.conf" "$NGINX_DEST"
    ln -sf "$NGINX_DEST" "/etc/nginx/sites-enabled/$DOMAIN"
    nginx -t
    systemctl reload nginx
fi

echo ""
echo "==> Done. Service status:"
systemctl --no-pager --lines=5 status feri-irawansyah || true
echo ""
echo "First time only — HTTPS isn't set up automatically. Run:"
echo "  apt install -y certbot python3-certbot-nginx"
echo "  certbot --nginx -d $DOMAIN -d www.$DOMAIN"
