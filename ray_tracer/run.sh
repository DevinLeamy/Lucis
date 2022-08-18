RANDOM=$(date +%s%N | cut -b10-19)

image_file="./images/render_$RANDOM.ppm" 
cargo run > "$image_file"
echo "Saved to: [$image_file]"
