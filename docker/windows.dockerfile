FROM alpine

RUN apk add socat gzip qemu-img qemu qemu-system-x86_64

RUN mkdir -p /opt/qemu/{sock,images,volumes}
RUN mkdir -p /opt/qemu/local/images

ARG qemu_img=windows2012:1.4

ENV QEMU_IMG $qemu_img

ADD entry.sh /entry.sh

CMD /entry.sh
