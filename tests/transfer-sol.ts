import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from '@solana/web3.js'
import { TransferSol } from "../target/types/transfer_sol";
import { readFileSync } from "fs";
// import { fs } from "fs";


describe("transfer-sol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TransferSol as Program<TransferSol>;
  // const user = 
  const fileContent = readFileSync('./keypair/user_kp.json');
    let user = anchor.web3.Keypair.fromSecretKey(new Uint8Array(JSON.parse(fileContent.toString())));
  
  console.log("🚀 ~ describe ~ user public key:", user.publicKey.toBase58())
  console.log("🚀 ~ describe ~ user secret key:", user.secretKey)

  xit("Is initialized!", async () => {

  const [treasury, _bump] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode('treasury'),
    ],
    program.programId
  )
  console.log("🚀 ~ describe ~ Treasury:", treasury.toBase58())

    // Add your test here.
    console.log("Inside it block :  ");
    
    const tx = await program.methods.initialize()
    .accounts({
      treasury : treasury,
      user : user.publicKey,
      systemProgram  : anchor.web3.SystemProgram.programId,
    })
    .signers([user])
    .rpc();
    console.log("Your transaction signature", tx);
  });

  xit("Is Deposited !!!", async () => {

  const [treasury, bump] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode('treasury'),
    ],
    program.programId
    )
  console.log("🚀 ~ it ~ _bump:", bump)
  console.log("🚀 ~ describe ~ Treasury:", treasury.toBase58())

    // Add your test here.
    const tx = await program.methods.depositLamports(new anchor.BN(1000000000))
    .accounts({
      treasury: treasury,
      user: user.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([user])
    .rpc();
    console.log("Your transaction signature", tx);
  });

  xit("Is Withdrawn !!!", async () => {

  const [treasury, _bump] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("treasury"),
    ],
    program.programId
  )
  console.log("🚀 ~ describe ~ Treasury:", treasury.toBase58())

    // Add your test here.
    const tx = await program.methods.withdrawLamports(new anchor.BN(1000000000))
    .accounts({
      treasury: treasury,
      user: user.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("PDA Data !!!", async () => {

    const [treasury, _bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("treasury"),
      ],
      program.programId
    )
    console.log("🚀 ~ describe ~ Treasury:", treasury.toBase58())

    let pda_bump = _bump;
    console.log("🚀 ~ it ~ pda_bump:", pda_bump)

  });


  it("Is initialized2 !", async () => {

    const [treasury, _bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('treasury'),
      ],
      program.programId
    )
    console.log("🚀 ~ describe ~ Treasury:", treasury.toBase58())
  
      // Add your test here.
      console.log("Inside it block :  ");
      
      const tx = await program.methods.initialize2()
      .accounts({
        treasury : treasury,
        user : user.publicKey,
        systemProgram  : anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
      console.log("Your transaction signature", tx);
    });
  
    it("Is Deposited2 !!!", async () => {
  
    const [treasury, bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('treasury'),
      ],
      program.programId
      )
    console.log("🚀 ~ it ~ _bump:", bump)
    console.log("🚀 ~ describe ~ Treasury:", treasury.toBase58())
  
      // Add your test here.
      const tx = await program.methods.depositLamports2(new anchor.BN(1000000000))
      .accounts({
        treasury: treasury,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
      console.log("Your transaction signature", tx);
    });
  
    it("Is Withdrawn2 !!!", async () => {
  
    const [treasury, _bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("treasury"),
      ],
      program.programId
    )
    console.log("🚀 ~ describe ~ Treasury:", treasury.toBase58())
  
      // Add your test here.
      const tx = await program.methods.withdrawLamports2(new anchor.BN(1000000000))
      .accounts({
        treasury: treasury,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
      console.log("Your transaction signature", tx);
    });

});
