import {HDKey} from "micro-ed25519-hdkey";
import * as bip39 from "bip39";
import {Keypair, Connection} from "@solana/web3.js"
import {
  TOKEN_2022_PROGRAM_ID,
  transferChecked,
  createAssociatedTokenAccountIdempotent,
  getAssociatedTokenAddress
} from '@solana/spl-token';
import config from "./.config.json" assert { type: "json" };

const connection = new Connection(config.rpc, "confirmed");

export const createTestAccounts = () => {
  // created via `bip39.generateMnemonic()`
  const mnemonic = "drastic retreat orient any silk radio gasp live sphere fee quick";
  const seed = bip39.mnemonicToSeedSync(mnemonic, "");
  const hd = HDKey.fromMasterSeed(seed.toString("hex"));
  const accounts = [];

  for (let i = 0; i < 100; i++) {
    const path = `m/44'/501'/${i}'/0'`;
    const keypair = Keypair.fromSeed(hd.derive(path).privateKey);
    accounts.push(keypair);
  }

  return accounts;
}

export const createAtaIfNeeded = async (payer, mint, account) => {
  const ata = await getAssociatedTokenAddress(
    mint,
    account.publicKey,
    true,
    TOKEN_2022_PROGRAM_ID
  );

  const accountInfo = await connection.getAccountInfo(ata);

  if(!accountInfo) {
    await createAssociatedTokenAccountIdempotent(
      connection,
      payer, // root is always the payer
      mint,
      account.publicKey,
      {},
      TOKEN_2022_PROGRAM_ID,
    );
  }

  return ata;
}

let count = 1;

export const tranfer = async (mint, decimals, payer, from, to, amount) => {
  const fromAta = await createAtaIfNeeded(payer, mint, from);
  const toAta = await createAtaIfNeeded(payer, mint, to);
  let amountToSend;
  
  if(!amount) {
    const fromTokenBalance = await connection.getTokenAccountBalance(fromAta);
    const balance = BigInt(fromTokenBalance.value.amount);
    // send 90% of the total balance
    amountToSend = BigInt((balance * BigInt(90) / BigInt(100)));
  } else {
    amountToSend = amount;
  }

  await transferChecked(
    connection,
    payer,
    fromAta,
    mint,
    toAta,
    from.publicKey,
    amountToSend,
    decimals,
    [payer, from],
    null,
    TOKEN_2022_PROGRAM_ID,
  );

  console.log(`[${count++}] Transfered ${amountToSend} from ${fromAta} to ${toAta}`)
}
