"use client";
import SidebarPages from "@/components/sidebar-pages";
import React, { useEffect, useState } from "react";
import { Connection, PublicKey } from "@solana/web3.js";

// Constants
const PROGRAM_ID = new PublicKey(
  "3zYbScQeVE1A2oA4b7v7UBQQytSqrcvScjFVxH8zwAPj"
); // Your program ID
const RPC_ENDPOINT =
  "https://api.devnet.solana.com";

  
  const connection = new Connection(RPC_ENDPOINT, 'confirmed');
const Page = () => {
  const [tokenAccounts, setTokenAccounts] = useState<any[]>([]);

  useEffect(() => {
    const getTokens = async () => {
      console.log("Getting token accounts...");
      try {
        const accounts = await connection.getParsedProgramAccounts(PROGRAM_ID);

        // Filter accounts to find mint accounts
        const mintAccounts = accounts.filter(account => {
          // Token mint accounts have a specific data structure, including a certain data length and certain flags.
          // You may need to parse the account data to determine if it's a mint account.
          // Here we assume all accounts returned are token mints, which might not be accurate.
          // You should implement the necessary checks based on your specific use case.
          return account.account.data; // Placeholder condition, refine this logic as needed
        });

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
      
        console.log('Found mint accounts:', parsedAccounts);
      } catch (error) {
        console.log("Error getting token accounts:", error);
      }
    };

    getTokens();
  }, []);

  return (
    <div>
      <SidebarPages />
      <div>{tokenAccounts.length}</div>      
    </div>
  );
};

export default Page;
