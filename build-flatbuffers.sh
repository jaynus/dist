mkdir -p target/flatbuffers && cd target/flatbuffers
cmake -G "Unix Makefiles" ../../flatbuffers -DCMAKE_INSTALL_PREFIX=../../bin
make -j8 && make install
cd ../../
mkdir -p bin/bin
cp target/flatbuffers/flatc bin/bin
