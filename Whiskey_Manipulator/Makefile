

compile_rust_test:
	@echo "compiling rust test"
	@cd ./test_rust && cargo build 
run_rust_test:
	@cd ./test_rust/target/debug && ./test_rust
crun_rust_test: compile_rust_source compile_c_source compile_rust_test run_rust_test
compile_c_source:
	@echo "compiling c source"
	@cd ./c_source && gcc -Wall -g -c main.c -o ../compiled_static/Clib.o
compile_rust_source:
	@echo "compiling rust source"
	@cd rust_source && cargo build
