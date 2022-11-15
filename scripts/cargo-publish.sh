set -e
BASEDIR=$(dirname $0)
pushd ${BASEDIR}/../core
cargo publish
echo "waiting 10sec..."
sleep 10
popd
pushd ${BASEDIR}/../macro
cargo publish
echo "waiting 10sec..."
sleep 10
popd
pushd ${BASEDIR}/../
cargo publish
popd

