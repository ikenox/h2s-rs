BASEDIR=$(dirname $0)
pushd ${BASEDIR}/../core
cargo publish
popd
pushd ${BASEDIR}/../macro
cargo publish
popd
pushd ${BASEDIR}/../
cargo publish
popd

