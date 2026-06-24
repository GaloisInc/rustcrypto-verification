
.PHONY: aes-verif sha1-verif sha2-verif

clean:
	cargo clean

verify-aes: saw-build
	CRYPTOLPATH=${CRYPTOLPATH} saw aws-verif/aes.saw

verify-sha1: saw-build
	cd sha1-verif && CRYPTOLPATH=${PWD}/../cryptol-specs saw sha1.saw

verify-sha2: saw-build
	cd sha2-verif && CRYPTOLPATH=${PWD}/../cryptol-specs saw sha2.saw

saw-build:
	RUSTFLAGS="--cfg aes_force_soft" cargo saw-build
	find target -name "*sha1_verif*linked-mir.json" | xargs -I % mv % target/sha1_verif.linked-mir.json
	find target -name "*sha2_verif*linked-mir.json" | xargs -I % mv % target/sha2_verif.linked-mir.json
	find target -name "*aes_verif*linked-mir.json" | xargs -I % mv % target/aes_verif.linked-mir.json
