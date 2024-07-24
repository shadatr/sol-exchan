"use client";
import SidebarPages from "@/components/sidebar-pages";
import React, { use, useEffect, useState } from "react";
import { Connection, PublicKey } from "@solana/web3.js";
import { AccountLayout } from "@solana/spl-token";

// Constants
const PROGRAM_ID = new PublicKey(
  "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"
); // Your program ID
const TOKEN_PROGRAM_ID = new PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
); // Token Program ID
const RPC_ENDPOINT =
  "https://mainnet.helius-rpc.com/?api-key=8fe5c65a-5f2d-4a16-801d-0ba296fbcd38"; // Or use 'https://api.devnet.solana.com' for devnet

const Page = () => {
  const [tokenAccounts, setTokenAccounts] = useState<any[]>([]);

  useEffect(() => {
    const getTokens = async () => {
      try {
        const connection = new Connection(RPC_ENDPOINT, "processed");
        
        const accounts = await connection.getParsedProgramAccounts(PROGRAM_ID);
        console.log(accounts);

        // Parse and extract token account information
        // const tokenAccounts = accounts.map(({ pubkey, account }) => {
        //   const accountInfo = AccountLayout.decode(account.data);
        //   return {
        //     pubkey,
        //     balance: accountInfo.amount,
        //   };
        // });

        // tokenAccounts.sort((a, b) => Number(b.balance) - Number(a.balance));
        // console.log(tokenAccounts);
        // setTokenAccounts(tokenAccounts);
        // const parsedAccounts = accounts
        //   .map((accountInfo) => {
        //     const accountData = accountInfo.account.data;
        //     if ('parsed' in accountData) {
        //       const parsedData = accountData.parsed;
        //       return {
        //         pubkey: accountInfo.pubkey.toBase58(),
        //         mint: parsedData.info.mint,
        //         owner: parsedData.info.owner,
        //         tokenAmount: parsedData.info.tokenAmount.uiAmount,
        //       };
        //     }
        //     return null;
        //   })
        //   .filter((account) => account !== null);

        // setTokenAccounts(parsedAccounts as any[]);
        // console.log('All token accounts owned by the program:', parsedAccounts);
      } catch (error) {
        console.error("Error getting token accounts:", error);
      }
    };

    getTokens();
  }, []);

  return (
    <div>
      <SidebarPages />
    </div>
  );
};

export default Page;
