# Copyright (C) 2022-2025 vaxerski
# Licensed under BSD 3-Clause License
# Makefile: https://github.com/hyprwm/Hyprland/blob/main/Makefile
# LICENSE: https://github.com/hyprwm/Hyprland/blob/main/LICENSE
# Partially trimmed down for this project

PREFIX = /usr/local

stub:
	@echo "Do not run $(MAKE) directly without any arguments.."

release:
	cmake --no-warn-unused-cli -DCMAKE_BUILD_TYPE:STRING=Release -DCMAKE_INSTALL_PREFIX:STRING=${PREFIX} -S . -B ./build
	cmake --build ./build --config Release --target all -j`nproc 2>/dev/null || getconf NPROCESSORS_CONF`

debug:
	cmake --no-warn-unused-cli -DCMAKE_BUILD_TYPE:STRING=Debug -DCMAKE_INSTALL_PREFIX:STRING=${PREFIX} -S . -B ./build
	cmake --build ./build --config Debug --target all -j`nproc 2>/dev/null || getconf NPROCESSORS_CONF`

clear:
	rm -rf build

all:
	$(MAKE) clear
	$(MAKE) release

install:
	cmake --install ./build

uninstall:
	xargs rm < ./build/install_manifest.txt