# Maintainer: Marc Sweetlove <marcs100@gmail.com>

pkgname=scribe-cli
pkgver=0.1.1
pkgrel=1
pkgdesc='Command line version of Scribe'
url='https://github.com/marcs100/scribe-cli'
license=()
makedepends=('rust')
depends=()
conflicts=('scribe-cli')
provides=('scriibe-cli')
arch=('x86_64')
source=("${pkgname}-${pkgver}.tar.gz::https://github.com/marcs100/scribe-cli/archive/${pkgver}.tar.gz")
sha256sums=('6c592e0a925fd249da231346f731bdd40861db114aa118e40c8c665fbbeba290')

options=(!lto) # build fails without this (sqlite link)

prepare() {
    cd "$pkgname-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    cd "$pkgname-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    cd "$pkgname-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "$pkgname-$pkgver/target/release/scribe"
    # for custom license, e.g. MIT
    # install -Dm644 LICENSE "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
}

