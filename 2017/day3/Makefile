day3: day3_hs day3_rs

day3_hs:
	ghc -outputdir build_hs -O3 -o $@ src/haskell/Main.hs src/haskell/Day3.hs ../lib/Lib.hs && rm -rf build_hs

day3_rs:
	rustc -C opt-level=3 src/rust/main.rs -o $@

day3_c:
	gcc -O3 -o $@ -I../lib ../lib/lib.c src/c/main.c

clean:
	rm -f day3_hs day3_rs

bench: day3
	hyperfine -N --warmup 5 './day3_hs 289326' './day3_rs 289326'

.PHONY: day3_hs day3_rs clean bench
