<h1 align=center><code>ord</code></h1>

<div align=center>
  <a href=https://crates.io/crates/ord>
    <img src=https://img.shields.io/crates/v/ord.svg alt="crates.io version">
  </a>
  <a href=https://github.com/ordinals/ord/actions/workflows/ci.yaml>
    <img src=https://github.com/ordinals/ord/actions/workflows/ci.yaml/badge.svg alt="build status">
  </a>
  <a href=https://github.com/ordinals/ord/releases>
    <img src=https://img.shields.io/github/downloads/ordinals/ord/total.svg alt=downloads>
  </a>
  <a href=https://discord.gg/ordinals>
    <img src=https://img.shields.io/discord/987504378242007100?logo=discord alt="chat on discord">
  </a>
</div>
<br>

`ord` is an index, block explorer, and command-line wallet. It is experimental
software with no warranty. See [LICENSE](LICENSE) for more details.

Ordinal theory imbues satoshis with numismatic value, allowing them to
be collected and traded as curios.

Ordinal numbers are serial numbers for satoshis, assigned in the order in which
they are mined, and preserved across transactions.

See [the docs](https://docs.ordinals.com) for documentation and guides.

See [the BIP](bip.mediawiki) for a technical description of the assignment and
transfer algorithm.

See [the project board](https://github.com/orgs/ordinals/projects/1) for
currently prioritized issues.

Join [the Discord server](https://discord.gg/87cjuz4FYg) to chat with fellow
ordinal degenerates.

Donate
------

Ordinals is open-source and community funded. The current lead maintainer of
`ord` is [raphjaph](https://github.com/raphjaph/). Raph's work on `ord` is
entirely funded by donations. If you can, please consider donating!

The donation address is
[bc1qguzk63exy7h5uygg8m2tcenca094a8t464jfyvrmr0s6wkt74wls3zr5m3](https://mempool.space/address/bc1qguzk63exy7h5uygg8m2tcenca094a8t464jfyvrmr0s6wkt74wls3zr5m3).

This address is 2 of 4 multisig wallet with keys held by
[raphjaph](https://twitter.com/raphjaph),
[erin](https://twitter.com/realizingerin),
[rodarmor](https://twitter.com/rodarmor), and
[ordinally](https://twitter.com/veryordinally).

Bitcoin received will go towards funding maintenance and development of `ord`,
as well as hosting costs for [ordinals.com](https://ordinals.com).

Thank you for donating!

Wallet
------

`ord` relies on Bitcoin Core for private key management and transaction signing.
This has a number of implications that you must understand in order to use
`ord` wallet commands safely:

- Bitcoin Core is not aware of inscriptions and does not perform sat
  control. Using `bitcoin-cli` commands and RPC calls with `ord` wallets may
  lead to loss of inscriptions.

- `ord wallet` commands automatically load the `ord` wallet given by the
  `--name` option, which defaults to 'ord'. Keep in mind that after running
  an `ord wallet` command, an `ord` wallet may be loaded.

- Because `ord` has access to your Bitcoin Core wallets, `ord` should not be
  used with wallets that contain a material amount of funds. Keep ordinal and
  cardinal wallets segregated.

Security
--------

The `ord server` explorer hosts untrusted HTML and JavaScript. This creates
potential security vulnerabilities, including cross-site scripting and spoofing
attacks. You are solely responsible for understanding and mitigating these
attacks. See the [documentation](docs/src/security.md) for more details.

Installation
------------

`ord` is written in Rust and can be built from
[source](https://github.com/ordinals/ord). Pre-built binaries are available on the
[releases page](https://github.com/ordinals/ord/releases).

You can install the latest pre-built binary from the command line with:

```sh
curl --proto '=https' --tlsv1.2 -fsLS https://ordinals.com/install.sh | bash -s
```

Once `ord` is installed, you should be able to run `ord --version` on the
command line.

Building
--------

On Linux, `ord` requires `libssl-dev` when building from source.

On Debian-derived Linux distributions, including Ubuntu:

```
sudo apt-get install pkg-config libssl-dev build-essential
```

On Red Hat-derived Linux distributions:

```
yum install -y pkgconfig openssl-devel
yum groupinstall "Development Tools"
```

You'll also need Rust:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the `ord` repo:

```
git clone https://github.com/ordinals/ord.git
cd ord
```

To build a specific version of `ord`, first checkout that version:

```
git checkout <VERSION>
```

And finally to actually build `ord`:

```
cargo build --release
```

Once built, the `ord` binary can be found at `./target/release/ord`.

`ord` requires `rustc` version 1.79.0 or later. Run `rustc --version` to ensure
you have this version. Run `rustup update` to get the latest stable release.

### Docker

A Docker image can be built with:

```
docker build -t ordinals/ord .
```

### Homebrew

`ord` is available in [Homebrew](https://brew.sh/):

```
brew install ord
```

### Debian Package

To build a `.deb` package:

```
cargo install cargo-deb
cargo deb
```

Contributing
------------

If you wish to contribute there are a couple things that are helpful to know. We
put a lot of emphasis on proper testing in the code base, with three broad
categories of tests: unit, integration and fuzz. Unit tests can usually be found at
the bottom of a file in a mod block called `tests`. If you add or modify a
function please also add a corresponding test. Integration tests try to test
end-to-end functionality by executing a subcommand of the binary. Those can be
found in the [tests](tests) directory. We don't have a lot of fuzzing but the
basic structure of how we do it can be found in the [fuzz](fuzz) directory.

We strongly recommend installing [just](https://github.com/casey/just) to make
running the tests easier. To run our CI test suite you would do:

```
just ci
```

This corresponds to the commands:

```
cargo fmt -- --check
cargo test --all
cargo test --all -- --ignored
```

Have a look at the [justfile](justfile) to see some more helpful recipes
(commands). Here are a couple more good ones:

```
just fmt
just fuzz
just doc
just watch ltest --all
```

If the tests are failing or hanging, you might need to increase the maximum
number of open files by running `ulimit -n 1024` in your shell before you run
the tests, or in your shell configuration.

We also try to follow a TDD (Test-Driven-Development) approach, which means we
use tests as a way to get visibility into the code. Tests have to run fast for that
reason so that the feedback loop between making a change, running the test and
seeing the result is small. To facilitate that we created a mocked Bitcoin Core
instance in [mockcore](./crates/mockcore)

Syncing
-------

`ord` requires a synced `bitcoind` node with `-txindex` to build the index of
satoshi locations. `ord` communicates with `bitcoind` via RPC.

If `bitcoind` is run locally by the same user, without additional
configuration, `ord` should find it automatically by reading the `.cookie` file
from `bitcoind`'s datadir, and connecting using the default RPC port.

If `bitcoind` is not on mainnet, is not run by the same user, has a non-default
datadir, or a non-default port, you'll need to pass additional flags to `ord`.
See `ord --help` for details.

`bitcoind` RPC Authentication
-----------------------------

`ord` makes RPC calls to `bitcoind`, which usually requires a username and
password.

By default, `ord` looks a username and password in the cookie file created by
`bitcoind`.

The cookie file path can be configured using `--cookie-file`:

```
ord --cookie-file /path/to/cookie/file server
```

Alternatively, `ord` can be supplied with a username and password on the
command line:

```
ord --bitcoin-rpc-username foo --bitcoin-rpc-password bar server
```

Using environment variables:

```
export ORD_BITCOIN_RPC_USERNAME=foo
export ORD_BITCOIN_RPC_PASSWORD=bar
ord server
```

Or in the config file:

```yaml
bitcoin_rpc_username: foo
bitcoin_rpc_password: bar
```

Logging
--------

`ord` uses [env_logger](https://docs.rs/env_logger/latest/env_logger/). Set the
`RUST_LOG` environment variable in order to turn on logging. For example, run
the server and show `info`-level log messages and above:

```
$ RUST_LOG=info cargo run server
```

Set the `RUST_BACKTRACE` environment variable in order to turn on full rust
backtrace. For example, run the server and turn on debugging and full backtrace:

```
$ RUST_BACKTRACE=1 RUST_LOG=debug ord server
```

New Releases
------------

Release commit messages use the following template:

```
Release x.y.z

- Bump version: x.y.z → x.y.z
- Update changelog
- Update changelog contributor credits
- Update dependencies
```

Translations
------------

To translate [the docs](https://docs.ordinals.com) we use
[mdBook i18n helper](https://github.com/google/mdbook-i18n-helpers).

See
[mdbook-i18n-helpers usage guide](https://github.com/google/mdbook-i18n-helpers/blob/main/i18n-helpers/USAGE.md)
for help.

Adding a new translations is somewhat involved, so feel free to start
translation and open a pull request, even if your translation is incomplete.

Take a look at
[this commit](https://github.com/ordinals/ord/commit/329f31bf6dac207dad001507dd6f18c87fdef355)
for an example of adding a new translation. A maintainer will help you integrate it
into our build system.

To start a new translation:

1. Install `mdbook`, `mdbook-i18n-helpers`, and `mdbook-linkcheck`:

   ```
   cargo install mdbook mdbook-i18n-helpers mdbook-linkcheck
   ```

2. Generate a new `pot` file named `messages.pot`:

   ```
   MDBOOK_OUTPUT='{"xgettext": {"pot-file": "messages.pot"}}'
   mdbook build -d po
   ```

3. Run `msgmerge` on `XX.po` where `XX` is the two-letter
   [ISO-639](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) code for
   the language you are translating into. This will update the `po` file with
   the text of the most recent English version:

   ```
   msgmerge --update po/XX.po po/messages.pot
   ```

4. Untranslated sections are marked with `#, fuzzy` in `XX.po`. Edit the
   `msgstr` string with the translated text.

5. Execute the `mdbook` command to rebuild the docs. For Chinese, whose
   two-letter ISO-639 code is `zh`:

   ```
   mdbook build docs -d build
   MDBOOK_BOOK__LANGUAGE=zh mdbook build docs -d build/zh
   mv docs/build/zh/html docs/build/html/zh
   python3 -m http.server --directory docs/build/html --bind 127.0.0.1 8080
   ```

6. If everything looks good, commit `XX.po` and open a pull request on GitHub.
   Other changed files should be omitted from the pull request.

# Problem Statement and Solution Domain

---

## 14. Problem Domain

Bitcoin provides:

* An immutable, totally ordered ledger
* Strong economic finality
* A minimal, intentionally non-expressive execution environment

However, **Bitcoin intentionally lacks**:

* Native programmable state
* Contract-level execution
* Identity-aware semantics
* Cross-domain message passing

BRC-20 emerged as an attempt to represent fungible assets atop Bitcoin’s inscription mechanism.
While functional, the current design exhibits **structural limitations** that preclude scalability, verifiability, and protocol evolution.

---

## 15. Failure Modes of Existing BRC-20 Systems

### 15.1 Indexer Authority

**Problem**

State interpretation depends on centralized indexers applying informal parsing rules.

**Consequences**

* Divergent balances across indexers
* Non-replayable state
* Hidden rule changes
* Protocol capture risk

---

### 15.2 Event-Based Semantics

**Problem**

BRC-20 treats inscriptions as *events*, not *state transitions*.

**Consequences**

* No canonical “current state”
* Inability to reason about correctness
* No invariant enforcement
* No proof of validity beyond trust

---

### 15.3 Lack of Deterministic Failure

**Problem**

Invalid operations are silently ignored or inconsistently applied.

**Consequences**

* Undefined behavior
* Non-deterministic outcomes
* Client disagreement
* Inability to reason formally

---

### 15.4 No Constraint Encoding

**Problem**

Transfer rules, vesting, supply caps, or governance logic cannot be expressed natively.

**Consequences**

* All logic offloaded to indexers
* No cryptographic enforcement
* No composability
* No formal verification

---

### 15.5 Absence of Identity Semantics

**Problem**

All addresses are treated equivalently.

**Consequences**

* No soulbound assets
* No reputation
* No access control
* No revocation semantics

---

### 15.6 Cross-Chain Trust Assumptions

**Problem**

Existing bridges rely on:

* Custodians
* Federations
* Wrapped representations

**Consequences**

* Counterparty risk
* Liquidity fragmentation
* Bridge insolvency failures

---

### 15.7 Economic Misalignment

**Problem**

Protocol activity does not necessarily strengthen Bitcoin.

**Consequences**

* Blockspace consumption without security contribution
* Miner misalignment
* Long-term sustainability risk

---

## 16. Design Constraints

Any solution operating atop Bitcoin MUST:

1. Preserve Bitcoin’s consensus rules
2. Avoid trusted intermediaries
3. Be replayable from genesis
4. Admit light-client verification
5. Remain execution-free on-chain
6. Align incentives with miners

---

## 17. Solution Overview

BRC-20 v2 resolves the above by **reframing the problem**.

The protocol does **not** attempt to add execution to Bitcoin.
Instead, it introduces **provable state evolution**.

---

## 18. Core Solutions

---

### 18.1 State Machines over Events

**Resolution**

All token behavior is defined as deterministic state transitions:

```
Sₙ = T(Sₙ₋₁, P, C)
```

Where:

* `S` is state
* `P` is a proof
* `C` is contextual Bitcoin data

**Result**

* Canonical state
* Replayability
* Formal reasoning
* Stateless clients

---

### 18.2 Cryptographic Validity Proofs

**Resolution**

Transition correctness is attested via zero-knowledge proofs.

**Result**

* No execution on Bitcoin
* Arbitrary logic off-chain
* On-chain commitments only
* Verifiable correctness

---

### 18.3 Canonical Serialization & Hashing

**Resolution**

All protocol objects are canonically serialized prior to hashing.

**Result**

* Indexer independence
* Deterministic replay
* No hidden interpretation layers

---

### 18.4 Identity as a Constraint, Not Metadata

**Resolution**

Identity enters the system as a cryptographic commitment, not a label.

**Result**

* Soulbound assets
* Conditional transfers
* Revocation & expiry
* Privacy preservation

---

### 18.5 Explicit Failure Semantics

**Resolution**

All invalid transitions resolve to deterministic failure states.

**Result**

* No silent errors
* Verifiable rejection
* Client agreement
* Formal safety analysis

---

### 18.6 Proof-Carrying Cross-Domain Messages

**Resolution**

Bitcoin becomes the root of truth for other chains via proof export.

**Result**

* No wrapped assets
* No custody
* No federations
* Trust-minimized interoperability

---

### 18.7 Time as a First-Class Primitive

**Resolution**

Block height replaces wall-clock time.

**Result**

* Native vesting
* Epoch governance
* Deterministic scheduling
* No oracle dependencies

---

### 18.8 Economic Alignment

**Resolution**

All activity consumes Bitcoin blockspace and fees.

**Result**

* Miner incentive alignment
* Post-subsidy security contribution
* Sustainable protocol usage

---

## 19. What This Enables That Was Previously Impossible

| Capability                | Legacy BRC-20 | BRC-20 v2 |
| ------------------------- | ------------- | --------- |
| Stateless verification    | ❌             | ✅         |
| Deterministic replay      | ❌             | ✅         |
| Supply invariants         | ❌             | ✅         |
| Vesting                   | ❌             | ✅         |
| Soulbound tokens          | ❌             | ✅         |
| Identity constraints      | ❌             | ✅         |
| Cross-chain trustless use | ❌             | ✅         |
| Miner-aligned economics   | ❌             | ✅         |

---

## 20. Non-Goals

BRC-20 v2 intentionally does **not**:

* Introduce smart contracts on Bitcoin
* Modify Bitcoin consensus
* Compete with Ethereum-style execution
* Replace Layer-2 systems

It is a **state and proof protocol**, not a VM.

---

## 21. Summary

The problem is **not** that Bitcoin lacks programmability.
The problem is that existing protocols lack **verifiable state evolution**.

BRC-20 v2 resolves this by:

* Making state explicit
* Making correctness provable
* Making identity enforceable
* Making time deterministic
* Making bridges trustless

Bitcoin becomes not a smart contract platform, but a **cryptographic court of final appeal**.
