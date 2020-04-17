FROM debian:10

RUN apt-get update
RUN apt-get install -y \
  curl unzip

RUN curl 'https://www.nordicsemi.com/api/sitecore/Products/MedialibraryZipDownload2' \
  --data 'ids=502F00475E6A4276A763FFE3545F88A3%7C&fileName=DeviceDownload' \
  --output /tmp/nrf5_sdk.zip

RUN unzip /tmp/nrf5_sdk.zip -d /tmp

RUN mkdir -p /opt/nrf5_sdk && unzip /tmp/nRF5SDK*.zip -d /opt/nrf5_sdk

RUN apt-get install -y gcc-arm-none-eabi

COPY ./Makefile.posix /opt/nrf5_sdk/components/toolchain/gcc/

RUN apt-get install -y make

RUN make -C /opt/nrf5_sdk/examples/peripheral/blinky/pca10059/mbr/armgcc/

RUN apt-get install -y gdb
