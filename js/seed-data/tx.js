import {TransactionMessage, VersionedTransaction} from "@solana/web3.js";

export const signAndSend = async(connection, tx, signers) => {
  let latestBlockhash = await connection.getLatestBlockhash("confirmed");

  // Step 1 - Sign your transaction with the required `Signers`
  tx.sign(signers);
  
  // Step 2 - Send our v0 transaction to the cluster
  const txid = await connection.sendTransaction(tx, {maxRetries: 5, skipPreflight: false});

  // Step 3 - Confirm Transaction
  const confirmation = await connection.confirmTransaction({
    signature: txid,
    blockhash: latestBlockhash.blockhash,
    lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
  });

  if (confirmation.value.err) {
    throw new Error(`   ❌ - Transaction not confirmed.\nReason: ${confirmation.value.err}`);
  }
}

/// Create and send a versioned transaction
export const createAndSendV0Tx = async (
  connection,
  instructions,
  payerKey,
  signers,
  addressLUTs = [],
) => {
  // 1 - Fetch the latest blockhash
  let latestBlockhash = await connection.getLatestBlockhash("confirmed");

  // 2 - Generate Transaction Message
  const messageV0 = new TransactionMessage({
    payerKey,
    recentBlockhash: latestBlockhash.blockhash,
    instructions,
  }).compileToV0Message(addressLUTs);
  const tx = new VersionedTransaction(messageV0);

  // 3 -Sign and send
  await signAndSend(connection, tx, signers);
}
