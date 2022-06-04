import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import BN = require("bn.js");
import {
  getKeypair,
  getProgramId,
  getPublicKey,
} from "./utils";

const start = async () => {
  const keypairUser = getKeypair("main");
  const usdcAcctUser = getPublicKey("usdc");
  const cryptex_usdcAcctUser = getPublicKey("cusdc");
  const programId = getProgramId();

  const mint_signer_keypair = getKeypair("mint_auth");

  const connection = new Connection("https://api.devnet.solana.com", "confirmed");
 
  const send_instruction = new TransactionInstruction({
    programId: programId,
    data: Buffer.from(
      Uint8Array.of(0, ...new BN(20000).toArray("le", 8))
    ),
    keys: [
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: new PublicKey('ACqqDBXdFhgatszRESwmdkfgLH7coJm7SxaTuiEhEQ9y'), isSigner: false, isWritable: true },
      { pubkey: usdcAcctUser, isSigner: false, isWritable: true },
      { pubkey: keypairUser.publicKey, isSigner: true, isWritable: false },
    ],
  });

  console.log("sending...");
  await connection.sendTransaction(
    new Transaction().add(send_instruction),
    [keypairUser],
    { skipPreflight: false, preflightCommitment: "confirmed" }
  );

  // sleep to allow time to update
  await new Promise((resolve) => setTimeout(resolve, 1000));

  const mint_instruction = new TransactionInstruction({
    programId: programId,
    data: Buffer.from(
      Uint8Array.of(1, ...new BN(2000000000).toArray("le", 8))
    ),
    keys: [
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: cryptex_usdcAcctUser, isSigner: false, isWritable: true },
      { pubkey: new PublicKey('6zdV6NKr7JnnyFxwGyBjgD3N8sJrR9rM5nmqUw7msrS'), isSigner: false, isWritable:true },
      { pubkey: mint_signer_keypair.publicKey, isSigner: true, isWritable: false },
    ],
  });

  console.log("minting...")
  await connection.sendTransaction(
    new Transaction().add(mint_instruction),
    [mint_signer_keypair],
    { skipPreflight: false, preflightCommitment: "confirmed" }
  );

  console.log("Swapping Complete");
};

start();
