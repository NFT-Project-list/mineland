import React from "react";
import logo from "assets/images/logo.png";
import { Col, Container, Link, Row } from "assets/styles/common.style";
import SocialLinks from "components/SocialLinks";

export const Footer = () => (
  <div className="h-36 bg-[#140E38]/80 pt-3 shadow-lg">
    <Container>
      <Row className="justify-between pt-4">
        <div className="w-1/4">
          <Row>
            <Col>
              <img src={logo} alt="logo" width="55" className="opacity-70" />
            </Col>
            <Col className="ml-3">
              <span className="stone-font text-2xl font-normal hover:text-indigo-50 opacity-70">
                MineLand
              </span>
              <div className=" uppercase text-sm pt-1">
                <span className="opacity-70">© MADE BY </span>
                <a
                  href="https://someteam.eu"
                  target="_blank"
                  className="font-semibold"
                  rel="noreferrer"
                >
                  SomeTeam.eu
                </a>
              </div>
            </Col>
          </Row>
        </div>
        <div className="w-1/2 pl-12">
          <Row>
            <Col className="w-1/3">
              <Link size="sm" className="block leading-7 uppercase" to="/mines">
                Mines
              </Link>
              <Link
                size="sm"
                className="block leading-7 uppercase"
                to="/stones"
              >
                Stones
              </Link>
              <Link
                size="sm"
                className="block leading-7 uppercase"
                to="/collections"
              >
                Collections
              </Link>
            </Col>
            <Col className="w-1/3">
              <Link
                size="sm"
                className="block leading-7 uppercase"
                to="/market"
              >
                Market
              </Link>
              <Link size="sm" className="block leading-7 uppercase" to="/faq">
                FAQ
              </Link>
            </Col>
            <Col className="w-1/3">
              <Link
                size="sm"
                className="block leading-7 uppercase"
                to="/contact-us"
              >
                Contact US
              </Link>
              <Link
                size="sm"
                className="block leading-7 uppercase"
                to="/terms-conditions"
              >
                Terms & Conditions
              </Link>
              <Link
                size="sm"
                className="block leading-7 uppercase"
                to="/privacy-policy"
              >
                Privacy Policy
              </Link>
            </Col>
          </Row>
        </div>
        <div className="w-1/4 text-right">
          <SocialLinks />
        </div>
      </Row>
    </Container>
  </div>
);
