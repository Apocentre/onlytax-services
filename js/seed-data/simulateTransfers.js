import {HDKey} from "micro-ed25519-hdkey";
import * as bip39 from "bip39";
import {Keypair, PublicKey, Connection} from "@solana/web3.js"
import {
  TOKEN_2022_PROGRAM_ID,
  transferChecked,
  createAssociatedTokenAccountIdempotent,
  getAssociatedTokenAddress
} from '@solana/spl-token';
import config from "./.config.json" assert { type: "json" };

const connection = new Connection(config.rpc, "confirmed");
const rootWallet = Keypair.fromSecretKey(Buffer.from(config.rootWallet));
const mint = new PublicKey("9NxTF8W3gB1y49LBn1GTp5QqPmkdp4P8HJDiqJgJQSUB");
const decimals = 6;

const main = async () => {
  const testAccounts = createTestAccounts();

  // transfer chain: rootWallet -> testAccounts[0] -> testAccounts[1] -> testAccounts[2]...
  await tranfer(rootWallet, testAccounts[0], BigInt(1000 * Math.pow(10, 6)))

  for(let i = 0; i < 99; i++) {
    const from = testAccounts[i]
    const to = testAccounts[i + 1]; 

    await tranfer(from, to)
  }
}

const createTestAccounts = () => {
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

const createAtaIfNedded = async (account) => {
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
      rootWallet, // root is always the payer
      mint,
      account.publicKey,
      {},
      TOKEN_2022_PROGRAM_ID,
    );
  }

  return ata;
}


let count = 1;

const tranfer = async (from, to, amount) => {
  const fromAta = await createAtaIfNedded(from);
  const toAta = await createAtaIfNedded(to);
  let amountToSend;
  
  if(!amount) {
    const fromTokenBalance = await connection.getTokenAccountBalance(fromAta);
    const balance = BigInt(fromTokenBalance.value.amount);
    // send 50% of the total balance
    amountToSend = BigInt((balance * BigInt(50) / BigInt(100)));
  } else {
    amountToSend = amount;
  }

  await transferChecked(
    connection,
    rootWallet, // root is always the payer
    fromAta,
    mint,
    toAta,
    from.publicKey,
    amountToSend,
    decimals,
    [rootWallet, from],
    null,
    TOKEN_2022_PROGRAM_ID,
  );

  console.log(`[${count++}] Transfered ${amountToSend} from ${fromAta} to ${toAta}`)
}

main()
.then(() => console.log("Success"))
.catch((error) => console.log("Error: ", error))
