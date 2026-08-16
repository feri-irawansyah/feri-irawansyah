# Deploy ke VPS (Debian 13)

Panduan ini asumsi:

- Build dilakukan **di lokal** (`bash scripts/build.sh`), lalu **seluruh isi folder `dist/` apa adanya** di-upload ke VPS. **Jangan compile di VPS** — VPS kentang ga cukup buat _menjalankan_ binary yang sudah jadi, tapi beresiko OOM kalau dipakai buat _compile_ (LTO + codegen-units=1 makan RAM banyak saat build).
- `dist/` sekarang self-contained: binary + site assets + `public/` + `.env` (siap pakai) + **config Nginx, unit systemd, dan `install.sh`** yang otomatis nyetel semuanya di server. Jadi di server tinggal jalanin 1 script, bukan ngetik config manual.
- Database pakai Postgres cloud (Aiven, lewat `DATABASE_URL` di `.env`) — jadi **tidak perlu install Postgres di VPS ini**.
- OpenSSL sudah ada di VPS.
- Domain (`feri-irawansyah.my.id`) A record-nya sudah diarahkan ke IP VPS. Ganti nama domain di `scripts/templates/nginx.conf` dan `scripts/templates/install.sh` kalau beda (di-bundle ulang ke `dist/` tiap `build.sh` jalan).
- App dijalankan sebagai user khusus non-root (`feriapp`) di `/opt/feri-irawansyah` — kalau ada apa-apa (bug/vuln), kerusakannya kebatasin ke folder itu doang, bukan akses root penuh ke server.

---

## 0. Setup awal VPS (sekali aja, pas baru dapet VPS)

Sebelum upload/build apa-apa, siapin dulu user & folder tujuannya:

```bash
ssh root@VPS_IP
useradd --system --no-create-home --shell /usr/sbin/nologin feriapp
mkdir -p /opt/feri-irawansyah
chown -R feriapp:feriapp /opt/feri-irawansyah
exit
```

Kenapa gini:

- **User `feriapp`** — service jalan sebagai user ini, bukan `root`. Kalau ada apa-apa (bug/vuln), kerusakannya kebatasin ke folder ini doang, bukan akses root penuh ke server. `--no-create-home --shell /usr/sbin/nologin` artinya user ini nggak bisa dipakai buat login/SSH, cuma buat jalanin proses.
- **`/opt/feri-irawansyah`**, bukan `~/feri-irawansyah` (home `root`) — `/root` defaultnya `chmod 700`, jadi user lain kayak `feriapp` nggak akan bisa akses folder itu sama sekali, walau file di dalamnya udah di-`chown`. `/opt` itu lokasi standar Linux (FHS) buat aplikasi self-contained kayak gini. Boleh diganti ke path lain (`/srv/...` misal) asal bukan di dalam `/root` — kalau diganti, sesuaikan juga `APP_DIR` di `scripts/templates/install.sh` dan `WorkingDirectory`/`ExecStart` di `scripts/templates/feri-irawansyah.service`.

## Setup pertama kali (build + upload + install)

```bash
# 1. Build di lokal
bash scripts/build.sh

# 2. Upload isi dist/ ke folder yang udah disiapin di langkah 0
rsync -av --progress --partial --exclude 'uploads' dist/ root@VPS_IP:/opt/feri-irawansyah/

# 3. Jalanin installer di server (nginx, systemd — sekali jalan)
ssh root@VPS_IP 'cd /opt/feri-irawansyah && sudo bash install.sh'
```

`install.sh` (isinya di-bundle dari `scripts/templates/install.sh`, ke-copy ke `dist/install.sh` tiap `build.sh` jalan) ngerjain ini, dan aman dijalanin berkali-kali:

- Install Nginx kalau belum ada.
- Bikin user sistem `feriapp` kalau belum ada (fallback — normalnya udah dibikin di langkah 0).
- `chown` seluruh `/opt/feri-irawansyah` ke `feriapp`, kunci permission `.env` (`600`) dan binary (`+x`).
- Copy `dist/systemd/feri-irawansyah.service` → `/etc/systemd/system/`, `daemon-reload`, `enable`, `restart`.
- Copy `dist/nginx/feri-irawansyah.conf` → `/etc/nginx/sites-available/`, symlink ke `sites-enabled/`, `nginx -t`, `reload`.

