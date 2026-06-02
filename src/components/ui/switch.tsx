import * as React from "react";

import { cn } from "@/lib/utils";

export interface SwitchProps
  extends Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, "onChange" | "value"> {
  checked: boolean;
  onCheckedChange?: (checked: boolean) => void;
}

export const Switch = React.forwardRef<HTMLButtonElement, SwitchProps>(
  ({ checked, className, disabled, onCheckedChange, ...props }, ref) => (
    <button
      aria-checked={checked}
      className={cn(
        "relative inline-flex h-5 w-9 shrink-0 items-center rounded-full border transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50",
        checked ? "border-primary bg-primary" : "border-border bg-secondary",
        className,
      )}
      disabled={disabled}
      onClick={(event) => {
        props.onClick?.(event);
        if (!event.defaultPrevented) {
          onCheckedChange?.(!checked);
        }
      }}
      ref={ref}
      role="switch"
      type="button"
      {...props}
    >
      <span
        className={cn(
          "block h-4 w-4 rounded-full bg-white shadow-sm transition-transform",
          checked ? "translate-x-4" : "translate-x-0.5",
        )}
      />
    </button>
  ),
);
Switch.displayName = "Switch";
