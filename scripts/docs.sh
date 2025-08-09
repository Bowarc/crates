# builds and publish the docs

echo Cleaning old docs
cargo clean --doc
rm -rf ./docs

echo Building updated docs
# cargo doc --no-deps
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --no-deps

echo Creating ./docs
mkdir ./docs

echo Copying generated docs to ./docs
cp -r ./target/doc/* ./docs

echo Commiting docs
git add ./docs
git commit -m "Updating docs"

echo Pushing docs
git push
