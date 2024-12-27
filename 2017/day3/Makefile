day3: day3_hs day3_rs day3_cpp

day3_hs:
	ghc -outputdir build_hs -O3 -o $@ src/haskell/Main.hs src/haskell/Day3.hs ../lib/Lib.hs && rm -rf build_hs

day3_rs:
	rustc -C opt-level=3 src/rust/main.rs -o $@

day3_cpp:
	clang++ -std=c++17 -O3 -o $@ src/cpp/main.cpp

clean:
	rm -f day3_hs day3_rs

bench: day3
	hyperfine -N --warmup 5 './day3_hs 289326' './day3_rs 289326' './day3_cpp 289326' 'python src/python/day3.py 289326'

.PHONY: day3_hs day3_rs day3_cpp clean bench
