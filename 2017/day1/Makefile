# Haskell deps: ghc, bytestring lib

day1: day1_hs day1_c day1_rs

day1_rs:
	rustc -C opt-level=3 src/rust/main.rs -o $@

day1_c:
	gcc -O3 -o $@ -I../lib ../lib/lib.c src/c/main.c

day1_hs:
	ghc -outputdir build_hs -O3 -o $@ src/haskell/Main.hs src/haskell/Day1.hs ../lib/Lib.hs && rm -rf build_hs

clean:
	rm day1_hs day1_c day1_rs

bench: day1
	hyperfine --warmup 5 './day1_hs input.txt' './day1_c input.txt' './day1_rs input.txt' 'python src/python/day1.py input.txt'

.PHONY: day1_hs day1_c day1_rs clean bench
