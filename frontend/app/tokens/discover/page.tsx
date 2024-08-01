"use client";
import SidebarPages from "@/components/sidebar-pages";
import React, { useEffect, useState } from "react";
import { Connection, PublicKey } from "@solana/web3.js";

// Constants
const PROGRAM_ID = new PublicKey(
  "3zYbScQeVE1A2oA4b7v7UBQQytSqrcvScjFVxH8zwAPj"
); // Your program ID

const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
const RPC_ENDPOINT =
  "https://api.devnet.solana.com";

  
const connection = new Connection(RPC_ENDPOINT, 'confirmed');
const Page = () => {
  const [tokenAccounts, setTokenAccounts] = useState<any[]>([]);

  useEffect(() => {
    const getTokens = async () => {
      console.log("Getting token accounts...");
      try {
        let response = await connection.getTokenAccountsByOwner(
          new PublicKey("3zYbScQeVE1A2oA4b7v7UBQQytSqrcvScjFVxH8zwAPj"), // owner here
          {
            programId: TOKEN_PROGRAM_ID,
          }
        );
        console.log("Response:", response);
        const parsedAccounts = response.value.map(account => {
          const { pubkey, account: { data } } = account;
          if ('parsed' in data) {
            return {
              pubkey: pubkey.toBase58(),
              // mint: data.parsed.info.mint,
              // owner: data.parsed.info.owner,
              // tokenAmount: data.parsed.info.tokenAmount
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
        console.log("Token accounts:", parsedAccounts);
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
