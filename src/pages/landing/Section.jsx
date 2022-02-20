import React from "react";
import { Col } from "assets/styles/common.style";

export const Section = ({ title, description, children }) => {
  return (
    <Col className="items-center text-center">
      <h1 className="stone-font text-5xl leading-tight title-shadow">{title}</h1>
      <h3 className="my-10  text-lg leading-normal w-3/4">{description}</h3>
      {children}
    </Col>
  );
};
