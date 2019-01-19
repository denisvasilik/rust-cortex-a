FROM buildpack-deps:stretch 

RUN git clone git://git.qemu.org/qemu.git; \
    cd qemu; \
    ./configure --target-list=arm-softmmu --enable-modules \
        --enable-tcg-interpreter --enable-debug-tcg \
        --python=/usr/bin/python2.7; \
    make; \
    make install; \
    cd ..; \
    rm -rf qemu
