# Bare Metal Application for ARMv7 Cortex-A9

A simple bare metal application for ARMv7 Cortex-A9.

## Docker Image

A docker image is used to run qemu and start a debug session of the application. The make target `docker-image` builds the image. After the image has been built, the make target `qemu-docker` can be used to start a debug session.

```
~$ make docker-image
~$ make qemu-docker
```

## Sample Debug Session

The debug session started in within the docker container listens on the `port 1234`.

```bash
~$ make qemu-docker 
~$ make debug
(gdb) target remote :1234
(gdb) break entry_point
(gdb) continue
(gdb) list
```
