hyperfine --warmup 10 'wget -q -O- http://localhost:8000/hbs > /dev/null' # 267 runs
hyperfine --warmup 10 'wget -q -O- http://localhost:8000/maud > /dev/null' # 281 runs => 10% faster
# kirby on herd osx has like 110 runs for similar tests