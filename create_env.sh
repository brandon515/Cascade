echo "export PREFIX=\"\$HOME/opt/cross\"
export TARGET=x86_64-elf
export PATH=\"$PREFIX/bin:\$PATH\"" >> $HOME/.bashrc
PREFIX="$HOME/opt/cross"
TARGET=x86_64-elf
mkdir -p $PREFIX
sudo sed "$ i deb http://lug.mtu.edu/ubuntu impish main" /etc/apt/sources.list
sudo apt update
sudo apt install build-essential bison flex libgmp3-dev libmpc-dev libmpfr-dev texinfo nasm qemu xorriso grub-pc qemu-kvm
mkdir ../cross-com
cd ../cross-com
wget https://ftp.gnu.org/gnu/binutils/binutils-2.37.tar.xz
wget https://ftp.gnu.org/gnu/gcc/gcc-11.2.0/gcc-11.2.0.tar.xz
tar -xvf binutils-2.37.tar.xz
tar -xvf gcc-11.2.0.tar.xz
mkdir build-gcc
mkdir build-bin
cd build-bin
../binutils-2.37/configure --target=$TARGET --prefix="$PREFIX" --with-sysroot --disable-nls --disable-werror
make
make install
cd ../build-gcc
echo "# Add libgcc multilib variant without red-zone requirement
 
MULTILIB_OPTIONS += mno-red-zone
MULTILIB_DIRNAMES += no-red-zone" > ../gcc-11.2.0/gcc/config/t-x86_64-elf
sed -i "1900 i tmake_file=\"\${tmake_file} i386/t-x86_64-elf\" # include the new multilib configuration" ../gcc-11.2.0/gcc/config.gcc
../gcc-11.2.0/configure --target=$TARGET --prefix="$PREFIX" --disable-nls --enable-languages=c,c++ --without-headers
make all-gcc
make all-target-libgcc
make install-gcc
make install-target-libgcc
