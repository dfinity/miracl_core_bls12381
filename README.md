This repo contains a subset of [`miracl_core`](https://github.com/miracl/core) for curve BLS 12-381.
It was built from [commit 938c5e7](https://github.com/miracl/core/commit/938c5e754e8281572831bc28bd589e914118c0a2)
according to [these instructions](https://github.com/miracl/core/tree/master/rust#using-miracl-core-with-cargo),
running `python3 config64.py` for curve #31 (bls12381).

The version of this crate follows the version of the Miracl Core releases, with a patch that
increments for every local fixes (like documentation or otherwise) that don't affect the code.
