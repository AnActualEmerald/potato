test:
	cargo run -- roms/test_opcode.ch8
	
test-all: build
	-timeout 5 cargo run -- roms/IBM_Logo.ch8
	-timeout 5 cargo run -- roms/test_opcode.ch8
	
	
	
build:
	cargo build