import React, { useState } from "react";
import { Col, Container, InnerPageWrapper, Row, Wrapper } from 'assets/styles/common.style';
import { Header } from "components/Header";
import { Footer } from 'components/Footer';
import { InnerPageHead } from 'components/InnerPageHead';
import { TokenContent } from 'utils/content';
import { convertFromYocto, login } from '../near/api';
import { Button } from '../components/basic/Button';

export const Token = ({ currentUser }) => {

  return (
    <>
      <InnerPageWrapper>
        <Header currentUser={currentUser} />

        <Wrapper>
          <Container className="text-white text-center mt-8">
            <InnerPageHead title={TokenContent.title} description={TokenContent.description} />

            <div className="bg-main mx-auto p-10 mb-10 rounded-xl mt-10 flex">
              <div className="w-3/4">
                <p>
                  Your Balance: {" "}
                  <span className="text-xl font-semibold">{convertFromYocto(currentUser.tokenBalance, 2)} ZML</span>
                  <span className="ml-4 mr-4 text-gray-500 text-xl align-middle">|</span>
                  <span>
                    Staked ZML: <span className="text-xl font-semibold">0 ZML</span>
                  </span>
                </p>
                <div className="mt-8">
                  <input type="number" min="1"
                         className="px-3 py-2.5 rounded-md mr-2 bg-transparent border-indigo-500 text-indigo-100 border-2"
                         placeholder="Amount ZML" />
                  <Button secondary title="Deposit" />
                </div>
              </div>

              <div className="w-1/4 border-l-2 border-gray-800 border-dashed">
                <div className="w-64 h-80 bg-[#0d376f] ml-4 px-10 pt-24">
                  <p className="mb-4 font-semibold">Select Monster to increase your reward:</p>
                  <p>Common: +2%</p>
                  <p>Common: +4%</p>
                  <p>Common: +12%</p>
                  <p>Common: +25%</p>
                </div>
              </div>
            </div>


          </Container>
        </Wrapper>

        <Footer />
      </InnerPageWrapper>
    </>
  )
};
