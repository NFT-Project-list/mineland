import React from "react";
import { Btn, Row } from "assets/styles/common.style";
import { ArrowRightIcon } from "@heroicons/react/solid";

export const Button = ({
  title,
  onClick,
  size,
  icon,
  noIcon,
  disabled,
  animated,
  secondary,
  dark,
}) => {
  const sizeMapping = {
    sm: "text-sm px-4 py-2",
    md: "text-base px-5 py-1.5",
    lg: "text-lg px-6 py-1.5",
  };

  const iconMapping = {
    sm: "h-4",
    md: "h-5",
    lg: "h-6",
  };

  return (
    <>
      <Btn
        animated={animated}
        className={`border-4 rounded-lg font-semibold uppercase text-white group transition ease-in-out duration-200 ${
          disabled && "grayscale cursor-default"
        } ${
          secondary
            ? "border-orange-500 hover:text-orange-500 hover:border-orange-600"
            : dark
            ? "border-purple-600 text-purple-500 hover:text-purple-300 hover:border-purple-400"
            : "border-transparent bg-orange-600 hover:bg-orange-700"
        }`}
        onClick={onClick}
      >
        <Row
          className={`justify-center whitespace-nowrap ${
            sizeMapping[size ?? "md"]
          } `}
        >
          {title}
          {noIcon ?? icon ?? (
            <ArrowRightIcon className={`ml-2 ${iconMapping[size ?? "md"]}`} />
          )}
        </Row>
      </Btn>
    </>
  );
};