Setelah itu, dua langkah manual (sekali aja, nggak diotomatisin karena keduanya "beresiko kalau salah" — HTTPS ganggu domain, firewall bisa ngunci akses SSH sendiri):

### HTTPS (Let's Encrypt via certbot)

**Sebelum ini**: pastiin A record domain (`feri-irawansyah.my.id` dan `www`) di DNS provider (mis. idcloudhost) udah nunjuk ke IP VPS yang bener. Kalau baru pindah VPS, gampang lupa update ini — certbot bakal gagal dengan error `Timeout during connect` kalau DNS masih ngarah ke IP lama/salah.

```bash
ssh root@VPS_IP
apt install -y certbot python3-certbot-nginx
certbot --nginx -d feri-irawansyah.my.id -d www.feri-irawansyah.my.id
```

Cek auto-renew: `systemctl status certbot.timer`.

### Firewall

**Sebelum `ufw enable`** — cek dulu 3 hal ini, biar nggak keblokir sendiri dari SSH (kalau salah, harus dibenerin lewat console provider, bukan SSH lagi):

```bash
# 1. Pastiin port SSH yang bener-bener dipakai (kadang provider VPS ganti dari 22)
ss -tlnp | grep ssh
grep -i "^Port" /etc/ssh/sshd_config

# 2. Cek app & Nginx masih sehat
systemctl status feri-irawansyah
curl -I https://feri-irawansyah.my.id
ss -tlnp | grep -E ':80|:443'

# 3. Cek ufw sendiri udah keinstall belum (Debian minimal biasanya belum bawa)
which ufw || apt install -y ufw
ufw status verbose   # harusnya masih "inactive"
```

Kalau port SSH-nya bukan 22 (default), tambahin `ufw allow <port_itu>` juga sebelum `enable`. Kalau udah yakin:

```bash
ufw allow OpenSSH
ufw allow 'Nginx Full'   # buka 80 + 443
ufw enable
```

Port 3000 **tidak** perlu dibuka — app cuma listen di `127.0.0.1:3000` (lihat bagian `.env` di bawah), jadi cuma bisa diakses lewat Nginx.

---

## `dist/.env`

`scripts/build.sh` otomatis nambahin 4 baris ini ke `dist/.env` (nggak perlu diedit manual):

```env
LEPTOS_SITE_ROOT=site
LEPTOS_SITE_PKG_DIR=pkg
LEPTOS_SITE_ADDR=127.0.0.1:3000
LEPTOS_ENV=PROD
```

`LEPTOS_SITE_ADDR` sengaja `127.0.0.1` (bukan `0.0.0.0`) — biar app cuma bisa diakses dari server itu sendiri lewat Nginx, bukan langsung tembus lewat port 3000 dari luar.

(Kalau nanti bikin ulang `.env` dari nol / dari `.env` sumber project alih-alih dari `dist/.env` hasil build, ke-4 baris di atas harus ditambahin manual.)

---

## Redeploy (update versi baru)

```bash
bash scripts/build.sh
rsync -av --progress --partial --exclude 'uploads' dist/ root@VPS_IP:/opt/feri-irawansyah/
ssh root@VPS_IP 'cd /opt/feri-irawansyah && sudo bash install.sh'
```

`--exclude 'uploads'` penting dipakai terus, biar folder `uploads/` yang udah berisi file di server nggak ketimpa/kehapus. `install.sh` aman dijalanin lagi tiap redeploy — cuma refresh ownership + restart service + reload Nginx.

---

## Cheatsheet

| Perintah                                 | Kegunaan                                |
| ---------------------------------------- | --------------------------------------- |
| `sudo systemctl status feri-irawansyah`  | Cek status service                      |
| `sudo systemctl restart feri-irawansyah` | Restart manual (tanpa lewat install.sh) |
| `journalctl -u feri-irawansyah -f`       | Lihat log realtime                      |
| `sudo nginx -t`                          | Validasi config Nginx                   |
| `sudo systemctl reload nginx`            | Apply perubahan config Nginx            |
| `ss -tlnp`                               | Lihat semua port yang lagi listen       |
| `sudo ufw status verbose`                | Cek rule & status firewall              |
| `dig +short feri-irawansyah.my.id`       | Cek A record domain ngarah kemana       |
