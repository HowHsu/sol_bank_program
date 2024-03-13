import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {PublicKey} from '@solana/web3.js'
import type { BankContract } from "../target/types/bank_contract";

describe("Bank", async () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BankContract as anchor.Program<BankContract>;
  
  let admin = new PublicKey("8pWXLmFkVae27kPiympG2R6JZytmeK9xjQ8SndYKwEvD");

  let [bank_owner] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("bank_owner"),
    ],
    program.programId
  );

  it("set admin", async () => {
    await program.methods
      .setAdmin(admin)
      .accounts({
        bankOwner: bank_owner,
        deployer: program.provider.publicKey,
      })
      .rpc();
  });
});
