import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SelfManagingTeams } from "../target/types/self_managing_teams";
import { Keypair } from "@solana/web3.js"

describe("self-managing-teams", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SelfManagingTeams as Program<SelfManagingTeams>;

  const teamMembersKp = Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
                .initialize()
                .accounts({
                  teamMembers: teamMembersKp.publicKey,
                })
                .signers([teamMembersKp])
                .rpc();
    console.log("Your transaction signature", tx);
  });

  const davidsKp = Keypair.generate();

  it ("Added member!", async () => {
    const tx = await program.methods
                .createTeamMemberAccount("David", "Prodsec Engineer")
                .accounts({
                  teamMember: davidsKp.publicKey
                })
                .signers([davidsKp])
                .rpc()
    console.log("Your transaction signature", tx);
  })
});
