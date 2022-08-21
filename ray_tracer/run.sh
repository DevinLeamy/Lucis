RANDOM=$(shuf -i 1-10000000 -n 1)

image_file="./images/render_$RANDOM.ppm" 
cargo run > "$image_file"
echo "Saved to: [$image_file]"

open "$image_file"
