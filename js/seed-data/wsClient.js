import {io} from "socket.io-client";
import {Transaction} from "@solana/web3.js";

const main = async () => {
  const socket = io("ws://localhost:8090", {
    transports: ["websocket"],
  });

  socket.on("connect", () => {
    socket.emit("collect", "9NxTF8W3gB1y49LBn1GTp5QqPmkdp4P8HJDiqJgJQSUB", "681gMAUpbqTms3RW773mHXjx8hvw4SyosjC2ZcizG6bd");
    socket.emit("new-tokens");

    socket.on("681gMAUpbqTms3RW773mHXjx8hvw4SyosjC2ZcizG6bd", (msg) => {
      console.log("New encoded transaction", msg);

      const tx = Transaction.from(msg.data);
      console.log("Decoded transaction", tx);
    });
  });

  socket.io.on("error", (error) => {
    console.log("Error", error)
  });
}

main()
.then(() => console.log("Success"))
.catch((error) => console.log("Error: ", error))
