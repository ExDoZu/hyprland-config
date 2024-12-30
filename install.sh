#!/bin/bash

### install packages
sudo pacman -Syu --needed - < hyprland-packages.txt
sudo pacman -Syu --needed - < user-packages.txt

### grub background :)
sudo cp pictures/grub/grub.png /boot/grub/grub.png

### build utils. Yes, I use Rust instead of sh. So what?
rustup default stable
cd scripts/wallpaper-handler
cargo build --release
mv target/release/wallpaper-handler ../wallpaper-handler-bin
cd ..
rm -r wallpaper-handler
cd ..

### install media
mkdir -p ~/Pictures/Screenshots
cp -r pictures/. ~/Pictures/hyprland-config-pictures

### install configs
cp -r hypr ~/.config/
cp -r kitty ~/.config/
cp -r Thunar ~/.config/
cp -r waybar ~/.config/
