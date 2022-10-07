cd deps

git clone --depth 1 --branch 6.3.2 https://github.com/igormironchik/args-parser.git
git clone --depth 1 --branch release-1.12.1 https://github.com/google/googletest.git
git clone --depth 1 --branch v7.3.0 https://github.com/SergeyBel/AES.git

cd googletest
mkdir build
mkdir dist
cd build
cmake -DCMAKE_INSTALL_PREFIX:PATH=../dist build ..
make -j
make install

cd ../../AES 
mkdir dist 
make 
make install PREFIX=./dist