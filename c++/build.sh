
rm tests tekton benchmarks

g++ -static  -std=c++20 -O3 -mavx2 \
    -Ideps/args-parser\
    -Ideps/googletest/dist/include\
    -Ldeps/googletest/dist/lib\
    -o tests\
    tekton_tests.cxx deps/googletest/dist/lib/libgtest.a
     

g++ -static -std=c++20 -O3 -mavx2 \
    -Ideps/args-parser\
     -o tekton\
    tekton.cxx

g++ -static -std=c++20 -O3 -mavx2 \
     -o benchmarks\
     -Ideps/AES/src\
    tekton_benchmarks.cxx deps/AES/src/AES.cpp
    
chmod +x tests 
chmod +x tekton