# Haskell deps: ghc, bytestring lib

day1: day1_hs day1_c day1_rs

day1_rs:
	rustc -C opt-level=3 src/rust/main.rs -o $@

day1_c:
	gcc -o $@ -I../lib src/c/main.c

day1_hs:
	ghc -outputdir build_hs -O3 -o $@ src/haskell/Main.hs src/haskell/Day1.hs ../lib/Lib.hs && rm -rf build_hs

clean:
	rm day1_hs day1_c day1_rs

.PHONY: day1_hs day1_c day1_rs clean
