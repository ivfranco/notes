CARGO=cargo.exe

head -c $((1024 * 1024)) < /dev/urandom > input.bin
$CARGO run --bin create_csum -q -- input.bin -o output.bin
$CARGO run --bin check_csum -q -- -i input.bin -c output.bin

printf "\xff" | dd of=input.bin bs=1 seek=8000 count=1 conv=notrunc
$CARGO run --bin check_csum -q -- -i input.bin -c output.bin
