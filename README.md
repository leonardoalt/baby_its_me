# Baby it's me

In this CTF you need to create a zkSNARK that proves that you provided a valid
signature for the message "Baby it's me, ADDRESS" and BabyJubjub public key
(4342719913949491028786768530115087822524712248835451589697801404893164183326,
4826523245007015323400664741523384119579596407052839571721035538011798951543).
The zkSNARK will be verified on chain, where the message will be created using
your `msg.sender` as the `ADDRESS`.

The Ethereum address of the zkSNARK verifier is
0x7817fa297EDd88a21f9f99c0422F26c02Af3F1a5.  After you generate a zkSNARK
locally, you need to call `verifyProof` in the verifier contract directly from
your address, similarly to how it's done in the tests in `verifier`.  Then you
can submit your address as the solution in the Curta UI or contract.

## Cloning

Clone this repo with `--recursive`.

Requirements:
- [ZoKrates 0.8.7](zokrates.github.io).
- [Foundry](https://github.com/foundry-rs/foundry).

## Generating the zkSNARK

The circuit in `snark/signature.zok` verifies that a given BabyJubjub public
key `sender` signed a message `msg` producing a signature `sig`.

In this challenge the signer is constant and expected to be the public key that
we want to impersonate, given in the first paragraph above.
The signature is private to the prover and will not be revealed by the proof.

After cloning this repository, you should have the compiled circuit `out`, the
proving key `proving.key`, and the verification key `verification.key` in the
`snark` directory.

The binary `out` and the keys are given for reproducibility, and you should not
need to verify those.

### Testing your signature

In order to test your private signature before trying out in the smart
contract, create a new JSON file with the input values below. Note that the
first pair `(x, y)` is the public key above and therefore it is fixed to these
values. The third number is the message to be signed and can be generated using
your Ethereum address in the contract in `verifier/Verifier.sol:generate`.
The fourth and the last number are the same, and must be the Ethereum address
you will use to submit your solution. The second copy is committed to the
proof, and the first one will be used onchain as a public input in the verifier
from `msg.sender`.
The pair `(r, s)` is the private signature. Note that the numbers here are
just examples and this should not work.

// snark/in.json
```json
[
  {
    "x": "4342719913949491028786768530115087822524712248835451589697801404893164183326",
    "y": "4826523245007015323400664741523384119579596407052839571721035538011798951543"
  },
  "1234567890123456789012345678901234567890123456789012345678901234567890",
  "ETH_ADDR",
  {
    "r": {
      "x": "1234567890123456789012345678901234567890123456789012345678901234567890",
      "y": "1234567890123456789012345678901234567890123456789012345678901234567890"
    },
    "s": "1234567890123456789012345678901234567890123456789012345678901234567890"
  },
  "ETH_ADDR"
]
```

Make sure you only use decimal numbers inside quotes (big numbers).

#### Compute the witness

```bash
$ cd snark
$ cat in.json | zokrates compute-witness --abi --stdin --verbose
```

If you see
```
Witness:
[]

Witness file written to 'witness'
```
then your input values pass signature verification and you can now generate a proof.

If you see
```
Execution failed: Assertion failed at signature.zok
```
or other errors then your input values do not pass signature verification and
the proof will not be successful.

After successfully generating a witness you will have a file named `witness`.

### Generate the proof

```bash
$ cd snark && make proof
```

After successfully generating a proof you will have a file named `proof.json`.
This proof can be verified on chain.

### Testing the proof

The proof can be tested by ZoKrates with
```bash
$ cd snark
$ zokrates verify
```

The next step is verifying the proof in the smart contract. The file
`verifier/src/SnarkVerifier.sol` is auto generated from ZoKrates and represents
the verification key. You don't have to, but you can generate the same file
with
```bash
$ cd snark
$ make verifier
```

You are **not** expected to have to analyze this file.

The Foundry tests have a zero knowledge proof that should not work, it is there
just as a template that you can use to locally test your solution.  To test
your proof, replace the values in the `proof` struct in
`verifier/test/Verifier.t.sol:testProof` with the `a`, `b`, and `c` values from
your `snark/proof.json`.  Note that you also need to replace `vm.prank(me)`
with your own address.
Now test your proof with

```bash
$ cd verifier
$ forge test --mt testProof
```
