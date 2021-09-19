nrfutil pkg generate --hw-version 52 --sd-req 0x00 --application ./build.hex --application-version 1 prog.zip
nrfutil dfu usb-serial -pkg prog.zip -p COM5