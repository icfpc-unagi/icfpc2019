# lambda-client

## Requirements

### Python 3.6 or later

To install, you need Python 3.6 or later and PIP.

If you are using Windows, make sure to tick the 'Add Python to environment
variables' tickbox under Advanced Options when you install Python, or otherwise
add Python to your system `PATH`.


Then:

```
pip3 install -r requirements.txt
```

## Configuration

Edit `lambda.conf` and fill in your private and public IDs:

```
[SECRET]
PrivateKey = <your private id>
PublicKey = <your public id>
```

**NB**: if you change `lambda.conf`, you must restart the daemon process for the
changes to take effect.

## Usage

### Basic usage

First, run the daemon:

```
./lambdad.py
```

Then, in a separate terminal, you can execute `lambda-cli.py` commands. For
example:

```
./lambda-cli.py getblockchaininfo
```

To get help, run `./lambda-cli.py --help` or `./lambda-cli.py <subcommand> --help`.

If you are running Windows and `./lambda-cli.py getblockchainfo` creates a
console that disappears immediately, run:

```
python.exe lambda-cli.py getblockchainfo
```

### Accessing JSON fields

With any command that returns a JSON output, you can access fields in the output
directly from `lambda-cli.py`. For example:

```
./lambda-cli.py getmininginfo puzzle
./lambda-cli.py getblockchaininfo block_ts
./lambda-cli.py getbalances 42
```

### Historic blocks

The `getblockchaininfo`, `getmininginfo`, `getbalances` and `getbalance`
commands return information pertaining to the _current block_. To retrieve
information about previous blocks, use the `getblockinfo` subcommand. For
example:

```
# Get info about current block
./lambda-cli.py getblockinfo
# Get info about previous blocks (1 and 3)
./lambda-cli.py getblockinfo 1
./lambda-cli.py getblockinfo 1 block_ts
./lambda-cli.py getblockinfo 3 balances 42

```

#### `blocks/` folder on disk

Your `lambdad.py` daemon also saves the output of `getblockinfo` on disk, in the
`blocks/` folder relative to where `./lambdad.py` is called from. (You can
change `DataDir` in `lambda.conf` if you want to place it somewhere else.) You
might find this easier to integrate into your workflow than the JSON-RPC
interface (explained below).

### Checking your own balance

If you have set-up your `PublicKey` correctly in `lambda.conf`, running
`./lambda-cli.py getbalance` will return _your_ current balance.

If you set a different team's `PublicKey`, your submissions will still work, but
running `getbalance` will return that team's balance rather than your own.

### Submitting solutions to block task and puzzle

Similarly, if you have set-up your `PrivateKey`, you can use `lambda-cli.py` to
submit solutions to the block task and puzzle:

```
./lambda-cli.py submit --help
# the actual file names don't matter; file extensions do (to help you catch mistakes)
./lambda-cli.py submit 3 path_to_task.sol path_to_puzzle_sol.desc
```

**NB**: paths passed to `./lambda-cli.py submit` are interpreted relative to the
daemon process, not to the CLI process. To keep things simple, we recommend
running both in the same directory.

Note that providing the block number you are submitting for is **mandatory**.
This ensures you always submit to the block you intend and helps prevent
mistakes, since submissions are final.

## JSON-RPC

You do not have to use `lambda-cli.py` (although we recommend it). The
`lambdad.py` process exposes a standard
[JSON-RPC](https://www.jsonrpc.org/specification) interface on port 8332.

Your favourite programming language likely has a library to interact with
JSON-RPC servers, but we cannot provide any support with that. However, here are
some examples of how to talk to the JSON-RPC `lambdad.py` interface via cURL:

```
curl --data-binary '{"jsonrpc":"2.0","id":"curl","method":"getblockchaininfo","params":[]}' -H 'content-type:text/plain;' http://127.0.0.1:8332/

curl --data-binary '{"jsonrpc":"2.0","id":"curl","method":"getbalance","params":[42]}' -H 'content-type:text/plain;' http://127.0.0.1:8332/

curl --data-binary '{"jsonrpc":"2.0","id":"curl","method":"getblockinfo","params":[1]}' -H 'content-type:text/plain;' http://127.0.0.1:8332/
```

The return value of the called method is in the `result` item of the output.

If you end up using some clever JSON-RPC contraption, do let us know in the
`README.md` of your final submission! Happy hacking :-)
