Making perf tracing work:
1. WSL2 on windows, use ubuntu

1. perf was not installed
https://docs.clamav.net/manual/Development/performance-profiling.html
sudo apt install flex bison
git clone https://github.com/microsoft/WSL2-Linux-Kernel --depth 1
cd WSL2-Linux-Kernel/tools/perf
make -j8
sudo cp perf /usr/local/bin

1. Every caller was showing as unknown, solved by: https://users.rust-lang.org/t/flamegraph-shows-every-caller-is-unknown/52408/2 echo 0 |sudo tee /proc/sys/kernel/kptr_restrict
