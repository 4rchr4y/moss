import type { OsType } from "@tauri-apps/plugin-os";
import { useEffect, useState } from "react";
import { cn } from "@/components/window-controls/libs/utils";
import { getOsType } from "@/components/window-controls/libs/plugin-os";
import type { WindowTitlebarProps } from "@/components/window-controls/types";
import { WindowControls } from "@/components/window-controls/WindowControls";

export function WindowTitlebar({
  children,
  controlsOrder = "system",
  className,
  windowControlsProps,
  ...props
}: WindowTitlebarProps) {
  const [osType, setOsType] = useState<OsType | undefined>(undefined);

  // for macOS testing: setOsType("macos");
  useEffect(() => {
    getOsType().then((type) => {
      setOsType(type);
    });
  }, []);

  const left =
    controlsOrder === "left" ||
    (controlsOrder === "platform" && windowControlsProps?.platform === "macos") ||
    (controlsOrder === "system" && osType === "macos");

  const customProps = (style: string) => {
    if (windowControlsProps?.justify !== undefined) return windowControlsProps;

    const {
      justify: windowControlsJustify,
      className: windowControlsClassName,
      ...restProps
    } = windowControlsProps || {};
    return {
      justify: false,
      className: cn(windowControlsClassName, style),
      ...restProps,
    };
  };

  return (
    <div
      className={cn("bg-background flex select-none flex-row overflow-hidden", className)}
      data-tauri-drag-region
      {...props}
    >
      {left ? (
        <>
          <WindowControls {...customProps("ml-2 mt-3 z-50")} />
          {children}
        </>
      ) : (
        <>
          {children}
          <WindowControls {...customProps("ml-auto")} />
        </>
      )}
    </div>
  );
}
