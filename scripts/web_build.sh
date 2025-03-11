
rm -rf ./website/src/pkg
# TODO: remove debug flag
wasm-pack build --out-dir ./website/src/pkg/ --target web --weak-refs --debug -- --no-default-features --features web 
