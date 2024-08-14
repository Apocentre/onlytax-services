import { PublicKey } from "@solana/web3.js"

const main = async () => {
  const mintAccount = new PublicKey("FxLRYanUFeBzTJNrrenLGKTRF8A7sh7fFXxUkXhV77Kc");
}

main()
.then(() => console.log("Success"))
.catch((error) => console.log("Error: ", error))
