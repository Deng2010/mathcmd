# Maintainer: Deng Ziqian <heavenaef@outlook.com>
_pkgname='mathcmd'
pkgname="$_pkgname-git"
pkgver=v0.3.1.r7.g58cd2e3
pkgrel=1
pkgdesc="A simple program to solve math problems."
arch=('any')
url="https://github.com/Deng2010/mathcmd"
license=('MIT')
groups=()
depends=()
makedepends=('git' 'cargo')
provides=('mathcmd')
conflicts=('mathcmd')
replaces=()
backup=()
options=()
install=
source=("$pkgname::git+https://github.com/Deng2010/mathcmd")
noextract=()
md5sums=('SKIP')
pkgver() {
    cd "$pkgname"
    git describe --long --tags --abbrev=7 | sed 's/\([^-]*-g\)/r\1/;s/-/./g'
}

prepare() {
    cd "$srcdir/$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$srcdir/$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "$srcdir/$pkgname/target/release/${_pkgname}"
}
