import { twMerge } from "tailwind-merge";
import { ComponentPropsWithoutRef } from "react";

export const IconTitle = ({ title, className, ...props }: { title?: string } & ComponentPropsWithoutRef<"div">) => {
  return (
    <div className={twMerge("opacity-80 group-hover:opacity-100 group-focus:opacity-100 ", className)} {...props}>
      <div className=" whitespace-nowrap">{title}</div>
    </div>
  );
};

export default IconTitle;
