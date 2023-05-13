all:
	zokrates compile -i signature.zok

setup:
	zokrates setup

verifier:
	zokrates export-verifier
	sed -i -e '189,202d' verifier.sol

proof:
	zokrates generate-proof

clean:
	rm abi.json out out.r1cs out.wtns witness verification.key proof.json verifier.sol
