echo '1' | sudo tee /proc/sys/net/ipv4/ip_forward
echo '1' | sudo tee /proc/sys/net/ipv4/conf/eth0/forwarding
echo '1' | sudo tee /proc/sys/net/ipv4/conf/longge/forwarding

sudo iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
sudo iptables -A FORWARD -i longge -o eth0 -j ACCEPT
sudo iptables -A FORWARD -i eth0 -o longge -m state --state RELATED,ESTABLISHED -j ACCEPT
