
# Warning

This crate is no longer used within the Internet Computer, and it will not be
maintained further by DFINITY. Additionally, be warned that this crate is known
to have issues with regards to side channel vulnerabilities. In the near future
all published versions will be yanked from crates.io. If you wish to maintain
this crate and take ownership of it on crates.io, please contact the developers
to coordinate this.

# About This Crate

This repo contains a subset of [`miracl_core`](https://github.com/miracl/core) for curve BLS 12-381.
It was built from [commit b670e58](https://github.com/miracl/core/commit/b670e5803554080ad2a61cb2cb10e433fa20306d)
according to [these instructions](https://github.com/miracl/core/tree/master/rust#using-miracl-core-with-cargo),
running `python3 config64.py` for curve #31 (bls12381).

The version of this crate follows the version of the Miracl Core releases, with a patch that
increments for every local fixes (like documentation or otherwise) that don't affect the code.
