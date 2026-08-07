# Maintainer: Johannuel <johannuel@users.noreply.github.com>
# Contributor: Johannuel <johannuel@users.noreply.github.com>

pkgname=rust-space-cleaner
pkgver=0.1.0
pkgrel=1
pkgdesc="A safe cache hunter: scans 24 sources of disk junk and lets you reclaim the space via a ratatui TUI"
arch=('x86_64' 'aarch64')
url="https://github.com/Johannuel/rust-space-cleaner"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo' 'rust')
source=("$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$srcdir/rust-space-cleaner-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$srcdir/rust-space-cleaner-$pkgver"
  install -Dm755 "target/release/rust-space-cleaner" \
    "$pkgdir/usr/bin/rust-space-cleaner"
}