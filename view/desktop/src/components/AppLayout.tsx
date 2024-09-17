import { ComponentProps, forwardRef } from "react";
import { twMerge } from "tailwind-merge";
import { TitleBar } from "./TitleBar";
import StatusBar from "./StatusBar";

export const RootLayout = ({ children, className, ...props }: ComponentProps<"main">) => {
  return (
    <div className="h-full grid grid-rows-[auto_1fr_auto]">
      <TitleBar />

      <main className={twMerge("bg-pageBackground", className)} {...props}>
        {children}
      </main>

      <StatusBar className="w-full bottom-0 h-5.5" branch="MOSSMVP-37-Backend-Migrate-existing-backend-in-Tauri" />
    </div>
  );
};

export const Sidebar = ({ className, children, ...props }: ComponentProps<"aside">) => {
  return (
    <aside
      className={twMerge("flex flex-col mb-5.5 bg-sideBarBackground", className)}
      //className={twMerge('w-[200px] mt-8 h-[100vh + 10px] overflow-auto bg-bgPrimary', className)}
      {...props}
    >
      {children}
    </aside>
  );
};

export const Content = forwardRef<HTMLDivElement, ComponentProps<"div">>(({ children, className, ...props }, ref) => (
  <div ref={ref} className={twMerge("mb-5.5 flex-1 overflow-auto bg-bgPrimary", className)} {...props}>
    {children}
  </div>
));

export const Properties = ({ className, children, ...props }: ComponentProps<"aside">) => {
  return (
    <aside className={twMerge("w-[50px] mt-8 h-[100vh + 10px] overflow-auto bg-bgPrimary", className)} {...props}>
      {children}
    </aside>
  );
};

export const DockviewPanelLayout = ({ children }: { children: React.ReactNode }) => {
  return <div className="h-full overflow-auto">{children}</div>;
};
