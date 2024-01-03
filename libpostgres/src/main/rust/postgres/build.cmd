cargo build --target armv7-linux-androideabi --release
cargo build --target aarch64-linux-android --release
@REM cargo build --target i686-linux-android --release
@REM cargo build --target x86_64-linux-android --release

copy target\armv7-linux-androideabi\release\libpostgres.so ..\..\jniLibs\armeabi-v7a\libpostgres.so
copy target\aarch64-linux-android\release\libpostgres.so ..\..\jniLibs\arm64-v8a\libpostgres.so
@REM copy target\x86_64-linux-android\release\libpostgres.so ..\..\jniLibs\x86_64\libpostgres.so
@REM copy target\i686-linux-android\release\libpostgres.so ..\..\jniLibs\x86\libpostgres.so