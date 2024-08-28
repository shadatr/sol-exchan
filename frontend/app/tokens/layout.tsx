
import "@solana/wallet-adapter-react-ui/styles.css";

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <>{children}</>
    
  );
}
