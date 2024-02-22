import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { S } from "../target/types/s";
import { ASSOCIATED_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import {
  createMint,
} from '@solana/spl-token';

describe("s", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.S as Program<S>;
  const authority = provider.wallet as anchor.Wallet;

  const poolName = "ABC";
  it("Is initialized!", async () => {
    const mintAddress = await createMint(
      provider.connection,
      authority.payer,
      authority.publicKey,
      authority.publicKey,
      9
    );

    const [poolAddress] = anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode(poolName)],
      program.programId
    );

    const [poolTokenAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        poolAddress.toBuffer(),
        TOKEN_PROGRAM_ID.toBuffer(),
        mintAddress.toBuffer(),
      ],
      ASSOCIATED_PROGRAM_ID
    );

    const tx = await program.methods.initPool(poolName)
    .accounts({
        authority: authority.publicKey,
        pool: poolAddress,
        mint: mintAddress,
        poolTokenAccount: poolTokenAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID
    })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});

