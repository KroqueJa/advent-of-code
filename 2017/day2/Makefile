day2: day2_hs day2_rs day2_c

day2_hs:
	ghc -outputdir build_hs -O3 -o $@ src/haskell/Main.hs src/haskell/Day2.hs ../lib/Lib.hs && rm -rf build_hs

day2_rs:
	rustc -C opt-level=3 src/rust/main.rs -o $@

day2_c:
	gcc -O3 -o $@ -I../lib ../lib/lib.c src/c/main.c

clean:
	rm -f day2_hs day2_c day2_rs

bench: day2
	hyperfine -N --warmup 5 './day2_hs input.txt' './day2_rs input.txt' './day2_c input.txt' 'python src/python/day2.py input.txt'

.PHONY: day2_hs day2_c day2_rs day2_c clean bench
