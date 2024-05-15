echo 1 > sudo /proc/sys/net/ipv4/ip_forward

# iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
sudo nft add table nat
sudo nft add chain nat postrouting { type nat hook postrouting priority 100 \; }
sudo nft add rule ip nat postrouting oifname "eth0" counter masquerade

# iptables -A FORWARD -i eth0 -o longge -m state --state RELATED,ESTABLISHED -j ACCEPT
sudo nft add table ip filter
sudo nft add chain ip filter forward { type filter hook forward priority 0 \; }
sudo nft add rule ip filter forward iifname "eth0" oifname "longge" ct state related,established counter accept

# iptables -A FORWARD -i longge -o eth0 -j ACCEPT
sudo nft add rule ip filter forward iifname "longge" oifname "eth0" counter accept
