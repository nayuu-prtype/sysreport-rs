# Sysreport

![Rust](https://img.shields.io/badge/rust-v1.70+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

Sysreport é um utilitário de linha de comando escrito em rust que coleta as informações do sistema e as envia para um webhook (discord), utilizando as crates sysinfo e reqwest

## requisitos
- Git
- Rustc
- Cargo
- Webhook

## instalando sysreport

Clone o repositório
```bash
git clone git@github.com:nayuu-prtype/sysreport-rs
cd sysreport-rs/
```

mude o nome do .env.example para .env
```bash
mv .env.example .env
```

com o seu editor preferido coloque seu webhook na variável WEBHOOK=https://discord.com/api/webhooks/sua_url_aqui
```bash
nvim .env
```

rode o sysreport com o cargo
```bash
cargo run
```

para compilar a versão final otimizada:
```bash
cargo build --release
# o bínario final gerado estará em .target/release/sysreport
```

exemplo de saída:

```
Telemetry script:

┌ Software ┐
│hostname  │   nitrov15
│          │
│system    │   NixOS
│kernel    │   7.1.2
│uptime    │   7.3d
└──────────┘

┌ Hardware ──────┐
│cpu usage       │   3.17% CPU usage
│                │
│cpu avg one     │   1.0/16 CPUs
│cpu avg five    │   0.9/16 CPUs
│cpu avg fifteen │   0.9/16 CPUs
│                │
│ram usage       │   8.2/15.3GB RAM
│swap usage      │   0.2/8.8GB RAM
└────────────────┘

Networks

network: wlan0
data: 1.51GB (down)
data: 172.38MB (up)

network: lo
data: 1.26MB (down)
data: 1.26MB (up)

Disks

name: "/dev/nvme0n1p2"
disk: 159.00/458.72GB, available: 299.72GB
format: "ext4"
mount: "/"
type: SSD

name: "/dev/nvme0n1p2"
disk: 159.00/458.72GB, available: 299.72GB
format: "ext4"
mount: "/nix/store"
type: SSD

name: "/dev/nvme0n1p1"
disk: 0.12/1.00GB, available: 0.88GB
format: "vfat"
mount: "/boot"
type: SSD

Temperature

36.0 °C  coretemp Core 4
32.0 °C  coretemp Core 20
34.0 °C  coretemp Core 8
33.0 °C  coretemp Core 12
38.0 °C  coretemp Core 26
38.0 °C  coretemp Core 25
38.0 °C  coretemp Core 27
35.0 °C  coretemp Core 0
38.0 °C  coretemp Package id 0
38.0 °C  coretemp Core 24
34.0 °C  coretemp Core 16
42.0 °C  acpitz temp1
29.9 °C  nvme Composite HFS512GEJ9X110N
29.9 °C  nvme Sensor 1 HFS512GEJ9X110N
32.8 °C  nvme Sensor 2 HFS512GEJ9X110N
32.0 °C  mt7921_phy0 temp1
```

também é possível configurar o sysreport para rodar periodicamente como um service do systemd

## Contribuições
Contribuições são bem-vindas! Sinta-se à vontade para abrir uma issue ou enviar um pull request

## Licença
MIT
