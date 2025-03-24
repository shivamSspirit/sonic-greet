import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GreetSvm } from '../target/types/greet_svm';
import { PublicKey, Keypair } from "@solana/web3.js";

describe("svm-greet", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.GreetSvm as Program<GreetSvm>;
  const user = provider.wallet as anchor.Wallet;

  // Derive the PDA address using the seeds specified on the program
  const [greetPDAAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from("data"), user.publicKey.toBuffer()],
    program.programId
  );

  it("Is initialized!", async () => {
    const transactionSignature = await program.methods.initialize("Hello, Svm maxim!")
      .accounts({
        user: user.publicKey,
        greetAccount: greetPDAAccount,
      })
      .rpc();

    console.log("Transaction Signature:", transactionSignature);
  });

  it("Fetch Account", async () => {
    const greetPdaAccount = await program.account.greetAccount.fetch(greetPDAAccount);
    console.log(JSON.stringify(greetPdaAccount, null, 2));
  });
});

