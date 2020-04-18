nrf5_build_image:
	docker build -t nrf5_build docker/nrf5_build

openocd_image:
	docker build -t openocd docker/openocd

CURRENT_DIR := $(shell pwd)

build:
	docker run --rm -v $(CURRENT_DIR)/app:/app -w /app nrf5_build make

flash_app:
	docker run --rm -v $(CURRENT_DIR)/app:/app -w /app openocd \
		-f openocd.cfg \
		-c init \
		-c "reset init" \
		-c halt \
		-c "nrf5 mass_erase" \
		-c "program /app/_build/nrf52832_xxaa.hex verify" \
		-c reset \
		-c exit

flash_softdevice: softdevice/s132.hex
	docker run --rm -v $(CURRENT_DIR)/softdevice:/softdevice -w /app openocd \
		-f openocd.cfg \
		-c init \
		-c "reset init" \
		-c halt \
		-c "nrf5 mass_erase" \
		-c "program /softdevice/s132.hex verify" \
		-c reset \
		-c exit

softdevice/s132.hex:
	mkdir -p softdevice
	docker run --rm -v $(CURRENT_DIR)/softdevice:/softdevice nrf5_build cp /opt/nrf5_sdk/components/softdevice/s132/hex/s132_nrf52_7.0.1_softdevice.hex /softdevice/s132.hex
