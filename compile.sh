SECONDS=0 
echo generating artifacts
pyoxidizer generate-python-embedding-artifacts pyembedded
echo compiling
PYO3_CONFIG_FILE=$(pwd)/pyembedded/pyo3-build-config-file.txt PYTHONPATH=pyembedded/stdlib cargo run --release
echo $SECONDS