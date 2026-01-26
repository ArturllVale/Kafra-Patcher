Docker may be used to build Kafra Patcher through containers.
Through Docker, the build process contains all necessary dependencies.

To build Kafra Patcher with Docker, complete the following procedure:
```shell
# Build the Kafra Patcher build container
$ cd /path/to/kpatcher/docker
$ docker image build -t kpatcher .

# Use the kpatcher image to build kpatcher
$ docker run -v /path/to/kpatcher:/kpatcher kpatcher
$ file /path/to/kpatcher/target/x86_64-pc-windows-gnu/release/KPatcher.exe
/path/to/kpatcher/target/x86_64-pc-windows-gnu/release/KPatcher.exe: PE32+ executable (GUI) x86-64, for MS Windows
```
