"use client";
import React, { useState } from "react";
import { Sidebar, SidebarBody, SidebarLink } from "./ui/sidebar";
import {
  IconArrowLeft,
  IconBrandTabler,
  IconSettings,
  IconUserBolt,
  IconWallet,
  IconPlus,
} from "@tabler/icons-react";
import Link from "next/link";
import Image, { ImageLoader } from "next/image";
import { cn } from "@/lib/utils";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { useWallet } from "@solana/wallet-adapter-react";

export default function SidebarPages() {
  const links = [
    {
      label: "Dashboard",
      href: "/tokens/discover",
      icon: (
        <IconBrandTabler className="text-neutral-700 dark:text-neutral-200 h-8 w-8 flex-shrink-0" />
      ),
    },
    {
      label: "Profile",
      href: "/tokens/profile",
      icon: (
        <IconUserBolt className="text-neutral-700 dark:text-neutral-200 h-8 w-8 flex-shrink-0" />
      ),
    },
    {
      label: "Add Token",
      href: "/token/settings",
      icon: (
        <IconPlus className="text-neutral-700 dark:text-neutral-200 h-8 w-8 flex-shrink-0" />
      ),
    },
    {
      label: "Logout",
      href: "#",
      icon: (
        <IconArrowLeft
          className="text-neutral-700 dark:text-neutral-200 h-8 w-8 flex-shrink-0"
          onClick={() => wallet.disconnect()}
        />
      ),
    },
  ];
  const [open, setOpen] = useState(false);
  const wallet = useWallet();
  const walletAddress = wallet.publicKey 
    ? wallet.publicKey.toBase58().slice(0, 4) + "..." + wallet.publicKey.toBase58().slice(-4) 
    : "No Wallet Connected";

  const imageUrl = `https://api.dicebear.com/9.x/fun-emoji/svg?seed=${encodeURIComponent(
    wallet.publicKey ? wallet.publicKey.toBase58() : ""
  )}`;

  const loaderProp: ImageLoader = ({ src }: { src: string }) => {
    return `https://api.dicebear.com/9.x/fun-emoji/svg?seed=${encodeURIComponent(
      src
    )}`;
  };
  return (
    <div className="fixed">
      <div
        className={cn(
          "rounded-md flex flex-col md:flex-row bg-gray-100 dark:bg-black w-full flex-1 max-w-7xl mx-auto  border-neutral-200 dark:border-neutral-700 overflow-hidden",
          "h-[100vh]" // for your use case, use `h-screen` instead of `h-[60vh]`
        )}
      >
        <Sidebar open={open} setOpen={setOpen}>
          <SidebarBody className="justify-between gap-10 ">
            <div className="flex flex-col flex-1 overflow-y-auto">
              {open ? <Logo /> : <LogoIcon />}
              {wallet.connected ? (
                <div className="mt-8 flex flex-col gap-2">
                  {links.map((link, idx) => (
                    <SidebarLink key={idx} link={link} />
                  ))}
                </div>
              ) : (
                ""
              )}
            </div>
            <div>
              {wallet.connected && wallet.publicKey ? (
                <SidebarLink
                  link={{
                    label: walletAddress,
                    href: "#",
                    icon: (
                      <Image
                        src={imageUrl}
                        className="h-8 w-8 flex-shrink-0 rounded-full"
                        width={50}
                        height={50}
                        alt="Avatar"
                        loader={loaderProp}
                      />
                    ),
                  }}
                />
              ) : (
                <div className="mt-8">
                  <SidebarLink
                    key={1}
                    link={{
                      label: (
                        <WalletMultiButton
                          style={{
                            background: "#FF78C9",
                            borderRadius: "30px",
                          }}
                        />
                      ),
                      href: "#",
                      icon: (
                        <IconWallet className="text-neutral-700 dark:text-neutral-200 h-8 w-8 flex-shrink-0" />
                      ),
                    }}
                  />
                </div>
              )}
            </div>
          </SidebarBody>
        </Sidebar>
      </div>
    </div>
  );
}
export const Logo = () => {
  return (
    <Link
      href="/"
      className="font-normal flex space-x-2 items-center text-sm text-black py-1 relative z-20"
    >
      <Image src={"/SOLCHAN.png"} alt={""} width={100} height={100} />
    </Link>
  );
};
export const LogoIcon = () => {
  return (
    <Link
      href="/"
      className="font-normal flex space-x-2 items-center text-sm text-black py-1 relative z-20"
    >
      <Image src={"/SC.png"} alt={""} width={30} height={30} />
    </Link>
  );
};
