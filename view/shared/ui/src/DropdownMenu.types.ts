import * as DropdownMenuPrimitive from "@radix-ui/react-dropdown-menu";

export interface DropdownMenuProps extends DropdownMenuPrimitive.DropdownMenuProps {}

export interface DropdownMenuTriggerProps
  extends Pick<DropdownMenuPrimitive.DropdownMenuTriggerProps, "asChild" | "className" | "children"> {}

type CommonContentProps = Pick<
  DropdownMenuPrimitive.DropdownMenuContentProps,
  | "asChild"
  | "loop"
  | "onCloseAutoFocus"
  | "onEscapeKeyDown"
  | "onPointerDownOutside"
  | "onFocusOutside"
  | "onInteractOutside"
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
  | "className"
  | "children"
>;

export interface DropdownMenuContentProps extends CommonContentProps {
  withArrow?: boolean;
  sideOffset?: number;
}

export interface DropdownMenuSubContentProps extends CommonContentProps {
  withArrow?: boolean;
  sideOffset?: number;
}

export interface DropdownMenuItemProps
  extends Pick<
    DropdownMenuPrimitive.DropdownMenuItemProps,
    "disabled" | "onSelect" | "textValue" | "className" | "children"
  > {}

export interface DropdownMenuCheckboxItemProps
  extends Pick<
    DropdownMenuPrimitive.DropdownMenuCheckboxItemProps,
    "checked" | "onCheckedChange" | "disabled" | "onSelect" | "textValue" | "className" | "children"
  > {
  closeOnSelect?: boolean;
}

export interface DropdownMenuRadioItemProps
  extends Pick<
    DropdownMenuPrimitive.DropdownMenuRadioItemProps,
    "value" | "disabled" | "onSelect" | "className" | "children"
  > {}

export interface DropdownMenuLabelProps
  extends Pick<DropdownMenuPrimitive.DropdownMenuLabelProps, "children" | "className"> {}

export interface DropdownMenuSeparatorProps
  extends Pick<DropdownMenuPrimitive.DropdownMenuSeparatorProps, "className"> {
  fullWidth?: boolean;
}

export interface DropdownMenuSubTriggerProps
  extends Pick<
    DropdownMenuPrimitive.DropdownMenuSubTriggerProps,
    "disabled" | "textValue" | "className" | "children"
  > {}

export interface DropdownMenuRadioGroupProps
  extends Pick<
    DropdownMenuPrimitive.DropdownMenuRadioGroupProps,
    "value" | "onValueChange" | "className" | "children"
  > {}

export interface DropdownMenuArrowProps
  extends Pick<DropdownMenuPrimitive.DropdownMenuArrowProps, "width" | "height" | "className"> {}

export interface DropdownMenuGroupProps
  extends Pick<DropdownMenuPrimitive.DropdownMenuGroupProps, "children" | "className"> {}

export interface DropdownMenuSubProps
  extends Pick<DropdownMenuPrimitive.DropdownMenuSubProps, "defaultOpen" | "open" | "onOpenChange" | "children"> {}

export interface DropdownMenuPortalProps
  extends Pick<DropdownMenuPrimitive.DropdownMenuPortalProps, "children" | "forceMount" | "container"> {}
