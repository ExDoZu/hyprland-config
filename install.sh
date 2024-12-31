#!/bin/bash

### grub background :)
sudo cp pictures/grub/grub.png /boot/grub/grub.png
sudo bash -c 'echo -e "\nGRUB_BACKGROUND=\"/boot/grub/grub.png\"\n" >> /etc/default/grub'
sudo grub-mkconfig -o /boot/grub/grub.cfg

### install packages
git clone https://aur.archlinux.org/paru.git
cd paru
makepkg -si --needed
cd ..
rm -rf paru
paru -Syu --needed - < hyprland-packages.txt
paru -S --needed - < user-packages.txt

### build utils. Yes, I use Rust instead of sh. So what?
rustup default stable
cd scripts/wallpaper-handler
cargo build --release
mkdir bin
mv target/release/wallpaper-handler bin/
cargo clean
cd ../..

### install media
mkdir -p ~/Pictures/Screenshots
cp -r pictures/. ~/Pictures/hyprland-config-pictures

### install configs
rm -rf ~/.config/hypr
cp -r hypr ~/.config/

rm -rf ~/.config/kitty
cp -r kitty ~/.config/

rm -rf ~/.config/Thunar
cp -r Thunar ~/.config/

rm -rf ~/.config/waybar
cp -r waybar ~/.config/

rm -rf ~/.config/wlogout
cp -r wlogout ~/.config/

rm -rf ~/.config/xfce4 
cp -r xfce4 ~/.config/ # for Thunar

### for VPN
sudo systemctl enable systemd-resolved.service && sudo systemctl start systemd-resolved.service 

reboot
