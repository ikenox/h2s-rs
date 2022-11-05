BASEDIR=$(dirname $0)
cd ${BASEDIR}/../core
cargo publish
cd ${BASEDIR}/../macro
cargo publish
cd ${BASEDIR}/../
cargo publish


