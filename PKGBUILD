pkgname=rusks
pkgver=1.0.0
pkgrel=1
pkgdesc="A command line task manager"
arch=('x86_64')
license=('GPL')
depends=('cargo' 'rust')
# sha256sums=('SKIP')

build() {
    cargo build --release
}

package() {
    cd "$srcdir/.."
    echo $(pwd)
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
