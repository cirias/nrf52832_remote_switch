CURRENT_DIR := $(shell pwd)
SRC_DIR := $(CURRENT_DIR)/app
OUT_DIR := $(CURRENT_DIR)/app/_build

SRC_FILES += \
	$(SRC_DIR)/nrf52.ld \
	$(SRC_DIR)/custom_board.h \
	$(SRC_DIR)/ble_cus.h \
	$(SRC_DIR)/ble_cus.c \
	$(SRC_DIR)/main.c \

OUT_FILES += \
	$(OUT_DIR)/nrf52832_xxaa.out \
	$(OUT_DIR)/nrf52832_xxaa.hex \

.PHONY: build

build: $(OUT_FILES)

$(OUT_FILES): $(SRC_FILES)
	docker run --rm -v $(CURRENT_DIR)/app:/app -w /app nrf5_build make

gdb: build
	docker run -it --net=host --rm -v $(CURRENT_DIR)/app:/app -w /app nrf5_build \
		gdb-multiarch -q -x openocd.gdb /app/_build/nrf52832_xxaa.out

openocd: build
	docker run -it --privileged --net=host --rm -v $(CURRENT_DIR)/softdevice:/softdevice -v $(CURRENT_DIR)/app:/app -w /app openocd \
		openocd

flash_all: build
	docker run --privileged --rm -v $(CURRENT_DIR)/softdevice:/softdevice -v $(CURRENT_DIR)/app:/app -w /app openocd \
		openocd \
		-f openocd.cfg \
		-c init \
		-c "reset init" \
		-c halt \
		-c "nrf5 mass_erase" \
		-c "program /softdevice/s132.hex verify" \
		-c "program /app/_build/nrf52832_xxaa.hex verify" \
		-c reset \
		-c exit

flash_app: build
	docker run --privileged --rm -v $(CURRENT_DIR)/app:/app -w /app openocd \
		openocd \
		-f openocd.cfg \
		-c init \
		-c "reset init" \
		-c halt \
		-c "program /app/_build/nrf52832_xxaa.hex verify" \
		-c reset \
		-c exit

flash_softdevice: softdevice/s132.hex
	docker run --privileged --rm -v $(CURRENT_DIR)/softdevice:/softdevice -w /app openocd \
		openocd \
		-f openocd.cfg \
		-c init \
		-c "reset init" \
		-c halt \
		-c "program /softdevice/s132.hex verify" \
		-c reset \
		-c exit

softdevice/s132.hex:
	mkdir -p softdevice
	docker run --rm -v $(CURRENT_DIR)/softdevice:/softdevice nrf5_build cp /opt/nrf5_sdk/components/softdevice/s132/hex/s132_nrf52_7.0.1_softdevice.hex /softdevice/s132.hex



nrf5_build_image:
	docker build -t nrf5_build docker/nrf5_build

openocd_image:
	docker build -t openocd docker/openocd
