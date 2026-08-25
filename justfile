binary_dir := "/usr/local/bin/"
layouts_dir := "/usr/local/share/layshift/layouts"

build:
    cargo build
build-release:
    LAYOUTS_DIR={{layouts_dir}} cargo build --release
install: build-release
    sudo install -Dm755 target/release/layshift {{binary_dir}}/layshift
    sudo mkdir -p {{layouts_dir}}
    sudo cp -r layouts/. {{layouts_dir}}/
uninstalll:
    sudo rm -f {{binary_dir}}/layshift
    sudo rm -rf {{layouts_dir}}
