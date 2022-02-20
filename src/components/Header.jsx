import React, { useEffect, useState } from "react";
import logo from "assets/images/logo.png";
import { Container, Link, NavLink, Row } from "assets/styles/common.style";
import { convertFromYocto, login, logout } from "near/api";
import { Button } from "./basic/Button";
import SocialLinks from "components/SocialLinks";

export const Header = ({ currentUser }) => {
  const [scroll, setScroll] = useState(false);
  const [userTokenBalance, setUserTokenBalance] = useState(0);

  useEffect(() => {
    // Change header bg on scroll
    window.addEventListener("scroll", () => {
      setScroll(window.scrollY > 60);
    });
  }, []);

  useEffect(() => {
    setUserTokenBalance(currentUser.tokenBalance);
  }, [currentUser]);

  return (
    <div
      className={`sticky top-0 z-30 py-5 transition ease-in-out duration-300 ${
        scroll ? "bg-[#140E38]/95 shadow-md" : ""
      }`}
    >
      <Container>
        <Row className="justify-between">
          <Link to="/" className="flex flex-row hover:text-indigo-50">
            <img src={logo} alt="logo" width="40" className="basis-1/4" />
            <span className="stone-font ml-3 text-4xl font-normal hover:text-indigo-50">
              MineLand
            </span>
          </Link>

          <div className="uppercase ml-20 hidden lg:block">
            {currentUser ? (
              <>
                <NavLink to="/mines">Mines</NavLink>
                <NavLink to="/stones">Stones</NavLink>
                <NavLink to="/collections">Collections</NavLink>
                <NavLink to="/market">Market</NavLink>
                <NavLink to="/faq">FAQ</NavLink>
              </>
            ) : (
              <>
                <NavLink to="/">Home</NavLink>
                <NavLink to="/#about">About</NavLink>
                <NavLink to="/#hot_to_play">How to play</NavLink>
                <NavLink to="/#roadmap">Roadmap</NavLink>
                <NavLink to="/#partners">Partners</NavLink>
                <NavLink to="/#contact_us">Contact</NavLink>
              </>
            )}
          </div>

          {currentUser ? (
            <>
              <div className="flex flex-row">
                <div className="text-right">
                  <div className="mr-10 w-40">
                    <Link to="/token" className="hover:text-indigo-100">
                      <p className="whitespace-nowrap overflow-hidden pt-1 font-medium">
                        {currentUser.accountId}
                      </p>
                      {userTokenBalance != null && (
                        <span className="font-bold text-xl">{convertFromYocto(userTokenBalance, 2)} MNL</span>
                      )}
                    </Link>

                  </div>
                </div>
                <div className="pt-2">
                  <Button secondary title="Log Out" onClick={logout} />
                </div>
              </div>
            </>
          ) : (
            <>
              <SocialLinks />
              <Button secondary title="Log In" onClick={login} />
            </>
          )}
        </Row>
      </Container>
    </div>
  );
};
