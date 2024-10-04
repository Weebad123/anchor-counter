import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { AnchorCounter } from "../target/types/anchor_counter";

describe("anchor-counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorCounter as Program<AnchorCounter>;
// This generates a new key pair for the counter account we'll be initializing
  const counter = anchor.web3.Keypair.generate();
// The below creates placeholders for a test of each instruction
  it("Is initialized!", async () => {
    // Add test for the initialize instruction here
    const tx = await program.methods
      .initialize()
      .accounts({ counter: counter.publicKey })
      .signers([counter])
      .rpc();

      const account = await program.account.counter.fetch(counter.publicKey);
      expect(account.count.toNumber()).to.equal(0);
  });

  it("Incremented the count", async () => {
    // creating the test for the increment instruction
    const tx = await program.methods
      .increment()
      .accounts({ counter: counter.publicKey, user: provider.wallet.publicKey })
      .rpc();

      const account = await program.account.counter.fetch(counter.publicKey);
      expect(account.count.toNumber()).to.equal(3);
  });

  it("Decremented the count", async () => {
    // creating the test for the decrement instruction
    const tx = await program.methods
      .decrement()
      .accounts({ counter: counter.publicKey, user: provider.wallet.publicKey })
      .rpc();

      const account = await program.account.counter.fetch(counter.publicKey);
      expect(account.count.toNumber()).to.equal(2);
  })
});
