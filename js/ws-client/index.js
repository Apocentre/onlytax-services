const {io} = require("socket.io-client");

const main = async () => {
  const socket = io("ws://localhost:8090", {
    transports: ["websocket"],
  });

  socket.on("connect", () => {
    socket.emit("collect", "9NxTF8W3gB1y49LBn1GTp5QqPmkdp4P8HJDiqJgJQSUB", "681gMAUpbqTms3RW773mHXjx8hvw4SyosjC2ZcizG6bd");
    socket.emit("new-tokens");

    socket.on("681gMAUpbqTms3RW773mHXjx8hvw4SyosjC2ZcizG6bd", (msg) => {
      console.log("New trade received", msg);
    });
  });

  socket.io.on("error", (error) => {
    console.log("Error", error)
  });
}

main()
.then(() => console.log("Success"))
.catch((error) => console.log("Error: ", error))
