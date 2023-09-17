c_compiler = gcc
build_dependencies = file1.o file2.o
file1_dependencies = file1.c
file2_dependencies = file2.c file2.h
bin_filename = ex_file
build: $(build_dependencies)
	$(c_compiler) -o $(bin_filename) $(build_dependencies)
file1.o: $(file1_dependencies)
	gcc -c $(file1_dependencies)
file2.o: $(file2_dependencies)
	gcc -c $(file2_dependencies)