#!/usr/bin/env python3
"""Genera o limpia el árbol de fixtures simulado para los tests de integración.

Uso:
    python3 tools/gen_fixtures.py --generate   # recrea tests/fixtures desde cero
    python3 tools/gen_fixtures.py --clean      # elimina tests/fixtures

Los tamaños de los archivos son deterministas: los tests asumen exactamente
esos bytes (alpha=1024, beta=512, target bueno=128, etc.).
"""

import argparse
import os
import shutil
import sys

RAIZ = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
FIXTURES = os.path.join(RAIZ, "tests", "fixtures")


def escribir(ruta: str, bytes_: int, relleno: str = "x") -> None:
    os.makedirs(os.path.dirname(ruta), exist_ok=True)
    with open(ruta, "wb") as f:
        f.write((relleno * bytes_)[:bytes_].encode())


def generar() -> None:
    if os.path.exists(FIXTURES):
        shutil.rmtree(FIXTURES)
    os.makedirs(FIXTURES)

    escribir(os.path.join(FIXTURES, ".cache", "alpha", "archivo.txt"), 1024)
    escribir(os.path.join(FIXTURES, ".cache", "beta", "dentro.bin"), 512)
    escribir(os.path.join(FIXTURES, ".cache", "cargo", "vacio"), 0)
    escribir(os.path.join(FIXTURES, ".cache", "pnpm", "store"), 0)
    escribir(os.path.join(FIXTURES, ".cache", "pip", "cache.bin"), 8)

    escribir(os.path.join(FIXTURES, ".rustup", "tmp", "tmpdir", "frag"), 16)

    escribir(os.path.join(FIXTURES, ".npm", "_cacache", "blob"), 32)

    escribir(
        os.path.join(FIXTURES, "Projects", "buena", "target", ".fingerprint", "marca"),
        2,
    )
    escribir(os.path.join(FIXTURES, "Projects", "buena", "target", "build", "out"), 96)
    escribir(os.path.join(FIXTURES, "Projects", "buena", "src", "main.rs"), 12)

    escribir(
        os.path.join(FIXTURES, "Projects", "falsa", "target", "datos.bin"), 2048
    )

    escribir(os.path.join(FIXTURES, "journal", "system.journal"), 64)
    escribir(os.path.join(FIXTURES, "journal", "current", "db"), 4)

    sin_permiso = os.path.join(FIXTURES, "journal", "sin_permiso")
    escribir(os.path.join(sin_permiso, "secreto"), 8)
    os.chmod(sin_permiso, 0o000)

    print(f"Fixtures generados en {FIXTURES}")


def limpiar() -> None:
    if os.path.exists(FIXTURES):
        shutil.rmtree(FIXTURES)
        print(f"Fixtures eliminados ({FIXTURES})")
    else:
        print("No había fixtures que limpiar")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--generate", action="store_true", help="recrear el árbol de fixtures"
    )
    parser.add_argument("--clean", action="store_true", help="eliminar el árbol")
    args = parser.parse_args()

    if args.generate:
        generar()
    elif args.clean:
        limpiar()
    else:
        parser.print_help()
        sys.exit(1)


if __name__ == "__main__":
    main()
