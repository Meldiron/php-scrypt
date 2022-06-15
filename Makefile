build:
	cargo build

run:
	php -dextension=./target/debug/libscrypt_php.so test.php