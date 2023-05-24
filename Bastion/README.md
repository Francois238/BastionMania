# Bastion
Le bastion de Bastion Mania.
## Container run option
### Exposed ports
- 22/tcp : SSH
- 9000/tcp : HTTP
- 60244/udp : Wireguard

### Capabilities
- NET_ADMIN : for /dev/tun access

### Volumes
- /data : Bastion data