"use client";
import SidebarPages from "@/components/sidebar-pages";
import React, { useEffect, useState } from "react";
import { Connection, PublicKey, clusterApiUrl } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';

// Constants
const PROGRAM_ID = new PublicKey(
  "HRfR1RiRCLHmfgG5SCmgXrBvdpCYtfGc8fCJSrEXej6W"
); // Your program ID
const RPC_ENDPOINT =
  "https://api.devnet.solana.com"; // RPC endpoint for the Solana network

  
  const connection = new Connection(RPC_ENDPOINT, 'confirmed');
const Page = () => {

  const [tokenAccounts, setTokenAccounts] = useState<any[]>([]);

  useEffect(() => {
    const getTokens = async () => {
      console.log("Getting token accounts...");
      try {
      
 
        //     const tokenAccounts1 = await connection.getProgramAccounts(
        //       PROGRAM_ID,
        //   // { programId:TOKEN_PROGRAM_ID } // Replace with actual Token Program ID
        // );
    
        // console.log("Token accounts:", tokenAccounts[0].pubkey.toBase58());
        const tokenAccounts = await connection.getParsedTokenAccountsByOwner(
          PROGRAM_ID,
      { programId:TOKEN_PROGRAM_ID } // Replace with actual Token Program ID
    );

    console.log("Token accounts:", tokenAccounts);

    const tokens = tokenAccounts.value.map((account) => {
      const tokenMint = account.account.data.parsed.info.mint;
      const tokenAmount = account.account.data.parsed.info.tokenAmount.uiAmount;
      return { tokenMint, tokenAmount };
    });

    console.log('Tokens in pool:', tokens);
    return tokens;
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
