sudo apt update -y &&
sudo apt install libsdl2-2.0-0 libsdl2-dev build_essential git -y &&
git clone https://github.com/0xBLCKLPTN/GEita.git &&
curl https://sh.rustup.rs -sSf | sh &&
cd GEita && cargo build --release
