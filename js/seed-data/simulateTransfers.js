import {Keypair, PublicKey, Connection} from "@solana/web3.js"
import config from "./.config.json" assert { type: "json" };
import {createTestAccounts, tranfer} from "./common.js"

const rootWallet = Keypair.fromSecretKey(Buffer.from(config.rootWallet));
const mint = new PublicKey("9NxTF8W3gB1y49LBn1GTp5QqPmkdp4P8HJDiqJgJQSUB");
const decimals = 6;

const main = async () => {
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
