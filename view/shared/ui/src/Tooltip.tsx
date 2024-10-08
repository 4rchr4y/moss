import * as React from "react";
import * as TooltipPrimitive from "@radix-ui/react-tooltip";
import { cn } from "./utils/utils";

// api-reference https://www.radix-ui.com/primitives/docs/components/tooltip#api-reference

export interface TooltipOptions {
  provider?: Pick<
    TooltipPrimitive.TooltipProviderProps,
    "delayDuration" | "skipDelayDuration" | "disableHoverableContent"
  >;
  root?: Pick<
    TooltipPrimitive.TooltipProps,
    "defaultOpen" | "open" | "onOpenChange" | "delayDuration" | "disableHoverableContent"
  >;
  content?: Pick<
    TooltipPrimitive.TooltipContentProps,
    | "aria-label"
    | "onEscapeKeyDown"
    | "onPointerDownOutside"
    | "forceMount"
    | "side"
    | "sideOffset"
    | "align"
    | "alignOffset"
    | "avoidCollisions"
    | "collisionBoundary"
    | "collisionPadding"
    | "arrowPadding"
    | "sticky"
    | "hideWhenDetached"
  >;
  arrow?: Pick<TooltipPrimitive.TooltipArrowProps, "asChild" | "width" | "height">;
  portal?: Pick<TooltipPrimitive.TooltipPortalProps, "forceMount" | "container">;
}

export const Tooltip = ({
  label,
  shortcut, //TODO shortcut doesn't have any functionality
  options,
  noArrow = false,
  asChild = false,
  children,
  className,
}: {
  label?: string;
  shortcut?: string[];
  options?: TooltipOptions;
  noArrow?: boolean;
  asChild?: boolean;
  className?: string;
  children?: React.ReactNode;
}) => {
  return (
    <TooltipPrimitive.Provider {...options?.provider}>
      <TooltipPrimitive.Root {...options?.root}>
        <TooltipPrimitive.Trigger asChild={asChild}>{children}</TooltipPrimitive.Trigger>
        <TooltipPrimitive.Content
          className={cn(
            `data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 -mb-px flex max-w-44 gap-2.5 overflow-hidden
              rounded-md
              bg-[#1E1E1E]
              px-2
              py-1
              text-xs
              text-white
              shadow-md`,
            className
          )}
          {...options?.content}
        >
          {options?.portal && <TooltipPrimitive.Portal {...options?.portal} />}
          {noArrow === false && <TooltipPrimitive.Arrow className="bg-inherit " {...options?.arrow} />}

          <div>{label}</div>

          {shortcut && (
            <div className="text-neutral-400 self-center uppercase">
              {shortcut.map((s) => (
                <span key={s}>{s}</span>
              ))}
            </div>
          )}
        </TooltipPrimitive.Content>
      </TooltipPrimitive.Root>
    </TooltipPrimitive.Provider>
  );
};

export default Tooltip;
