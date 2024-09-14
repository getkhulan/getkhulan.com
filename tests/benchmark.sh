hyperfine --warmup 10 'wget -q -O- http://localhost:8000/en > /dev/null' # 257 runs
hyperfine --warmup 10 'wget -q -O- http://localhost:8000/en/blog > /dev/null' # 257 runs
hyperfine --warmup 10 'wget -q -O- http://localhost:8000/en/blog/whales > /dev/null' # 257 runs
hyperfine --warmup 10 'wget -q -O- http://localhost:8000/de/blog/whales > /dev/null' # 257 runs
hyperfine --warmup 10 'wget -q -O- http://localhost:8000/hbs > /dev/null' # 267-285 runs
hyperfine --warmup 10 'wget -q -O- http://localhost:8000/maud > /dev/null' # 281 runs => max 10% faster than hbs
# kirby on herd osx has like 110 runs for similar tests