# SP1 ICS23 Template

An example of [ICS23] verification on Ethereum powered by SP1.

## Overview

ICS23 is the commitment standard used by IBC, allowing chains to verify state inclusion across light clients. The SP1 ICS23 template demonstrates how to use SP1 to generate proofs of ICS23 commitment verifications, and verify it on Ethereum.

When requesting a proof, you must supply the program with a merkle root hash. For Tendermint light clients, this can be retrieved from the block header, which should be separately verified using the [SP1 Tendermint] program.

## Example

Before you can successfully generate a proof locally, ensure you have followed the instructions to install [SP1].

For this example, we will be using [this][tx] IBC receive packet message. This specific transfer gives us the following inputs:

```shell
# Retrieved via the /block RPC endpoint.
MERKLE_ROOT     = eoeq0RV8L4fjApSM/rxSUCuhAuCf8tCYAJ0FsD94MVc=

# Retrieved via the /abci_query RPC endpoint.
MERKLE_PROOF    = CrEICq4ICj5jb21taXRtZW50cy9wb3J0cy90cmFuc2Zlci9jaGFubmVscy9jaGFubmVsLTEvc2VxdWVuY2VzLzE2NjYzMBIgp847odf8QF17fp6aNBeFqJbRWwWgHYNcPCxwAkuQuqgaDggBGAEgASoGAALGxvgFIi4IARIHAgTGxvgFIBohINrNshqA/y9qgXdROLe0fUtP1XoCS757C0Fu85vJbsZaIiwIARIoBAjGxvgFII+/WzDByTrJpal3B9WaC6/SF++mbeG6uGSFRxYLqAQyICIsCAESKAYQxsb4BSCvWg22kseAqzCUCUTiBKbjDrG4TfDStr0tpGtPexyH+iAiLAgBEigIIMbG+AUgxRpF0UxZF2zOVR8QT6EvC61vUXorenF7uZKRcwX8s0MgIi4IARIHDETGxvgFIBohIK7DxGA0YP34J/EmXOu8hgikAb13etvXTbCUNVbEYrW+Ii0IARIpDowBxsb4BSDT4M2cv/mnLMUKTHQZulip8vHKUElbNdBzsTGPd8OMOCAiLQgBEikQoALGxvgFIGIVYQfpbVuT2K0DTWPPvhWNyeDUfwgpzd2pdPxyHH9XICItCAESKRLoA8bG+AUgP+QObxTNQCVXqwMsCZmk9TCRA6wy8AZZbe8wWmNc9l0gIi8IARIIFpoLxsb4BSAaISBIpR1cDwDuZF3cQ6GECteCTs8sd9zgOF9VRWUTq0sS2iItCAESKRj8HcbG+AUgjLZ2LNA9GW0t9PBACwbmH39e0BXxLYC0K1ge2halt8cgIi0IARIpGsAuxsb4BSDwKfPQSQuv2vgpAZgOP7QW/c5UGKna8wHDAV1zyQlOnyAiLQgBEikcgGrGxvgFICpxng94UeMV7esaUbU7O7OOr9QzB3C+cFAvV3GK3rICICIuCAESKiCUgwPGxvgFIHpm2QCXTuvf258eYv4TbKmXrA2XdWaadsSHxVbv7JLgICIuCAESKiLghwbGxvgFIN7uoHBMZfsi7+bhRzvq7ODGYqIkccMxNLQQE+nuum4bICIuCAESKiTy+gjGxvgFIEm5B/pEZQYLSL8JkXtXPu+hargviKkc2JCaChPvOaD8ICIuCAESKiaQuhjGxvgFIHg6nsfqYAShC33VWYUdxLE8Gq8WXhuBz1eVE6ZpiOJ/ICIwCAESCSj28CDGxvgFIBohIJk6Ub5HvP6ABuWfRZvVHYHRvTTn9CLzT7PgS7NVwzdPIjAIARIJKtSUL8bG+AUgGiEg5KO88XUHzSFgZMZ+554Yr/M1hTxxSTTRl+8zoQelAyciMAgBEgksiLdFxsb4BSAaISAgZL4s7T0V8HJILLku2VHwB4j1OVWs+fYb1zGcVqNnCiIvCAESKy6k3ZABxsb4BSA90T6Zzge2m6oVHcylVTTPYKezDliiMFMOy3cdq5EGBSAK/gEK+wEKA2liYxIgpwrHCEyUBLPMO+6QTwJneGZvknrTB6GCb8MQgzzNa48aCQgBGAEgASoBACInCAESAQEaIOfPsmlu9wWED/rf+vbikBIXFyHk9w/rF+xqpegzcDpvIiUIARIhAVUv57zF+6/zdFDE2j8FslBXKkgdzSSC+q4AOWroTZI/IicIARIBARogrWXmiXek3UcGRGRj15mDSdYxGj51bwasJHlm4+upM+EiJQgBEiEBSTr90PNsxLxfDexuMosguq4Wo4QV3xTcKMX/VkCBKMQiJwgBEgEBGiB22s1GhVC/MK/NQf5++Irir0ry2djNzvhAZHZyReYxPg==

# https://github.com/cosmos/ibc-go/blob/v4.6.0/modules/light-clients/07-tendermint/types/client_state.go#L381-L382
MERKLE_PATH     = CgNpYmMKPmNvbW1pdG1lbnRzL3BvcnRzL3RyYW5zZmVyL2NoYW5uZWxzL2NoYW5uZWwtMS9zZXF1ZW5jZXMvMTY2NjMw

# https://github.com/cosmos/ibc-go/blob/v4.6.0/modules/core/04-channel/types/packet.go#L19
COMMITMENT_VALUE = p847odf8QF17fp6aNBeFqJbRWwWgHYNcPCxwAkuQuqg=
```

Now, we can pass these inputs into our program to generate a proof:

```shell
cd script
RUST_LOG=info cargo run --bin prove --release $MERKLE_ROOT $MERKLE_PROOF $MERKLE_PATH $COMMITMENT_VALUE
```

[ics23]: https://github.com/cosmos/ibc/tree/main/spec/core/ics-023-vector-commitments
[sp1]: https://succinctlabs.github.io/sp1
[sp1 tendermint]: https://github.com/succinctlabs/sp1-tendermint-example
[tx]: https://www.mintscan.io/osmosis/tx/619A51A1D9648FE2FBC38101A4B2645A0960126781452D082C8B3EBAA791AEB6?height=15783169
