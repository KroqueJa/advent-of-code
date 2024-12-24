day2: day2_hs

day2_hs:
	ghc -outputdir build_hs -O3 -o $@ src/haskell/Main.hs src/haskell/Day2.hs ../lib/Lib.hs && rm -rf build_hs

clean:
	rm -f day2_hs

bench: day2
	hyperfine --warmup 5 './day2_hs input.txt'

.PHONY: day2_hs day2_c day2_rs clean bench
