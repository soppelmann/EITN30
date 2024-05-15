echo '1' | sudo tee /proc/sys/net/ipv4/ip_forward
echo '1' | sudo tee /proc/sys/net/ipv4/conf/eth0/forwarding
echo '1' | sudo tee /proc/sys/net/ipv4/conf/longge/forwarding

sudo ip route add default via 192.168.12.241 dev longge
