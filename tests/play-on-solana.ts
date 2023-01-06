import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { Cartridge } from "../target/types/cartridge";
import fs from "fs";

describe("play-on-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.local();
  const cartidge = anchor.workspace.Cartridge as Program<Cartridge>;

  const user = Keypair.generate();
  const game = Keypair.generate();

  const gameFile = Buffer.from(fs.readFileSync("./tests/pokemon.gb", "utf8"));

  before(async () => {
    const fundTx = new Transaction();
    fundTx.add(
      SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: user.publicKey,
        lamports: 200 * LAMPORTS_PER_SOL,
      })
    );
    await provider.sendAndConfirm(fundTx);
  });

  describe("Cartridge", () => {
    it("Install game", async () => {
      const rent = await provider.connection.getMinimumBalanceForRentExemption(
        gameFile.length
      );

      await cartidge.methods
        .install()
        .accounts({
          signer: user.publicKey,
          game: game.publicKey,
        })
        .preInstructions([
          SystemProgram.createAccount({
            fromPubkey: user.publicKey,
            newAccountPubkey: game.publicKey,
            lamports: rent,
            space: gameFile.length,
            programId: cartidge.programId,
          }),
        ])
        .signers([user, game])
        .rpc();
    });

    it("Upload game", async () => {
      const promises = [];
      for (let i = 0; i < gameFile.length; i += 850) {
        promises.push(
          cartidge.methods
            .upload(new BN(i), gameFile.subarray(i, i + 850))
            .accounts({
              signer: user.publicKey,
              game: game.publicKey,
            })
            .signers([user])
            .rpc(),
        );
      }

      await Promise.all(promises);
    });
  });
});
