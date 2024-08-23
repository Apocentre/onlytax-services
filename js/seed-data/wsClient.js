import {io} from "socket.io-client";
import {VersionedTransaction, Connection} from "@solana/web3.js";
import {Keypair, PublicKey} from "@solana/web3.js"
import {createAtaIfNeeded} from "./common.js";
import config from "./.config.json" assert { type: "json" };

const withdrawWithheldAuthority = Keypair.fromSecretKey(Buffer.from(config.withdrawWithheldAuthorityKey));
const connection = new Connection(config.rpc, "confirmed");
const mint = new PublicKey("9NxTF8W3gB1y49LBn1GTp5QqPmkdp4P8HJDiqJgJQSUB");

const main = async () => {
  await createAtaIfNeeded(withdrawWithheldAuthority, mint, withdrawWithheldAuthority);

  const socket = io("ws://localhost:8090", {
    transports: ["websocket"],
  });

  socket.on("connect", () => {
    socket.emit("collect", "9NxTF8W3gB1y49LBn1GTp5QqPmkdp4P8HJDiqJgJQSUB", "681gMAUpbqTms3RW773mHXjx8hvw4SyosjC2ZcizG6bd");

    socket.on("681gMAUpbqTms3RW773mHXjx8hvw4SyosjC2ZcizG6bd", async (msg) => {
      console.log("New batch transaction");
      const tx = VersionedTransaction.deserialize(msg.data);
      let {blockhash} = await connection.getLatestBlockhash("confirmed");
      tx.message.recentBlockhash = blockhash;
      tx.sign([withdrawWithheldAuthority]);

      const txid = await connection.sendTransaction(tx, {maxRetries: 10, skipPreflight: false});
      console.log("Transaction sent", txid);
    });
  });

  socket.io.on("error", (error) => {
    console.log("Error", error)
  });
}

main()
.then(() => console.log("Success"))
.catch((error) => console.log("Error: ", error))
