PREFIX = /usr/local

all:
	$(MAKE) clean
	$(MAKE) release

release:
	cmake --no-warn-unused-cli -DCMAKE_BUILD_TYPE:STRING=Release -DCMAKE_INSTALL_PREFIX:STRING=${PREFIX} -S . -B ./build
	cmake --build ./build --config Release --target all -j`nproc 2>/dev/null || getconf NPROCESSORS_CONF`

debug:
	cmake --no-warn-unused-cli -DCMAKE_BUILD_TYPE:STRING=Debug -DCMAKE_INSTALL_PREFIX:STRING=${PREFIX} -S . -B ./build
	cmake --build ./build --config Debug --target all -j`nproc 2>/dev/null || getconf NPROCESSORS_CONF`

clean:
	rm -rf build

install:
	cmake --install ./build

uninstall:
	xargs rm < ./build/install_manifest.txt