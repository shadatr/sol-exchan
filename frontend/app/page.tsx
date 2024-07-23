import { FeaturesSection } from "@/components/features";
import { HoverBorderGradient } from "@/components/ui/hover-border-gradient";
import { MacbookScroll } from "@/components/ui/macbook-scroll";
import { IconCopyright } from "@tabler/icons-react";
import Link from "next/link";

export default function Home() {
  return (
    <div className="flex w-screen items-center justify-center flex-col gap-5 mt-40">
      <span className="gradient text-xxlg font-bold">BUY AND SELL TOKENS</span>
      <span className="text-sm font-medium">
        Your premier destination for buying and selling Solana tokens.
      </span>
      <HoverBorderGradient
        containerClassName="rounded-full"
        as="button"
        className="dark:bg-black px-10 bg-white text-black dark:text-white flex items-center space-x-2 border-black"
      >
        <Link href={"/discover"}>Discover</Link>
      </HoverBorderGradient>
      <div className="overflow-hidden dark:bg-primary bg-white w-full">
        <MacbookScroll src={`/mac.png`} showGradient={false} />
      </div>
      <FeaturesSection />

      <div className="flex flex-col h-screen items-center justify-center w-screen gap-10">
        <span className="gradient text-xxlg font-bold">
          Ready to trade on solana?
        </span>
        <HoverBorderGradient
          containerClassName="rounded-full"
          as="button"
          className="dark:bg-black px-10 bg-white text-black dark:text-white flex items-center space-x-2 border-black"
        >
          <Link href={"/discover"}>Connect your wallet and discover tokens</Link>
        </HoverBorderGradient>
      </div>
      <div className="flex items-start bottom-0 border border-darkGray w-screen text text-lightGray p-8">
        <IconCopyright /> solchan
      </div>
    </div>
  );
}
