FROM debian:10

RUN apt-get update
RUN apt-get install -y \
  git pkg-config make automake autoconf libtool tcl \
  libusb-1.0-0-dev libusb-dev libhidapi-dev libftdi1-dev \
  curl unzip

RUN git clone https://repo.or.cz/openocd.git /tmp/openocd && \
  cd /tmp/openocd && \
  ./bootstrap && \
  ./configure \
    --enable-maintainer-mode \
    --disable-werror \
    --enable-aice \
    --enable-amtjtagaccel \
    --enable-armjtagew \
    --enable-at91rm9200 \
    --enable-bcm2835gpio \
    --enable-buspirate \
    --enable-cmsis-dap \
    --enable-dummy \
    --enable-ep93xx \
    --enable-ftdi \
    --enable-gw16012 \
    --enable-ioutil \
    --enable-jlink \
    --enable-jtag_vpi \
    --enable-legacy-ft2232_libftdi \
    --enable-oocd_trace \
    --enable-opendous \
    --enable-openjtag_ftdi \
    --enable-osbdm \
    --enable-parport \
    --enable-presto_libftdi \
    --enable-remote-bitbang \
    --enable-rlink \
    --enable-stlink \
    --enable-sysfsgpio \
    --enable-ti-icdi \
    --enable-ulink \
    --enable-usb-blaster-2 \
    --enable-usb_blaster_libftdi \
    --enable-usbprog \
    --enable-vsllink \
    --enable-xlnx_pcie_xvc && \
  make && make install

RUN apt-get install -y udev
RUN cp /tmp/openocd/contrib/60-openocd.rules /lib/udev/rules.d/

# RUN curl 'https://www.nordicsemi.com/api/sitecore/Products/MedialibraryZipDownload2' \
  # --data 'ids=502F00475E6A4276A763FFE3545F88A3%7C&fileName=DeviceDownload' \
  # --output /tmp/download.zip

# RUN unzip 
