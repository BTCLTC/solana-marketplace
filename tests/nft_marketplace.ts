import * as anchor from "@project-serum/anchor"
import { Program, Provider } from "@project-serum/anchor"
import { NftMarketplace } from "../target/types/nft_marketplace"
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token"

const utils = require("./utils")
import * as fs from "fs"
import * as assert from "assert"
import { config } from "chai"

const provider = anchor.AnchorProvider.env()
anchor.setProvider(provider)
const program = anchor.workspace.NftMarketplace as Program<NftMarketplace>
const KEY_PATH = "tests/keys/"

const CONFIG_PDA_SEED = "config"
const TOKEN_CONFIG_PDA_SEED = "token_config"
const NFT_VAULT_PDA_SEED = "nft_vault"
const TOKEN_VAULT_PDA_SEED = "token_vault"
const SELL_PDA_SEED = "sell"
const OFFER_PDA_SEED = "offer"

describe("nft_marketplace", () => {
  let close_sell_mode: boolean = false
  let buy_mode: boolean = false
  let offer_cancel_mode: boolean = false
  let setting: any
  let nft_type: string
  let token_type: string = "usdc"
  let chainlinkProgram: anchor.web3.PublicKey = new anchor.web3.PublicKey(
    "HEvSKofvBgfaexv23kMabbYqxasxU3mQ4ibBMEmJWHny"
  )
  let chainlinkFeed: anchor.web3.PublicKey = new anchor.web3.PublicKey("HgTtcbcmp5BeThax5AU8vg4VwK79qAvAKKFMs8txMLW6")
  let nftMintObject: Token
  let nftMintPubKey: anchor.web3.PublicKey

  let usdcMintKeyPair: anchor.web3.Keypair
  let usdcMintObject: Token
  let usdcMintPubkey: anchor.web3.PublicKey

  let user_A: anchor.web3.Keypair
  let user_A_NFTWallet: anchor.web3.PublicKey
  let user_A_usdcWallet: anchor.web3.PublicKey

  let user_B: anchor.web3.Keypair
  let user_B_NFTWallet: anchor.web3.PublicKey
  let user_B_usdcWallet: anchor.web3.PublicKey

  // the program's config account
  let config: anchor.web3.PublicKey
  let config_bump: number

  // the token config account
  let token_config_pda: anchor.web3.PublicKey
  let token_config_pda_bump: number

  // the token vault account
  let token_vault_pda: anchor.web3.PublicKey
  let token_vault_pda_bump: number

  let nft_vault: anchor.web3.PublicKey
  let nft_vault_bump: number

  let trade_fee_rate: anchor.BN

  let sell_pda: anchor.web3.PublicKey
  let sell_pda_bump: number

  let offer_pda: anchor.web3.PublicKey
  let offer_pda_bump: number

  it("1. Prepare Tokens", async () => {
    let usdcKeyPairFile = fs.readFileSync(KEY_PATH + "usdc.json", "utf-8")
    let usdcKeyPairData = JSON.parse(usdcKeyPairFile)
    usdcMintKeyPair = anchor.web3.Keypair.fromSecretKey(new Uint8Array(usdcKeyPairData))
    usdcMintObject = await utils.createMint(
      usdcMintKeyPair,
      provider,
      provider.wallet.publicKey,
      null,
      6,
      TOKEN_PROGRAM_ID
    )
    usdcMintPubkey = usdcMintObject.publicKey
    console.log("USDC: ", usdcMintPubkey.toString())
  })

  it("2. Prepare User", async () => {
    // Load User_A
    let userAPairFile = fs.readFileSync(KEY_PATH + "user_A.json", "utf-8")
    let userAPairData = JSON.parse(userAPairFile)
    user_A = anchor.web3.Keypair.fromSecretKey(new Uint8Array(userAPairData))

    // Airdrop 10 SOL to User A
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user_A.publicKey, 10_000_000_000),
      "confirmed"
    )

    // Load User_B
    let userBPairFile = fs.readFileSync(KEY_PATH + "user_B.json", "utf-8")
    let userBPairData = JSON.parse(userBPairFile)
    user_B = anchor.web3.Keypair.fromSecretKey(new Uint8Array(userBPairData))

    // Airdrop 10 SOL to User B
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user_B.publicKey, 10_000_000_000),
      "confirmed"
    )

    // Create NFT Token for test
    let mintKeyNFT = anchor.web3.Keypair.generate()
    nftMintObject = await utils.createMint(mintKeyNFT, provider, provider.wallet.publicKey, null, 0, TOKEN_PROGRAM_ID)
    nftMintPubKey = nftMintObject.publicKey

    // Create NFT Account for User_A and User_B
    user_A_NFTWallet = await nftMintObject.createAssociatedTokenAccount(user_A.publicKey)

    user_B_NFTWallet = await nftMintObject.createAssociatedTokenAccount(user_B.publicKey)

    // Create USDC Account for User_A and User_B
    user_A_usdcWallet = await usdcMintObject.createAssociatedTokenAccount(user_A.publicKey)
    user_B_usdcWallet = await usdcMintObject.createAssociatedTokenAccount(user_B.publicKey)

    // Mint NFT to User_A
    await utils.mintToAccount(provider, nftMintPubKey, user_A_NFTWallet, 1)

    // Mint USDC to user_B
    await utils.mintToAccount(provider, usdcMintPubkey, user_B_usdcWallet, 1_000_000_000) // 1000 USDC

    console.log("User_A: ", user_A.publicKey.toString())
    console.log("User_B: ", user_B.publicKey.toString())
    console.log("User_A SOL: ", await provider.connection.getBalance(user_A.publicKey))
    console.log("User_B SOL: ", await provider.connection.getBalance(user_B.publicKey))

    assert.strictEqual(await utils.getTokenBalance(provider, user_A_NFTWallet), 1)
    assert.strictEqual(await utils.getTokenBalance(provider, user_B_NFTWallet), 0)
    assert.strictEqual(await utils.getTokenBalance(provider, user_B_usdcWallet), 1_000_000_000)
  })

  it("3. Load config", async () => {
    let configFile = fs.readFileSync("config/local.json", "utf-8")
    setting = JSON.parse(configFile)
    trade_fee_rate = new anchor.BN(setting["fee_rate"])
    nft_type = setting["nft_type"]
  })

  it("4. Setup", async () => {
    ;[config, config_bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(CONFIG_PDA_SEED), Buffer.from(nft_type)],
      program.programId
    )
    await program.methods
      .setup(nft_type, config_bump, trade_fee_rate)
      .accounts({
        owner: provider.wallet.publicKey,
        config: config,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY
      })
      .rpc()

    const config_fetch = await program.account.config.fetch(config)
    console.log("Trade Fee: ", config_fetch.tradeFeeRate.toString() + "%")
  })
  it("5. Token Setup", async () => {
    let tokenSetting = setting["tokens"][0]
    token_type = tokenSetting["token_type"]
    let tokenMint = new anchor.web3.PublicKey(tokenSetting["token_mint"])
    let need_init = tokenSetting["need_init"]
    let index: number = Number(tokenSetting["index"])
    let decimals: number = Number(tokenSetting["decimals"])
    ;[token_config_pda, token_config_pda_bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_CONFIG_PDA_SEED), Buffer.from(nft_type), Buffer.from(token_type)],
      program.programId
    )
    ;[token_vault_pda, token_vault_pda_bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_PDA_SEED), Buffer.from(nft_type), Buffer.from(token_type)],
      program.programId
    )

    if (need_init) {
      await program.methods
        .initTokenAccount(nft_type, token_type)
        .accounts({
          owner: provider.wallet.publicKey,
          config: config,
          tokenMint: tokenMint,
          tokenVault: token_vault_pda,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY
        })
        .rpc()
    }
    await program.methods
      .tokenSetup(nft_type, token_type, token_config_pda_bump, index, decimals)
      .accounts({
        owner: provider.wallet.publicKey,
        config: config,
        tokenMint: tokenMint,
        tokenVault: token_vault_pda,
        tokenConfig: token_config_pda,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY
      })
      .rpc()

    const config_fetch_two = await program.account.config.fetch(config)
    assert.strictEqual(config_fetch_two.usdcMint.mintKey.toString(), usdcMintPubkey.toString())
    assert.strictEqual(config_fetch_two.usdcMint.decimals, 6)
  })
  it("6. Start Sell", async () => {
    const sell_price = new anchor.BN(5_000_000) // 5 USDC
    ;[sell_pda, sell_pda_bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(SELL_PDA_SEED), Buffer.from(nft_type), user_A.publicKey.toBuffer(), nftMintPubKey.toBuffer()],
      program.programId
    )
    ;[nft_vault, nft_vault_bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(NFT_VAULT_PDA_SEED), nftMintPubKey.toBuffer()],
      program.programId
    )
    await program.methods
      .startSell(nft_type, sell_price, [0, 0, 0, 0, 0])
      .accounts({
        user: user_A.publicKey,
        config: config,
        nftMint: nftMintPubKey,
        nftVault: nft_vault,
        userNftVault: user_A_NFTWallet,
        sell: sell_pda,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY
      })
      .signers([user_A])
      .rpc()
    const sell_fetch = await program.account.sell.fetch(sell_pda)
    console.log("Sell ID: ", sell_fetch.id.toString())
    console.log("Sell Price: ", sell_fetch.price.toString())
    console.log("Flags: ", sell_fetch.flags.toString())
    assert.strictEqual(await utils.getTokenBalance(provider, user_A_NFTWallet), 0)
    assert.strictEqual(await utils.getTokenBalance(provider, nft_vault), 1)
  })
  it("7. Update Sell", async () => {
    const update_price = new anchor.BN(100_000_000) // 100 USDC
    await program.methods
      .updateSell(nft_type, update_price, [1, 0, 0, 0, 0])
      .accounts({
        user: user_A.publicKey,
        config: config,
        nftMint: nftMintPubKey,
        sell: sell_pda,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY
      })
      .signers([user_A])
      .rpc()
    const sell_fetch = await program.account.sell.fetch(sell_pda)
    console.log("Sell ID: ", sell_fetch.id.toString())
    console.log("Sell Price: ", sell_fetch.price.toString())
    console.log("Flags: ", sell_fetch.flags.toString())
  })
  if (close_sell_mode) {
    it("7. Close Sell", async () => {
      await program.methods
        .closeSell(nft_type)
        .accounts({
          user: user_A.publicKey,
          config: config,
          nftMint: nftMintPubKey,
          nftVault: nft_vault,
          userNftVault: user_A_NFTWallet,
          sell: sell_pda,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY
        })
        .signers([user_A])
        .rpc()
      assert.strictEqual(await utils.getTokenBalance(provider, user_A_NFTWallet), 1)
    })
  } else {
    if (buy_mode) {
      it("8. Buy (USDC)", async () => {
        try {
          await program.methods
            .buy(nft_type, token_type, 1)
            .accounts({
              buyer: user_B.publicKey,
              seller: user_A.publicKey,
              config: config,
              tokenConfig: token_config_pda,
              nftMint: nftMintPubKey,
              nftVault: nft_vault,
              buyerNftVault: user_B_NFTWallet,
              tokenMint: usdcMintPubkey,
              tokenVault: token_vault_pda,
              buyerTokenWallet: user_B_usdcWallet,
              sellerTokenWallet: user_A_usdcWallet,
              sell: sell_pda,
              chainlinkFeed: chainlinkFeed,
              chainlinkProgram: chainlinkProgram,
              systemProgram: anchor.web3.SystemProgram.programId,
              tokenProgram: TOKEN_PROGRAM_ID,
              rent: anchor.web3.SYSVAR_RENT_PUBKEY
            })
            .signers([user_B])
            .rpc()
        } catch (e) {
          console.log(e)
        }
        assert.strictEqual(await utils.getTokenBalance(provider, user_A_NFTWallet), 0)
        assert.strictEqual(await utils.getTokenBalance(provider, user_B_NFTWallet), 1)
        const fetch_token_config = await program.account.tokenConfig.fetch(token_config_pda)
        assert.strictEqual(fetch_token_config.fee.toNumber(), 10_000_000)
        assert.strictEqual(await utils.getTokenBalance(provider, user_A_usdcWallet), 90_000_000)
      })
    } else {
      it("9. Make Offer (USDC)", async () => {
        const offer_price = new anchor.BN(80_000_000) // 80 USDC
        const date = Date.now()
        const expired_at = date + 1000 * 60 * 60 * 24 * 3 // 3 days
        ;[offer_pda, offer_pda_bump] = await anchor.web3.PublicKey.findProgramAddress(
          [
            Buffer.from(OFFER_PDA_SEED),
            Buffer.from(nft_type),
            user_B.publicKey.toBuffer(),
            nftMintPubKey.toBuffer(),
            Buffer.from("1")
          ],
          program.programId
        )
        await program.methods
          .applyOffer(nft_type, token_type, new anchor.BN(1), 1, offer_price, new anchor.BN(expired_at))
          .accounts({
            buyer: user_B.publicKey,
            config: config,
            tokenConfig: token_config_pda,
            nftMint: nftMintPubKey,
            tokenMint: usdcMintPubkey,
            tokenVault: token_vault_pda,
            buyerTokenWallet: user_B_usdcWallet,
            sell: sell_pda,
            offer: offer_pda,
            chainlinkFeed: chainlinkFeed,
            chainlinkProgram: chainlinkProgram,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY
          })
          .signers([user_B])
          .rpc()
        const fetch_offer = await program.account.offer.fetch(offer_pda)
        assert.strictEqual(fetch_offer.offerPrice.toNumber(), 80_000_000)
        assert.strictEqual(await utils.getTokenBalance(provider, user_B_usdcWallet), 920_000_000)
      })
      if (offer_cancel_mode) {
        it("10. Cancel Offer (USDC)", async () => {
          await program.methods
            .cancelOffer(nft_type, token_type, new anchor.BN(1))
            .accounts({
              buyer: user_B.publicKey,
              config: config,
              tokenConfig: token_config_pda,
              nftMint: nftMintPubKey,
              tokenMint: usdcMintPubkey,
              tokenVault: token_vault_pda,
              buyerTokenWallet: user_B_usdcWallet,
              sell: sell_pda,
              offer: offer_pda,
              systemProgram: anchor.web3.SystemProgram.programId,
              tokenProgram: TOKEN_PROGRAM_ID,
              rent: anchor.web3.SYSVAR_RENT_PUBKEY
            })
            .signers([user_B])
            .rpc()
          assert.strictEqual(await utils.getTokenBalance(provider, user_B_usdcWallet), 1_000_000_000)
        })
      } else {
        it("11. Accept Offer (USDC)", async () => {
          await program.methods
            .acceptOffer(nft_type, token_type, new anchor.BN(1))
            .accounts({
              seller: user_A.publicKey,
              buyer: user_B.publicKey,
              config: config,
              tokenConfig: token_config_pda,
              nftMint: nftMintPubKey,
              nftVault: nft_vault,
              buyerNftVault: user_B_NFTWallet,
              tokenMint: usdcMintPubkey,
              tokenVault: token_vault_pda,
              sellerTokenWallet: user_A_usdcWallet,
              sell: sell_pda,
              offer: offer_pda,
              systemProgram: anchor.web3.SystemProgram.programId,
              tokenProgram: TOKEN_PROGRAM_ID,
              rent: anchor.web3.SYSVAR_RENT_PUBKEY
            })
            .signers([user_A])
            .rpc()
          assert.strictEqual(await utils.getTokenBalance(provider, user_B_NFTWallet), 1)
          assert.strictEqual(await utils.getTokenBalance(provider, user_A_usdcWallet), 72_000_000)
        })
      }
    }
  }
})
