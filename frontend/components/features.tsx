import { cn } from "@/lib/utils";
import {
  IconAdjustmentsBolt,
  IconCloud,
  IconCurrencyDollar,
  IconEaseInOut,
  IconHeart,
  IconHelp,
  IconRouteAltLeft,
  IconTerminal2,
  IconCopy
} from "@tabler/icons-react";

export function FeaturesSection() {
  const features = [
    {
      title: "Seamless Integration",
      description:
        "Plug and play with your existing systems effortlessly, making token creation and trading simple.",
      icon: <IconTerminal2 />,
    },
    {
      title: "User-Friendly Interface",
      description:
        "As intuitive as your favorite app, making token management accessible to everyone.",
      icon: <IconEaseInOut />,
    },
    {
      title: "Competitive Pricing",
      description:
        "Unbeatable prices with no hidden fees, offering the best value for your investment.",
      icon: <IconCurrencyDollar />,
    },
    {
      title: "Rock-Solid Reliability ",
      description: "Guaranteed 100% uptime, ensuring your tokens are always available and secure.",
      icon: <IconCloud />,
    },
    {
      title: "Scalable Solutions ",
      description: "Designed to grow with you, supporting both individual creators and large enterprises.",
      icon: <IconRouteAltLeft />,
    },
    {
      title: "Round-the-Clock Support ",
      description:
        "24/7 customer service with real-time assistance from our AI and human agents.",
      icon: <IconHelp />,
    },
    {
      title: "Satisfaction Guaranteed ",
      description:
        "Love our platform or get your money back, no questions asked.",
      icon: <IconAdjustmentsBolt />,
    },
    {
      title: "All-in-One Platform",
      description: "Everything you need for token creation, management, and trading in one place.",
      icon: <IconHeart />,
    },
  ];
  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4  relative z-10 py-10 max-w-7xl mx-auto">
      {features.map((feature, index) => (
        <Feature key={feature.title} {...feature} index={index} />
      ))}
    </div>
  );
}

const Feature = ({
  title,
  description,
  icon,
  index,
}: {
  title: string;
  description: string;
  icon: React.ReactNode;
  index: number;
}) => {
  return (
    <div
      className={cn(
        "flex flex-col lg:border-r  py-10 relative group/feature dark:border-neutral-800",
        (index === 0 || index === 4) && "lg:border-l dark:border-neutral-800",
        index < 4 && "lg:border-b dark:border-neutral-800"
      )}
    >
      {index < 4 && (
        <div className="opacity-0 group-hover/feature:opacity-100 transition duration-200 absolute inset-0 h-full w-full bg-gradient-to-t from-neutral-100 dark:from-neutral-800 to-transparent pointer-events-none" />
      )}
      {index >= 4 && (
        <div className="opacity-0 group-hover/feature:opacity-100 transition duration-200 absolute inset-0 h-full w-full bg-gradient-to-b from-neutral-100 dark:from-neutral-800 to-transparent pointer-events-none" />
      )}
      <div className="mb-4 relative z-10 px-10 text-neutral-600 dark:text-neutral-400">
        {icon}
      </div>
      <div className="text-lg font-bold mb-2 relative z-10 px-10">
        <div className="absolute left-0 inset-y-0 h-6 group-hover/feature:h-8 w-1 rounded-tr-full rounded-br-full bg-neutral-300 dark:bg-neutral-700 group-hover/feature:bg-blue-500 transition-all duration-200 origin-center" />
        <span className="group-hover/feature:translate-x-2 transition duration-200 inline-block text-neutral-800 dark:text-neutral-100">
          {title}
        </span>
      </div>
      <p className="text-sm text-neutral-600 dark:text-neutral-300 max-w-xs relative z-10 px-10">
        {description}
      </p>
    </div>
  );
};
