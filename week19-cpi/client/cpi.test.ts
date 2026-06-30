import { test } from "bun:test";
import assert from "node:assert/strict";
import { FailedTransactionMetadata, LiteSVM } from "litesvm";
import { getCreateAccountInstruction, getTransferSolInstruction } from "@solana-program/system";
import {
    AccountRole,
    appendTransactionMessageInstruction,
    createTransactionMessage,
    generateKeyPairSigner,
    lamports,
    pipe,
    setTransactionMessageFeePayerSigner,
    setTransactionMessageLifetimeUsingBlockhash,
    signTransactionMessageWithSigners,
    type KeyPairSigner,
    type Transaction,
} from "@solana/kit";

const verifyamount = (svm: LiteSVM, account: KeyPairSigner<string>, data: number[]) => {
    let newData = svm.getAccount(account.address);

    if (!newData.exists) {
        throw Error("The data account is not found")
    }
    else {
        for (let i = 0; i < data.length; i++) {
            assert(newData.data[i] === data[i])
        }
    }
}

const sendTransactionAndVerify = (svm: LiteSVM, transaction: Transaction) => {
    const result = svm.sendTransaction(transaction);

    svm.expireBlockhash();

    if (result instanceof FailedTransactionMetadata) {
        throw new Error(`Transaction failed: ${result.err()}`);
    }
}

test("it transfers SOL from one wallet to another", async () => {
    // Given a payer with 2 SOL and a recipient with 0 SOL.
    const svm = new LiteSVM();
    const contractPubkey = await generateKeyPairSigner();
    svm.addProgramFromFile(contractPubkey.address, "/Users/shivamgupta/Code/Rust-Assignments/week19-cpi/client/double.so");

    const cpiPubkey = await generateKeyPairSigner();
    svm.addProgramFromFile(cpiPubkey.address, "/Users/shivamgupta/Code/Rust-Assignments/week19-cpi/client/cpi.so");

    const payer = await generateKeyPairSigner();
    const dataAccount = await generateKeyPairSigner();
    svm.airdrop(payer.address, lamports(2_000_000_000n));

    // When we send 1 SOL from the payer to the recipient.
    const instruction = getCreateAccountInstruction({
        newAccount: dataAccount,
        payer: payer,
        programAddress: contractPubkey.address,
        lamports: Number(svm.minimumBalanceForRentExemption(BigInt(4))),
        space: 4
    });
    const transaction = await pipe(
        createTransactionMessage({ version: 0 }),
        (tx) => setTransactionMessageFeePayerSigner(payer, tx),
        (tx) => svm.setTransactionMessageLifetimeUsingLatestBlockhash(tx),
        (tx) => appendTransactionMessageInstruction(instruction, tx),
        (tx) => signTransactionMessageWithSigners(tx),
    );

    sendTransactionAndVerify(svm, transaction);

    //both are same
    verifyamount(svm, dataAccount, [0, 0, 0, 0])
    verifyamount(svm, dataAccount, [0])


    const contract_transaction = {
        programAddress: cpiPubkey.address,
        accounts: [
            { address: dataAccount.address, role: AccountRole.WRITABLE },
            { address: contractPubkey.address, role: AccountRole.WRITABLE },
        ],
        data: new Uint8Array([]),
        payer
    };

    const transaction2 = await pipe(
        createTransactionMessage({ version: 0 }),
        (tx) => setTransactionMessageFeePayerSigner(payer, tx),
        (tx) => svm.setTransactionMessageLifetimeUsingLatestBlockhash(tx),
        (tx) => appendTransactionMessageInstruction(contract_transaction, tx),
        (tx) => signTransactionMessageWithSigners(tx),
    );

    sendTransactionAndVerify(svm, transaction2);

    verifyamount(svm, dataAccount, [1, 0, 0, 0])

    const contract_transaction3 = {
        programAddress: cpiPubkey.address,
        accounts: [
            { address: dataAccount.address, role: AccountRole.WRITABLE },
            { address: contractPubkey.address, role: AccountRole.WRITABLE },
        ],
        data: new Uint8Array([]),
        payer
    };

    const transaction3 = await pipe(
        createTransactionMessage({ version: 0 }),
        (tx) => setTransactionMessageFeePayerSigner(payer, tx),
        (tx) => svm.setTransactionMessageLifetimeUsingLatestBlockhash(tx),
        (tx) => appendTransactionMessageInstruction(contract_transaction3, tx),
        (tx) => signTransactionMessageWithSigners(tx),
    );

    sendTransactionAndVerify(svm, transaction3);

    verifyamount(svm, dataAccount, [2, 0, 0, 0])

    // Then we expect the accounts to have the correct balances.
    const payerBalance = svm.getBalance(payer.address);

    assert.strictEqual(
        svm.getBalance(dataAccount.address),
        svm.minimumBalanceForRentExemption(BigInt(4)),
    );
    // console.log(svm.minimumBalanceForRentExemption(BigInt(4)));
    assert(payerBalance !== null);
    assert(payerBalance < lamports(2_000_000_000n));
});