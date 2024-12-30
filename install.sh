#!/bin/bash

### grub background :)
sudo cp pictures/grub/grub.png /boot/grub/grub.png
sudo grub-mkconfig -o /boot/grub/grub.cfg

### install packages
git clone https://aur.archlinux.org/paru.git
cd paru
makepkg -si
cd ..
rm -r paru
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
cp -r hypr ~/.config/
cp -r kitty ~/.config/
cp -r Thunar ~/.config/
cp -r waybar ~/.config/
cp -r wlogout ~/.config/

reboot
