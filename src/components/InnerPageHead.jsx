import React from "react";

export const InnerPageHead = ({ title, description }) => (
  <>
    <h1 className="stone-font font-normal text-6xl title-shadow">
      {title}
    </h1>
    <p className="w-1/2 mx-auto my-6">
      {description}
    </p>
  </>
);
