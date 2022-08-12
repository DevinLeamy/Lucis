counter="$(< counter.txt)"
image_file="./images/image$counter.ppm" 
cargo run > "$image_file"
echo "Saved to: [$image_file]"
bc <<< "1 + $(< counter.txt)" > counter.txt

