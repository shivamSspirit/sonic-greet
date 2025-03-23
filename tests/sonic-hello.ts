import {
  Keypair,
} from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor';
import { HelloSonicWorld } from '../target/types/hello_sonic_world';

const provider = anchor.AnchorProvider.env()
anchor.setProvider(provider)
const payer = provider.wallet as anchor.Wallet

const program = anchor.workspace.HelloSonicWorld as Program<HelloSonicWorld>;

describe('soo-counter', () => {
  // Create the greeting account
  const greetedAccountKeypair = Keypair.generate();

  // Initialize the greeting account

  it('Initialize Counter', async () => {
    await program.methods.initialize(payer.publicKey).accounts({
      greetingAccount: greetedAccountKeypair.publicKey,
      user: payer.publicKey,
    }).signers([greetedAccountKeypair, payer.payer]).rpc()
  });

  // Increment the counter

  it('Increment Counter', async () => {
    await program.methods.incrementGreeting().accounts({
      greetingAccount: greetedAccountKeypair.publicKey
    }).signers([payer.payer]).
      rpc()
  });

  // fetch counter data

  it('fetch account data', async () => {
    const account = await program.account.greetingAccount.fetch(
      greetedAccountKeypair.publicKey
    );
    console.log('Greeting successful. Account data:', account.counter.toString());
  })

})
