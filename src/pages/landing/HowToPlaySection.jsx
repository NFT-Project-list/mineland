import React from "react";
import { Col, Row } from "assets/styles/common.style";

export const HowToPlaySection = ({ reverse, title, desc, img }) => (
  <Row className={`justify-around my-10 flex-row}`}>
    {reverse ? (
      <>
        <Col className="w-1/2 text-right items-left">
          <h2 className="text-3xl leading-tight font-semibold">{title}</h2>
          <h3 className="my-5 text-lg leading-normal">{desc}</h3>
        </Col>
        <Col className="w-1/2">
          <img src={img} alt={title} width="80%" className="ml-12" />
        </Col>
      </>
    ) : (
      <>
        <Col className="w-1/2">
          <img src={img} alt={title} width="80%" />
        </Col>
        <Col className="w-1/2 text-left items-left">
          <h2 className="text-3xl leading-tight font-semibold">{title}</h2>
          <h3 className="my-5 text-lg leading-normal">{desc}</h3>
        </Col>
      </>
    )}
  </Row>
);
