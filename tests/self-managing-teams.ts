import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SelfManagingTeams } from "../target/types/self_managing_teams";
import { Keypair } from "@solana/web3.js"

describe("self-managing-teams", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SelfManagingTeams as Program<SelfManagingTeams>;

  const newAccountKp = Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
                .initialize()
                .accounts({
                  teamMembers: newAccountKp.publicKey,
                })
                .signers([newAccountKp])
                .rpc();
    console.log("Your transaction signature", tx);
  });
});
