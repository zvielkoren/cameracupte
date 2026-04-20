pkgname=camera-cupte-git
pkgver=v0.0.1.r1.g96ee685
pkgrel=1
pkgdesc="CameraCupte - Canon EOS Virtual Webcam & Controller"
arch=('x86_64')
url="https://github.com/zvielkoren/cameracupte"
license=('MIT')
depends=('gtk3' 'webkit2gtk-4.1' 'alsa-lib' 'xdotool' 'libjpeg-turbo')
makedepends=('cargo' 'npm' 'nodejs')
source=("git+file://${PWD}")
md5sums=('SKIP')

pkgver() {
  cd "$srcdir/cameracupte"
  git describe --long --tags 2>/dev/null | sed 's/\([^-]*-g\)/r\1/;s/-/./g' ||
  printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
  cd "$srcdir/cameracupte"
  export APPIMAGE_EXTRACT_AND_RUN=1
  export TURBOJPEG_SOURCE=explicit
  export TURBOJPEG_LIB_DIR=/usr/lib
  export TURBOJPEG_INCLUDE_DIR=/usr/include
  export TURBOJPEG_DYNAMIC=1
  
  # Prevent Tauri from trying to build deb/appimage during Arch packaging
  sed -i 's/"targets": \["deb", "appimage"\]/"targets": \[\]/g' src-tauri/tauri.conf.json
  sed -i 's/"targets": \["deb"\]/"targets": \[\]/g' src-tauri/tauri.conf.json
  
  npm install
  npm run build -- --no-bundle
}

package() {
  cd "$srcdir/cameracupte"
  
  install -Dm755 src-tauri/target/release/cameracupte "$pkgdir/usr/bin/camera-cupte"
  
  # Install desktop file and icons if available
  # install -Dm644 src-tauri/icons/128x128.png "$pkgdir/usr/share/pixmaps/camera-cupte.png"
}
