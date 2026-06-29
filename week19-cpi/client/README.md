# client

To install dependencies:

```bash
bun install
```

To run:

```bash
bun run index.ts
```

This project was created using `bun init` in bun v1.3.9. [Bun](https://bun.com) is a fast all-in-one JavaScript runtime.



This test uses the new Solana JS stack — not @solana/web3.js or Anchor’s client. Here’s a full walkthrough, with comparisons to what you already know.

What’s “new” here?
Old (@solana/web3.js)	New (this file)
One big @solana/web3.js package
@solana/kit — modular SDK from Anza (official successor)
Keypair.generate()
generateKeyPairSigner()
PublicKey
Address (often accessed as .address on signers)
SystemProgram.transfer({...})
getTransferSolInstruction({...}) from @solana-program/system
Transaction + manual .sign()
Transaction message pipeline + signTransactionMessageWithSigners()
Connection + solana-test-validator
LiteSVM — in-process mini-validator for tests
Jest / Vitest
Bun test (bun:test)
The mental model is the same (accounts, instructions, transactions, lamports), but the API is more typed, composable, and split into small packages.

The three libraries in play
1. @solana/kit
The main SDK. It re-exports building blocks for:

addresses, keys, signers
transaction messages and signing
RPC (not used in this test — LiteSVM replaces a real RPC)
2. @solana-program/system
Program-specific helpers for the System Program (native SOL transfers, account creation, etc.).
Like having typed wrappers instead of manually building SystemProgram.transfer instructions.

3. litesvm
A lightweight in-memory Solana runtime for tests. Think of it as:

no solana-test-validator process
no Connection
instant airdrops, balance checks, and transaction execution
It depends on @solana/kit internally, which is why those imports work even though package.json only lists litesvm.

Line-by-line breakdown
Imports

index.test.ts
Lines 1-14
import { test } from "bun:test";
import assert from "node:assert/strict";
import { FailedTransactionMetadata, LiteSVM } from "litesvm";
import { getTransferSolInstruction } from "@solana-program/system";
import {
    appendTransactionMessageInstruction,
    createTransactionMessage,
    generateKeyPairSigner,
    lamports,
    pipe,
    setTransactionMessageFeePayerSigner,
    setTransactionMessageLifetimeUsingBlockhash,
    signTransactionMessageWithSigners,
} from "@solana/kit";
bun:test — Bun’s built-in test runner (like Jest’s test()).
FailedTransactionMetadata — returned when a tx fails; you check instanceof instead of catching RPC errors.
getTransferSolInstruction — builds a System Program “transfer SOL” instruction (replaces SystemProgram.transfer).
lamports() — wraps a bigint as a typed lamport amount (1 SOL = 1_000_000_000 lamports).
pipe — functional helper to chain steps left-to-right (like lodash flow).
generateKeyPairSigner — creates a keypair that is also a Signer (knows how to sign).
Transaction helpers — build a v0 transaction message step by step.
Note: setTransactionMessageLifetimeUsingBlockhash is imported but unused; LiteSVM provides its own variant on the next line.

Setup: local chain + accounts

index.test.ts
Lines 16-23
test("it transfers SOL from one wallet to another", async () => {
    // Given a payer with 2 SOL and a recipient with 0 SOL.
    const svm = new LiteSVM();
    const contractPubkey = await generateKeyPairSigner();
    svm.addProgramFromFile(contractPubkey.address, "./double.so");
    const payer = await generateKeyPairSigner();
    const dataAccount = await generateKeyPairSigner();
    svm.airdrop(payer.address, lamports(2_000_000_000n));
new LiteSVM()
Creates an empty in-memory blockchain. No RPC URL, no validator binary.

generateKeyPairSigner()
Unlike Keypair.generate() which gives { publicKey, secretKey }, a Signer is an object with:

.address — the public key
signing capability baked in
You can pass the signer itself into instruction builders when it’s the source account.

addProgramFromFile(contractPubkey.address, "./double.so")
Loads your compiled Rust program (double.so) at a program ID you choose (contractPubkey.address).
In web3.js terms: deploy a program to a known address in the test VM.

Your Rust program (lib.rs) doubles a counter in an account — but this test doesn’t call it yet; it only loads it for later CPI tests.

payer and dataAccount
Two new keypairs. The comment calls dataAccount a “recipient” — it’s just the destination wallet for the SOL transfer.

svm.airdrop(payer.address, lamports(2_000_000_000n))
Funds the payer with 2 SOL. In web3.js you’d use connection.requestAirdrop(). Here it’s synchronous/instant.

Building the transfer instruction

index.test.ts
Lines 25-30
    const instruction = getTransferSolInstruction({
        source: payer,
        destination: dataAccount.address,
        amount: lamports(1_000_000_000n),
    });
This is the Kit equivalent of:

// old web3.js
SystemProgram.transfer({
  fromPubkey: payer.publicKey,
  toPubkey: recipient.publicKey,
  lamports: 1_000_000_000,
})
Differences:

source: payer — pass the full signer (needed for signing later).
destination: dataAccount.address — only the address is needed for the recipient.
lamports(1_000_000_000n) — bigint with a type tag, not a plain number.
Building and signing the transaction (the pipe pattern)

index.test.ts
Lines 31-37
    const transaction = await pipe(
        createTransactionMessage({ version: 0 }),
        (tx) => setTransactionMessageFeePayerSigner(payer, tx),
        (tx) => svm.setTransactionMessageLifetimeUsingLatestBlockhash(tx),
        (tx) => appendTransactionMessageInstruction(instruction, tx),
        (tx) => signTransactionMessageWithSigners(tx),
    );
In web3.js you often do:

const tx = new Transaction().add(instruction);
tx.feePayer = payer.publicKey;
tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
tx.sign(payer);
Kit splits that into explicit, typed steps:

Step	What it does
createTransactionMessage({ version: 0 })
Start a v0 transaction message (empty shell)
setTransactionMessageFeePayerSigner(payer, tx)
Set who pays fees; attach payer as signer
svm.setTransactionMessageLifetimeUsingLatestBlockhash(tx)
Set recent blockhash from LiteSVM (tx expiry)
appendTransactionMessageInstruction(instruction, tx)
Add the transfer instruction
signTransactionMessageWithSigners(tx)
Sign with all signers referenced in the message (payer, because it’s fee payer + transfer source)
pipe(initial, fn1, fn2, ...) feeds each step’s output into the next. Same idea as:

let tx = createTransactionMessage({ version: 0 });
tx = setTransactionMessageFeePayerSigner(payer, tx);
// ...
The big design win: signers travel with the message. You don’t manually call .sign() on a keypair list — Kit finds who needs to sign from the message itself.

Sending and checking for failure

index.test.ts
Lines 39-42
    const result = svm.sendTransaction(transaction);
    if (result instanceof FailedTransactionMetadata) {
        throw new Error(`Transaction failed: ${result.err()}`);
    }
svm.sendTransaction(transaction) — executes the tx in the VM (no network, no sendAndConfirmTransaction).

Return type is a union:

success metadata on success
FailedTransactionMetadata on failure
So you branch with instanceof instead of try/catch on RPC errors.

Assertions

index.test.ts
Lines 44-52
    const payerBalance = svm.getBalance(payer.address);
    assert.strictEqual(
        svm.getBalance(dataAccount.address),
        lamports(1_000_000_000n),
    );
    assert(payerBalance !== null);
    assert(payerBalance < lamports(1_000_000_000n));
dataAccount should have exactly 1 SOL (1 billion lamports).
payer should have less than 1 SOL left — not exactly 1 SOL, because they also paid transaction fees (same as on mainnet/devnet).
In web3.js you’d use connection.getBalance(). Here: svm.getBalance(address).

End-to-end flow (diagram)
LiteSVM
Create signers
Airdrop 2 SOL to payer
Build transfer instruction
Pipe: message + fee payer + blockhash + instruction
Sign with signers
svm.sendTransaction
Assert balances
web3.js vs Kit — same transaction, side by side
Old:

const connection = new Connection("http://127.0.0.1:8899");
const payer = Keypair.generate();
const recipient = Keypair.generate();
await connection.requestAirdrop(payer.publicKey, 2e9);
const ix = SystemProgram.transfer({
  fromPubkey: payer.publicKey,
  toPubkey: recipient.publicKey,
  lamports: 1e9,
});
const { blockhash } = await connection.getLatestBlockhash();
const tx = new Transaction({ feePayer: payer.publicKey, recentBlockhash: blockhash })
  .add(ix);
tx.sign(payer);
await connection.sendRawTransaction(tx.serialize());
New (this test):

const svm = new LiteSVM();
const payer = await generateKeyPairSigner();
const recipient = await generateKeyPairSigner();
svm.airdrop(payer.address, lamports(2_000_000_000n));
const instruction = getTransferSolInstruction({
  source: payer,
  destination: recipient.address,
  amount: lamports(1_000_000_000n),
});
const transaction = await pipe(
  createTransactionMessage({ version: 0 }),
  (tx) => setTransactionMessageFeePayerSigner(payer, tx),
  (tx) => svm.setTransactionMessageLifetimeUsingLatestBlockhash(tx),
  (tx) => appendTransactionMessageInstruction(instruction, tx),
  (tx) => signTransactionMessageWithSigners(tx),
);
svm.sendTransaction(transaction);
Same on-chain effect; Kit is more explicit and type-safe.

Context for your week19 CPI assignment
This file looks like a starter template before CPI (Cross-Program Invocation) work:

double.so — your Rust program that doubles a counter in account data.
The test loads that program but only transfers SOL so far.
Next steps would likely: fund/create a data account, build an instruction that calls your program, and maybe use CPI from another program.
The Rust side expects an account with Borsh-serialized { count: u32 } and doubles it on each invoke.

Practical tips as you learn Kit
Signers vs addresses — pass Signer when the account must sign; pass .address when it’s read-only or a destination.
lamports() always uses bigint — 1_000_000_000n, not 1e9.
Program helpers live in @solana-program/* — system, token, etc., not in the core kit package.
LiteSVM is test-only — for mainnet/devnet you’d swap svm.sendTransaction for an RPC client from @solana/kit (createSolanaRpc, etc.).
Run with bun test in the client/ folder.


I'll check your test file and README for how this project invokes the local program so the steps match your setup.

You're already most of the way there. You’ve loaded the program and created a data account — the missing piece is building an instruction that targets your program and sending it in a transaction.

Here’s the full pattern for your double.so program.

Mental model
Calling a Solana program always means sending a transaction with an instruction that has three parts:

Field	Your program
programAddress
contractPubkey.address (where you loaded double.so)
accounts
The accounts your Rust code reads, in the same order
data
Instruction bytes (your program ignores this — empty is fine)
Your Rust code expects one writable account:


lib.rs
Lines 16-22
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let mut account1 = accounts.iter();
    let data_account = next_account_info(&mut account1)?;
So the client instruction must pass dataAccount as account #0, marked writable.

Step 1 — Load the program (you already have this)
const svm = new LiteSVM();
const contractPubkey = await generateKeyPairSigner();
svm.addProgramFromFile(contractPubkey.address, "./double.so");
This is like deploying the .so to a known program ID inside LiteSVM.

Step 2 — Create the data account (you already have this)
Your program reads a Borsh { count: u32 } from account data, so you need a 4-byte account owned by your program:

const createAccountIx = getCreateAccountInstruction({
    newAccount: dataAccount,           // must sign (new keypair)
    payer: payer,
    programAddress: contractPubkey.address,  // owner = your program
    lamports: Number(svm.minimumBalanceForRentExemption(4n)),
    space: 4,
});
Fresh account data is all zeros → count = 0 → first invoke sets it to 1.

Step 3 — Build the program invoke instruction
This is what you started sketching around line 41. In Kit it’s a plain Instruction object — not byteDelta:

import { AccountRole } from "@solana/kit";
const invokeDoubleIx = {
    programAddress: contractPubkey.address,
    accounts: [
        {
            address: dataAccount.address,
            role: AccountRole.WRITABLE,  // program writes to this account
        },
    ],
    data: new Uint8Array([]),  // your Rust ignores instruction_data
};
Account roles cheat sheet:

Role	Meaning
AccountRole.READONLY
Passed in, not modified
AccountRole.WRITABLE
Passed in, program can write
AccountRole.READONLY_SIGNER
Must sign, read-only
AccountRole.WRITABLE_SIGNER
Must sign + writable
Your dataAccount does not need to sign the invoke — only the payer signs (as fee payer).

Step 4 — Send it in a transaction
Same pipe pattern as the create-account tx. You can do one tx with both instructions or two separate txs.

Option A: one transaction (create + invoke)
const transaction = await pipe(
    createTransactionMessage({ version: 0 }),
    (tx) => setTransactionMessageFeePayerSigner(payer, tx),
    (tx) => svm.setTransactionMessageLifetimeUsingLatestBlockhash(tx),
    (tx) => appendTransactionMessageInstruction(createAccountIx, tx),
    (tx) => appendTransactionMessageInstruction(invokeDoubleIx, tx),
    (tx) => signTransactionMessageWithSigners(tx),
);
const result = svm.sendTransaction(transaction);
if (result instanceof FailedTransactionMetadata) {
    throw new Error(`Transaction failed: ${result.err()}`);
}
signTransactionMessageWithSigners automatically signs with payer and dataAccount (needed for create account).

Option B: two transactions (what you have now + a second tx)
Send the create-account tx first, then build a second tx with only invokeDoubleIx.

Step 5 — Read the result
const account = svm.getAccount(dataAccount.address);
if (!account.exists) throw new Error("account missing");
// Borsh u32 little-endian (count field)
const count = new DataView(account.data.buffer, account.data.byteOffset, 4)
    .getUint32(0, true);
assert.strictEqual(count, 1);  // first invoke: 0 → 1
Call invoke again and count becomes 2, then 4, etc.

Full flow diagram
addProgramFromFile(double.so)
Tx1 - createAccount (payer pays, dataAccount signs)
allocate 4 bytes, owner = double program
Tx2 - invoke double program
accounts=[dataAccount writable]
read count, double it, write back
getAccount(dataAccount) → read count
Test
LiteSVM
SystemProgram
DoubleProgram
web3.js equivalent (for reference)
If this were old @solana/web3.js:

const ix = new TransactionInstruction({
  keys: [{ pubkey: dataAccount.publicKey, isSigner: false, isWritable: true }],
  programId: programId,
  data: Buffer.alloc(0),
});
Kit’s { programAddress, accounts, data } is the same thing, just typed.

Fix for your current draft (lines 41–50)
Replace the incomplete contract_transaction with:

const invokeDoubleIx = {
    programAddress: contractPubkey.address,
    accounts: [
        { address: dataAccount.address, role: AccountRole.WRITABLE },
    ],
    data: new Uint8Array([]),
};
Then append it to a transaction and call svm.sendTransaction(...).

Connecting to a real local validator (not LiteSVM)
If you later use solana-test-validator instead of LiteSVM:

Deploy: solana program deploy target/deploy/double.so
Create RPC: createSolanaRpc("http://127.0.0.1:8899")
Build the same instruction object
Send via RPC helpers instead of svm.sendTransaction
The instruction shape is identical — only the transport changes (svm vs RPC).

Common gotchas
Account order must match Rust — your program only reads account index 0.
Owner must be your program — create account with programAddress: contractPubkey.address.
Space must be 4 bytes — one Borsh u32.
Writable flag — program mutates account data, so use AccountRole.WRITABLE.
Create before invoke — program can’t write to an account that doesn’t exist yet.
If you want, I can wire this directly into your index.test.ts so the test creates the account, invokes double.so, and asserts count === 1.