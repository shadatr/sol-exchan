"use client";
import SidebarPages from "@/components/sidebar-pages";
import React, { useEffect, useState } from "react";
import { Connection, ParsedAccountData, PublicKey, clusterApiUrl } from "@solana/web3.js";

// Constants
const PROGRAM_ID = new PublicKey(
  "HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8"
); // Your program ID
const RPC_ENDPOINT =
  "https://api.devnet.solana.com"; // Or use 'https://api.devnet.solana.com' for devnet

  const connection = new Connection(RPC_ENDPOINT, 'confirmed');
const Page = () => {
  const [tokenAccounts, setTokenAccounts] = useState<any[]>([]);

  useEffect(() => {
    const getTokens = async () => {
      console.log("Getting token accounts...");
      try {
        const accounts = await connection.getParsedProgramAccounts(PROGRAM_ID);
        console.log(accounts);
        const parsedAccounts = accounts.map(account => {
          const { pubkey, account: { data } } = account;
          if ('parsed' in data) {
            return {
              pubkey: pubkey.toBase58(),
              mint: data.parsed.info.mint,
              owner: data.parsed.info.owner,
              tokenAmount: data.parsed.info.tokenAmount
            };
          } else {
            return {
              pubkey: pubkey.toBase58(),
              mint: null,
              owner: null,
              tokenAmount: null
            };
          }
        });
        
        console.log("parseaccoun",parsedAccounts);
        setTokenAccounts(accounts);
      } catch (error) {
        console.log("Error getting token accounts:", error);
      }
    };

    getTokens();
  }, []);

  return (
    <div>
      <SidebarPages />
      <div>fdsehjfdhjhfdhjdfvjhfdvjhvdfjhfdvjx{tokenAccounts.length}</div>      
    </div>
  );
};

export default Page;
