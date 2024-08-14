// Import necessary functions and constants from the Solana web3.js and SPL Token packages
// https://www.quicknode.com/guides/solana-development/spl-tokens/token-2022/transfer-fees

import {
  Connection,
  Keypair,
  SystemProgram,
  PublicKey,
  ComputeBudgetProgram,
} from "@solana/web3.js";
import {
  ExtensionType,
  createInitializeMintInstruction,
  createMintToInstruction,
  getMintLen,
  TOKEN_2022_PROGRAM_ID,
  createInitializeTransferFeeConfigInstruction,
  createAssociatedTokenAccountIdempotent,
} from "@solana/spl-token";
import config from "./.config.json" assert { type: "json" };
import {createAndSendV0Tx} from "./tx.js"

const {
  rpc, rootWallet, transferFeeConfigAuthority,
  withdrawWithheldAuthority,
} = config;

// Generate keys for payer, mint authority, and mint
const payer = Keypair.fromSecretKey(Buffer.from(rootWallet));

const mintAuthority = Keypair.fromSecretKey(Buffer.from(rootWallet));
const mintKeypair = Keypair.generate()
const mint = mintKeypair.publicKey;

// Define the extensions to be used by the mint
const extensions = [
  ExtensionType.TransferFeeConfig,
];

// Calculate the length of the mint
const mintLen = getMintLen(extensions);

// Set the decimals, fee basis points, and maximum fee
const decimals = 6;
const feeBasisPoints = 500; // 5%
const maxFee = BigInt("18446744073709551615"); // max u64
const mintAmount = BigInt(8_000_000_000 * Math.pow(10, decimals)); // Mint 1,000,000 tokens

const main = async () => {
  const connection = new Connection(rpc, "confirmed");

  // Step 2 - Create a New Token
  const mintLamports = await connection.getMinimumBalanceForRentExemption(mintLen);
  const instuctions = [
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: mint,
      space: mintLen,
      lamports: mintLamports,
      programId: TOKEN_2022_PROGRAM_ID,
    }),
    createInitializeTransferFeeConfigInstruction(
      mint,
      new PublicKey(transferFeeConfigAuthority),
      new PublicKey(withdrawWithheldAuthority),
      feeBasisPoints,
      maxFee,
      TOKEN_2022_PROGRAM_ID
    ),
    createInitializeMintInstruction(mint, decimals, mintAuthority.publicKey, null, TOKEN_2022_PROGRAM_ID),
    ComputeBudgetProgram.setComputeUnitPrice({
      microLamports: 20_000,
    }),
  ];

  await createAndSendV0Tx(
    connection,
    instuctions,
    payer.publicKey,
    [payer, mintKeypair]
  );

  console.log("New Token Created: ", mint);

  // Step 3 - Mint tokens to root wallet
  const rootWalletAta = await createAssociatedTokenAccountIdempotent(
    connection,
    payer,
    mint,
    new PublicKey(payer.publicKey),
    {},
    TOKEN_2022_PROGRAM_ID
  );


  const mintToIx = await createMintToInstruction(
    mint,
    rootWalletAta,
    mintAuthority.publicKey,
    mintAmount,
    [],
    TOKEN_2022_PROGRAM_ID
  );

  await createAndSendV0Tx(
    connection,
    [mintToIx],
    payer.publicKey,
    [payer]
  );

  console.log("Tokens Minted");
}

main()
.then(() => console.log("Success"))
.catch((error) => console.log("Error: ", error))
