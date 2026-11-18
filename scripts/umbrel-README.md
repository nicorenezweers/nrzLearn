# nrzLearn - Umbrel-Paket

Dieser Zweig enthaelt nur fertige Dateien fuer umbrelOS, gebaut mit
`scripts/umbrel-release.sh` auf dem Zweig `main`:

- `nrzlearn-server-amd64.gz`, `nrzlearn-server-arm64.gz` - der kleine Dateiserver
  (Linux, statisch gelinkt)
- `public.tar.gz` - die Web-App (WebAssembly), die statischen Seiten fuer Suchmaschinen
  in allen Sprachen und die Kurstexte, vorkomprimiert mit Brotli und gzip
- `SHA256SUMS` - Pruefsummen

`hooks/pre-start` der App `nrz-rust-learn` im nrzStore laedt sie vom Tag
`umbrel-<version>`. Der Quellcode liegt auf dem Zweig `main`.
