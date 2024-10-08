import { twMerge } from "tailwind-merge";
import { WindowTitlebar } from "@/components";
import { getOsType } from "@/components/window-controls/libs/plugin-os";
import { OsType } from "@tauri-apps/plugin-os";
import { useEffect, useState } from "react";
import { cn } from "@/utils";

interface TitleBarProps {
  showControls?: string;
  isMacOs?: boolean;
}

export const TitleBar = ({}: TitleBarProps) => {
  const [osType, setOsType] = useState<OsType | undefined>(undefined);

  useEffect(() => {
    getOsType().then((type) => {
      setOsType(type);
    });
  });

  return (
    <header
      data-tauri-drag-region
      className={cn("inset-0 bg-[rgba(var(--color-tool-bar-background))]", {
        "rounded-t-lg": osType != "windows",
      })}
    >
      <WindowTitlebar />
    </header>
  );
};
