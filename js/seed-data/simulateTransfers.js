import {Keypair, PublicKey} from "@solana/web3.js"
import config from "./.config.json" assert { type: "json" };
import {createTestAccounts, tranfer, createAtaIfNeeded} from "./common.js"

const rootWallet = Keypair.fromSecretKey(Buffer.from(config.rootWallet));
const withdrawWithheldAuthority = Keypair.fromSecretKey(Buffer.from(config.withdrawWithheldAuthorityKey));
const mint = new PublicKey("Ex7QKTHsGHMkVtYg8tgi48Wyw1XrH8hM4dp6B8SGenVz");
const decimals = 6;

const main = async () => {
  createAtaIfNeeded(rootWallet, mint, withdrawWithheldAuthority);

  const testAccounts = createTestAccounts();
  // transfer chain: rootWallet -> testAccounts[0] -> testAccounts[1] -> testAccounts[2]...
  await tranfer(mint, decimals, rootWallet, rootWallet, testAccounts[0], BigInt(1000 * Math.pow(10, 6)))

  for(let i = 0; i < 99; i++) {
    const from = testAccounts[i]
    const to = testAccounts[i + 1]; 

    await tranfer(mint, decimals, rootWallet, from, to)
  }
}

main()
.then(() => console.log("Success"))
.catch((error) => console.log("Error: ", error))
